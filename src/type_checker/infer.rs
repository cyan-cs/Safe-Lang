// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::*;
use crate::std_api;
use std::collections::HashMap;

use super::TypeChecker;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LiteralKind {
    Integer,
    String,
}

impl TypeChecker {
    pub(super) fn check_block(
        &self,
        block: &Block,
        symbols: &mut HashMap<String, Type>,
        loop_depth: usize,
    ) -> Result<(), String> {
        let mut scope = symbols.clone();

        for stmt in &block.statements {
            match stmt {
                Statement::Let(l) => {
                    let rhs_type =
                        self.check_binding(&l.name, &l.ty, &l.value, &scope, loop_depth)?;
                    scope.insert(l.name.clone(), rhs_type);
                }
                Statement::Const(c) => {
                    let rhs_type =
                        self.check_binding(&c.name, &c.ty, &c.value, &scope, loop_depth)?;
                    scope.insert(c.name.clone(), rhs_type);
                }
                Statement::If(stmt) => {
                    let cond_ty = self.infer_expr_with_loop(&stmt.condition, &scope, loop_depth)?;
                    if !Self::types_equal(&cond_ty, &Type::Path("bool".to_string())) {
                        return Err(format!(
                            "If condition must be bool, got {}",
                            Self::type_display(&cond_ty)
                        ));
                    }
                    let mut then_scope = scope.clone();
                    self.check_block(&stmt.then_block, &mut then_scope, loop_depth)?;
                    if let Some(else_block) = &stmt.else_block {
                        let mut else_scope = scope.clone();
                        self.check_block(else_block, &mut else_scope, loop_depth)?;
                    }
                }
                Statement::For(stmt) => {
                    let start_ty = self.infer_expr_with_loop(&stmt.start, &scope, loop_depth)?;
                    let end_ty = self.infer_expr_with_loop(&stmt.end, &scope, loop_depth)?;
                    let loop_var_ty =
                        Self::infer_for_loop_var_type(&stmt.start, &start_ty, &stmt.end, &end_ty)?;
                    let mut loop_scope = scope.clone();
                    loop_scope.insert(stmt.var_name.clone(), loop_var_ty);
                    self.check_block(&stmt.body, &mut loop_scope, loop_depth + 1)?;
                }
                Statement::Break | Statement::Continue => {
                    if loop_depth == 0 {
                        return Err("break/continue can only be used inside for-loops".to_string());
                    }
                }
                Statement::Expr(e) => {
                    self.infer_expr_with_loop(e, &scope, loop_depth)?;
                }
            }
        }
        Ok(())
    }

    fn infer_expr_with_loop(
        &self,
        expr: &Expression,
        scope: &HashMap<String, Type>,
        loop_depth: usize,
    ) -> Result<Type, String> {
        match expr {
            Expression::Literal(lit) => match lit {
                Literal::Integer(_) => Ok(Type::Path("i32".to_string())),
                Literal::String(_) => Ok(Type::Path("String".to_string())),
                Literal::Bool(_) => Ok(Type::Path("bool".to_string())),
            },
            Expression::Variable(name) => scope
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: '{name}'")),
            Expression::Binary { op, left, right } => {
                let left_ty = self.infer_expr_with_loop(left, scope, loop_depth)?;
                let right_ty = self.infer_expr_with_loop(right, scope, loop_depth)?;
                match op {
                    BinaryOp::Equal | BinaryOp::NotEqual => {
                        if Self::types_equal(&left_ty, &right_ty)
                            || (Self::is_compatible_int_target(&left_ty)
                                && Self::is_compatible_int_target(&right_ty))
                        {
                            Ok(Type::Path("bool".to_string()))
                        } else {
                            Err(format!(
                                "Comparison type mismatch: {} vs {}",
                                Self::type_display(&left_ty),
                                Self::type_display(&right_ty)
                            ))
                        }
                    }
                    BinaryOp::LessThan
                    | BinaryOp::LessEqual
                    | BinaryOp::GreaterThan
                    | BinaryOp::GreaterEqual => {
                        if Self::is_compatible_int_target(&left_ty)
                            && Self::is_compatible_int_target(&right_ty)
                        {
                            Ok(Type::Path("bool".to_string()))
                        } else {
                            Err(format!(
                                "Ordered comparison requires integer operands: {} and {}",
                                Self::type_display(&left_ty),
                                Self::type_display(&right_ty)
                            ))
                        }
                    }
                }
            }
            Expression::Ref { mutable, expr } => {
                let inner_ty = self.infer_expr_with_loop(expr, scope, loop_depth)?;
                Ok(Type::Ref {
                    mutable: *mutable,
                    inner: Box::new(inner_ty),
                })
            }
            Expression::Call(call) => {
                if std_api::is_print_function(&call.func_name)
                    || std_api::is_printl_function(&call.func_name)
                {
                    for arg in &call.args {
                        let inferred = self.infer_expr_with_loop(arg, scope, loop_depth)?;
                        if !Self::is_printable_type(&inferred) {
                            return Err(format!(
                                "print/printl does not support type {}",
                                Self::type_display(&inferred)
                            ));
                        }
                    }
                    return Ok(Type::Path("()".to_string()));
                }

                let (arg_types, ret_type) = self
                    .functions
                    .get(&call.func_name)
                    .ok_or_else(|| format!("Undefined function: '{}'", call.func_name))?;

                if call.args.len() != arg_types.len() {
                    return Err(format!(
                        "Arg count mismatch for '{}': expected {}, got {}",
                        call.func_name,
                        arg_types.len(),
                        call.args.len()
                    ));
                }

                for (i, arg_expr) in call.args.iter().enumerate() {
                    let inferred = self.infer_expr_with_loop(arg_expr, scope, loop_depth)?;

                    if let Some(LiteralKind::Integer) = Self::literal_kind(arg_expr) {
                        if Self::is_compatible_int_target(&arg_types[i]) {
                            continue;
                        }
                    }

                    if !Self::types_equal(&inferred, &arg_types[i]) {
                        return Err(format!(
                            "Type Mismatch in arg {} of '{}': expected {}, got {}",
                            i + 1,
                            call.func_name,
                            Self::type_display(&arg_types[i]),
                            Self::type_display(&inferred)
                        ));
                    }
                }

                Ok(ret_type
                    .clone()
                    .unwrap_or_else(|| Type::Path("()".to_string())))
            }
            Expression::Block(b) => self.infer_block_expr_with_loop(b, scope, loop_depth),
        }
    }

