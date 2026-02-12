// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::*;
use std::collections::HashSet;

use super::Molder;

impl Molder {
    // Phase 4: Rule Verification
    pub(super) fn phase4_verify_rules(&self) -> Result<(), String> {
        let mut global_vars = HashSet::new();

        for item in &self.source.items {
            if let Item::Function(func) = item {
                let func_unsafe = matches!(func.safety, SafetyLevel::Raw);
                self.verify_function_signature(func, &mut global_vars, func_unsafe)?;
                self.verify_rules_in_block(&func.body, &mut global_vars, func_unsafe)?;
            }
        }

        Ok(())
    }

    fn verify_function_signature(
        &self,
        func: &Function,
        global_vars: &mut HashSet<String>,
        in_unsafe: bool,
    ) -> Result<(), String> {
        for arg in &func.args {
            if !global_vars.insert(arg.name.clone()) {
                return Err(format!(
                    "Rule 4 Violation: Variable '{}' already defined.",
                    arg.name
                ));
            }
            self.verify_var_prefix(&arg.name, in_unsafe)?;
            self.verify_type_safety(&arg.ty, in_unsafe)?;
        }

        if let Some(ret) = &func.ret_type {
            self.verify_type_safety(ret, in_unsafe)?;
        }
        Ok(())
    }

    fn verify_rules_in_block(
        &self,
        block: &Block,
        global_vars: &mut HashSet<String>,
        in_unsafe: bool,
    ) -> Result<(), String> {
        let current_unsafe = in_unsafe || block.unsafe_block;

        for stmt in &block.statements {
            self.verify_rules_in_stmt(stmt, global_vars, current_unsafe)?;
        }
        Ok(())
    }

    fn verify_rules_in_stmt(
        &self,
        stmt: &Statement,
        global_vars: &mut HashSet<String>,
        in_unsafe: bool,
    ) -> Result<(), String> {
        match stmt {
            Statement::Let(l) => {
                if !global_vars.insert(l.name.clone()) {
                    return Err(format!(
                        "Rule 4 Violation: Variable '{}' already defined.",
                        l.name
                    ));
                }
                self.verify_var_prefix(&l.name, in_unsafe)?;
                if let Some(ty) = &l.ty {
                    self.verify_type_safety(ty, in_unsafe)?;
                }

                self.verify_rules_in_expr(&l.value, global_vars, in_unsafe)?;
                self.verify_raw_to_high_rule(&l.name, &l.value, in_unsafe)?;
            }
            Statement::Const(c) => {
                if !global_vars.insert(c.name.clone()) {
                    return Err(format!(
                        "Rule 4 Violation: Variable '{}' already defined.",
                        c.name
                    ));
                }
                self.verify_var_prefix(&c.name, in_unsafe)?;
                if let Some(ty) = &c.ty {
                    self.verify_type_safety(ty, in_unsafe)?;
                }

                self.verify_rules_in_expr(&c.value, global_vars, in_unsafe)?;
                self.verify_raw_to_high_rule(&c.name, &c.value, in_unsafe)?;
            }
            Statement::If(stmt) => {
                self.verify_rules_in_expr(&stmt.condition, global_vars, in_unsafe)?;
                self.verify_rules_in_block(&stmt.then_block, global_vars, in_unsafe)?;
                if let Some(else_block) = &stmt.else_block {
                    self.verify_rules_in_block(else_block, global_vars, in_unsafe)?;
                }
            }
            Statement::For(stmt) => {
                if !global_vars.insert(stmt.var_name.clone()) {
                    return Err(format!(
                        "Rule 4 Violation: Variable '{}' already defined.",
                        stmt.var_name
                    ));
                }
                self.verify_var_prefix(&stmt.var_name, in_unsafe)?;
                self.verify_rules_in_expr(&stmt.start, global_vars, in_unsafe)?;
                self.verify_rules_in_expr(&stmt.end, global_vars, in_unsafe)?;
                self.verify_rules_in_block(&stmt.body, global_vars, in_unsafe)?;
            }
            Statement::Break | Statement::Continue => {}
            Statement::Expr(e) => {
                self.verify_rules_in_expr(e, global_vars, in_unsafe)?;
            }
        }
        Ok(())
    }

