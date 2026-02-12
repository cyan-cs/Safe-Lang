// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::*;
use crate::std_api;

use super::Molder;

impl Molder {
    // Phase 2: Fully Qualified Names and Type Normalization
    pub(super) fn phase2_normalize_types(&mut self) -> Result<(), String> {
        for item in &mut self.source.items {
            if let Item::Function(func) = item {
                for arg in &mut func.args {
                    Self::normalize_type(&mut arg.ty);
                }
                if let Some(ret) = &mut func.ret_type {
                    Self::normalize_type(ret);
                }
                Self::normalize_block_types(&mut func.body);
                Self::normalize_block_calls(&mut func.body);
            }
        }
        Ok(())
    }

    fn normalize_block_types(block: &mut Block) {
        for stmt in &mut block.statements {
            match stmt {
                Statement::Let(l) => {
                    if let Some(ty) = &mut l.ty {
                        Self::normalize_type(ty);
                    }
                    Self::normalize_expr_types(&mut l.value);
                }
                Statement::Const(c) => {
                    if let Some(ty) = &mut c.ty {
                        Self::normalize_type(ty);
                    }
                    Self::normalize_expr_types(&mut c.value);
                }
                Statement::If(stmt) => {
                    Self::normalize_expr_types(&mut stmt.condition);
                    Self::normalize_block_types(&mut stmt.then_block);
                    if let Some(else_block) = &mut stmt.else_block {
                        Self::normalize_block_types(else_block);
                    }
                }
                Statement::For(stmt) => {
                    Self::normalize_expr_types(&mut stmt.start);
                    Self::normalize_expr_types(&mut stmt.end);
                    Self::normalize_block_types(&mut stmt.body);
                }
                Statement::Break | Statement::Continue => {}
                Statement::Expr(e) => Self::normalize_expr_types(e),
            }
        }
    }

    fn normalize_expr_types(expr: &mut Expression) {
        match expr {
            Expression::Block(block) => Self::normalize_block_types(block),
            Expression::Call(call) => {
                for arg in &mut call.args {
                    Self::normalize_expr_types(arg);
                }
            }
            Expression::Binary { left, right, .. } => {
                Self::normalize_expr_types(left);
                Self::normalize_expr_types(right);
            }
            Expression::Ref { expr, .. } => Self::normalize_expr_types(expr),
            _ => {}
        }
    }

    fn normalize_type(ty: &mut Type) {
        match ty {
            Type::Path(name) => {
                if name == "HighPtr" {
                    *name = "core::memory::safe::HighPtr".to_string();
                } else if name == "ValidatedPtr" {
                    *name = "core::memory::safe::ValidatedPtr".to_string();
                } else if name == "RawPtr" {
                    *name = "core::memory::raw::RawPtr".to_string();
                }
            }
            Type::RawPtr(inner) => Self::normalize_type(inner),
            Type::Ref { inner, .. } => Self::normalize_type(inner),
        }
    }

    fn normalize_block_calls(block: &mut Block) {
        for stmt in &mut block.statements {
            match stmt {
                Statement::Let(l) => Self::normalize_expr_calls(&mut l.value),
                Statement::Const(c) => Self::normalize_expr_calls(&mut c.value),
                Statement::If(stmt) => {
                    Self::normalize_expr_calls(&mut stmt.condition);
                    Self::normalize_block_calls(&mut stmt.then_block);
                    if let Some(else_block) = &mut stmt.else_block {
                        Self::normalize_block_calls(else_block);
                    }
                }
                Statement::For(stmt) => {
                    Self::normalize_expr_calls(&mut stmt.start);
                    Self::normalize_expr_calls(&mut stmt.end);
                    Self::normalize_block_calls(&mut stmt.body);
                }
                Statement::Break | Statement::Continue => {}
                Statement::Expr(e) => Self::normalize_expr_calls(e),
            }
        }
    }

    fn normalize_expr_calls(expr: &mut Expression) {
        match expr {
            Expression::Call(call) => {
                call.func_name = Self::normalize_function_name(&call.func_name);
                for arg in &mut call.args {
                    Self::normalize_expr_calls(arg);
                }
            }
            Expression::Binary { left, right, .. } => {
                Self::normalize_expr_calls(left);
                Self::normalize_expr_calls(right);
            }
            Expression::Ref { expr, .. } => Self::normalize_expr_calls(expr),
            Expression::Block(block) => Self::normalize_block_calls(block),
            _ => {}
        }
    }

    fn normalize_function_name(name: &str) -> String {
        if let Some(canonical) = std_api::canonical_name(name) {
            canonical.to_string()
        } else {
            name.to_string()
        }
    }
}