    pub(super) fn infer_block_expr(
        &self,
        block: &Block,
        scope: &HashMap<String, Type>,
    ) -> Result<Type, String> {
        self.infer_block_expr_with_loop(block, scope, 0)
    }

    fn infer_block_expr_with_loop(
        &self,
        block: &Block,
        scope: &HashMap<String, Type>,
        loop_depth: usize,
    ) -> Result<Type, String> {
        let mut block_scope = scope.clone();

        for stmt in &block.statements[..block.statements.len().saturating_sub(1)] {
            match stmt {
                Statement::Let(l) => {
                    let rhs_type =
                        self.check_binding(&l.name, &l.ty, &l.value, &block_scope, loop_depth)?;
                    block_scope.insert(l.name.clone(), rhs_type);
                }
                Statement::Const(c) => {
                    let rhs_type =
                        self.check_binding(&c.name, &c.ty, &c.value, &block_scope, loop_depth)?;
                    block_scope.insert(c.name.clone(), rhs_type);
                }
                Statement::If(stmt) => {
                    let cond_ty =
                        self.infer_expr_with_loop(&stmt.condition, &block_scope, loop_depth)?;
                    if !Self::types_equal(&cond_ty, &Type::Path("bool".to_string())) {
                        return Err(format!(
                            "If condition must be bool, got {}",
                            Self::type_display(&cond_ty)
                        ));
                    }
                    let _ = self.infer_block_expr_with_loop(
                        &stmt.then_block,
                        &block_scope,
                        loop_depth,
                    )?;
                    if let Some(else_block) = &stmt.else_block {
                        let _ =
                            self.infer_block_expr_with_loop(else_block, &block_scope, loop_depth)?;
                    }
                }
                Statement::For(stmt) => {
                    let start_ty =
                        self.infer_expr_with_loop(&stmt.start, &block_scope, loop_depth)?;
                    let end_ty = self.infer_expr_with_loop(&stmt.end, &block_scope, loop_depth)?;
                    let loop_var_ty =
                        Self::infer_for_loop_var_type(&stmt.start, &start_ty, &stmt.end, &end_ty)?;
                    let mut loop_scope = block_scope.clone();
                    loop_scope.insert(stmt.var_name.clone(), loop_var_ty);
                    self.check_block(&stmt.body, &mut loop_scope, loop_depth + 1)?;
                }
                Statement::Break | Statement::Continue => {
                    if loop_depth == 0 {
                        return Err("break/continue can only be used inside for-loops".to_string());
                    }
                }
                Statement::Expr(e) => {
                    self.infer_expr_with_loop(e, &block_scope, loop_depth)?;
                }
            }
        }

        if let Some(last_stmt) = block.statements.last() {
            match last_stmt {
                Statement::Let(l) => {
                    let _ =
                        self.check_binding(&l.name, &l.ty, &l.value, &block_scope, loop_depth)?;
                    Ok(Type::Path("()".to_string()))
                }
                Statement::Const(c) => {
                    let _ =
                        self.check_binding(&c.name, &c.ty, &c.value, &block_scope, loop_depth)?;
                    Ok(Type::Path("()".to_string()))
                }
                Statement::If(stmt) => {
                    let cond_ty =
                        self.infer_expr_with_loop(&stmt.condition, &block_scope, loop_depth)?;
                    if !Self::types_equal(&cond_ty, &Type::Path("bool".to_string())) {
                        return Err(format!(
                            "If condition must be bool, got {}",
                            Self::type_display(&cond_ty)
                        ));
                    }
                    let _ = self.infer_block_expr_with_loop(
                        &stmt.then_block,
                        &block_scope,
                        loop_depth,
                    )?;
                    if let Some(else_block) = &stmt.else_block {
                        let _ =
                            self.infer_block_expr_with_loop(else_block, &block_scope, loop_depth)?;
                    }
                    Ok(Type::Path("()".to_string()))
                }
                Statement::For(stmt) => {
                    let start_ty =
                        self.infer_expr_with_loop(&stmt.start, &block_scope, loop_depth)?;
                    let end_ty = self.infer_expr_with_loop(&stmt.end, &block_scope, loop_depth)?;
                    let loop_var_ty =
                        Self::infer_for_loop_var_type(&stmt.start, &start_ty, &stmt.end, &end_ty)?;
                    let mut loop_scope = block_scope.clone();
                    loop_scope.insert(stmt.var_name.clone(), loop_var_ty);
                    self.check_block(&stmt.body, &mut loop_scope, loop_depth + 1)?;
                    Ok(Type::Path("()".to_string()))
                }
                Statement::Break | Statement::Continue => {
                    if loop_depth == 0 {
                        Err("break/continue can only be used inside for-loops".to_string())
                    } else {
                        Ok(Type::Path("()".to_string()))
                    }
                }
                Statement::Expr(e) => self.infer_expr_with_loop(e, &block_scope, loop_depth),
            }
        } else {
            Ok(Type::Path("()".to_string()))
        }
    }