    fn verify_rules_in_expr(
        &self,
        expr: &Expression,
        global_vars: &mut HashSet<String>,
        in_unsafe: bool,
    ) -> Result<(), String> {
        match expr {
            Expression::Block(b) => {
                self.verify_rules_in_block(b, global_vars, in_unsafe)?;
            }
            Expression::Call(c) => {
                for arg in &c.args {
                    self.verify_rules_in_expr(arg, global_vars, in_unsafe)?;
                }
            }
            Expression::Binary { left, right, .. } => {
                self.verify_rules_in_expr(left, global_vars, in_unsafe)?;
                self.verify_rules_in_expr(right, global_vars, in_unsafe)?;
            }
            Expression::Ref { expr, .. } => {
                self.verify_rules_in_expr(expr, global_vars, in_unsafe)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn verify_var_prefix(&self, name: &str, in_unsafe: bool) -> Result<(), String> {
        if in_unsafe {
            if !(name.starts_with("raw_")
                || name.starts_with("validated_")
                || name.starts_with("high_"))
            {
                return Err(format!(
                    "Rule 5 Violation: Variable '{name}' in unsafe block must start with 'raw_', 'validated_', or 'high_'."
                ));
            }
        } else if !name.starts_with("high_") {
            return Err(format!(
                "Rule 5 Violation: Variable '{name}' outside unsafe must start with 'high_'."
            ));
        }
        Ok(())
    }

    fn verify_type_safety(&self, ty: &Type, in_unsafe: bool) -> Result<(), String> {
        if !in_unsafe && Self::is_unsafe_type(ty) {
            return Err(format!(
                "Rule 3 Violation: Unsafe type '{}' used outside unsafe block.",
                Self::type_display(ty)
            ));
        }
        Ok(())
    }

    fn is_unsafe_type(ty: &Type) -> bool {
        match ty {
            Type::RawPtr(_) => true,
            Type::Ref { inner, .. } => Self::is_unsafe_type(inner),
            Type::Path(name) => {
                name.contains("::raw::")
                    || name.contains("Raw<")
                    || name.ends_with("RawPtr")
                    || name.contains("Validated<")
                    || name.ends_with("ValidatedPtr")
            }
        }
    }

    fn type_display(ty: &Type) -> String {
        match ty {
            Type::Path(s) => s.clone(),
            Type::RawPtr(inner) => format!("*{}", Self::type_display(inner)),
            Type::Ref { mutable, inner } => {
                if *mutable {
                    format!("&mut {}", Self::type_display(inner))
                } else {
                    format!("&{}", Self::type_display(inner))
                }
            }
        }
    }

    fn verify_raw_to_high_rule(
        &self,
        name: &str,
        value: &Expression,
        in_unsafe: bool,
    ) -> Result<(), String> {
        if !in_unsafe {
            return Ok(());
        }

        if name.starts_with("validated_") {
            match value {
                Expression::Call(call) => {
                    if call.func_name != "validate_raw"
                        && call.func_name != "core::memory::safe::validate_raw"
                    {
                        return Err(format!(
                            "Rule 6 Violation: Validated variable '{name}' must be created via validate_raw()."
                        ));
                    }
                    if let Some(Expression::Variable(var)) = call.args.first() {
                        if !var.starts_with("raw_") {
                            return Err(format!(
                                "Rule 6 Violation: validate_raw() must use a raw_ value (got '{var}')."
                            ));
                        }
                    } else {
                        return Err(format!(
                            "Rule 6 Violation: validate_raw() must take a raw_ variable for '{name}'."
                        ));
                    }
                }
                _ => {
                    return Err(format!(
                        "Rule 6 Violation: Validated variable '{name}' must be created via validate_raw()."
                    ));
                }
            }
        }

        if name.starts_with("high_") {
            match value {
                Expression::Call(call) => {
                    if call.func_name != "into_high"
                        && call.func_name != "core::memory::safe::into_high"
                    {
                        return Err(format!(
                            "Rule 6 Violation: High variable '{name}' in unsafe must be created via into_high()."
                        ));
                    }
                    if let Some(Expression::Variable(var)) = call.args.first() {
                        if !var.starts_with("validated_") {
                            return Err(format!(
                                "Rule 6 Violation: into_high() must use a validated_ value (got '{var}')."
                            ));
                        }
                    } else {
                        return Err(format!(
                            "Rule 6 Violation: into_high() must take a validated_ variable for '{name}'."
                        ));
                    }
                }
                _ => {
                    return Err(format!(
                        "Rule 6 Violation: High variable '{name}' in unsafe must be created via into_high()."
                    ));
                }
            }
        }

        Ok(())
    }
}