    fn literal_kind(expr: &Expression) -> Option<LiteralKind> {
        match expr {
            Expression::Literal(Literal::Integer(_)) => Some(LiteralKind::Integer),
            Expression::Literal(Literal::String(_)) => Some(LiteralKind::String),
            _ => None,
        }
    }

    fn check_binding(
        &self,
        name: &str,
        ann: &Option<Type>,
        value: &Expression,
        scope: &HashMap<String, Type>,
        loop_depth: usize,
    ) -> Result<Type, String> {
        let rhs_type = self.infer_expr_with_loop(value, scope, loop_depth)?;
        if let Some(ann) = ann {
            self.validate_type(ann)?;
            if !Self::types_equal(ann, &rhs_type) {
                return Err(format!(
                    "Type Mismatch: Variable '{}' declared as {} but assigned {}",
                    name,
                    Self::type_display(ann),
                    Self::type_display(&rhs_type)
                ));
            }
        }
        Ok(rhs_type)
    }

    fn infer_for_loop_var_type(
        start_expr: &Expression,
        start_ty: &Type,
        end_expr: &Expression,
        end_ty: &Type,
    ) -> Result<Type, String> {
        if !Self::is_compatible_int_target(start_ty) || !Self::is_compatible_int_target(end_ty) {
            return Err(format!(
                "For range bounds must be integers, got {} and {}",
                Self::type_display(start_ty),
                Self::type_display(end_ty)
            ));
        }

        if Self::types_equal(start_ty, end_ty) {
            return Ok(start_ty.clone());
        }

        let start_is_int_literal =
            matches!(Self::literal_kind(start_expr), Some(LiteralKind::Integer));
        let end_is_int_literal = matches!(Self::literal_kind(end_expr), Some(LiteralKind::Integer));

        if start_is_int_literal {
            return Ok(end_ty.clone());
        }
        if end_is_int_literal {
            return Ok(start_ty.clone());
        }

        Err(format!(
            "For range type mismatch: {} vs {}. Use matching integer types or integer literals.",
            Self::type_display(start_ty),
            Self::type_display(end_ty)
        ))
    }

    fn is_printable_type(ty: &Type) -> bool {
        match ty {
            Type::Ref { inner, .. } => Self::is_printable_type(inner),
            Type::RawPtr(_) => false,
            Type::Path(name) => matches!(
                name.as_str(),
                "String"
                    | "core::types::String"
                    | "bool"
                    | "char"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "isize"
                    | "u8"
                    | "u16"
                    | "u32"
                    | "u64"
                    | "usize"
            ),
        }
    }
}
