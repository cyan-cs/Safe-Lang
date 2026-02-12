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
    // Phase 3: Explicit Unsafe (auto-wrap raw calls)
    pub(super) fn phase3_explicit_unsafe(&mut self) -> Result<(), String> {
        let raw_functions = self.raw_functions.clone();
        for item in &mut self.source.items {
            if let Item::Function(func) = item {
                let func_unsafe = matches!(func.safety, SafetyLevel::Raw);
                Self::wrap_raw_calls_in_block(&mut func.body, func_unsafe, &raw_functions)?;
                Self::verify_unsafe_boundaries(&func.body, func_unsafe, &raw_functions)?;
            }
        }
        Ok(())
    }

    fn wrap_raw_calls_in_block(
        block: &mut Block,
        in_unsafe: bool,
        raw_functions: &HashSet<String>,
    ) -> Result<(), String> {
        let current_unsafe = in_unsafe || block.unsafe_block;
        for stmt in &mut block.statements {
            match stmt {
                Statement::Let(l) => {
                    l.value = Self::wrap_raw_calls_in_expr(
                        l.value.clone(),
                        current_unsafe,
                        raw_functions,
                    )?;
                }
                Statement::Const(c) => {
                    c.value = Self::wrap_raw_calls_in_expr(
                        c.value.clone(),
                        current_unsafe,
                        raw_functions,
                    )?;
                }
                Statement::If(stmt) => {
                    stmt.condition = Self::wrap_raw_calls_in_expr(
                        stmt.condition.clone(),
                        current_unsafe,
                        raw_functions,
                    )?;
                    Self::wrap_raw_calls_in_block(
                        &mut stmt.then_block,
                        current_unsafe,
                        raw_functions,
                    )?;
                    if let Some(else_block) = &mut stmt.else_block {
                        Self::wrap_raw_calls_in_block(else_block, current_unsafe, raw_functions)?;
                    }
                }
                Statement::For(stmt) => {
                    stmt.start = Self::wrap_raw_calls_in_expr(
                        stmt.start.clone(),
                        current_unsafe,
                        raw_functions,
                    )?;
                    stmt.end = Self::wrap_raw_calls_in_expr(
                        stmt.end.clone(),
                        current_unsafe,
                        raw_functions,
                    )?;
                    Self::wrap_raw_calls_in_block(&mut stmt.body, current_unsafe, raw_functions)?;
                }
                Statement::Break | Statement::Continue => {}
                Statement::Expr(e) => {
                    *e = Self::wrap_raw_calls_in_expr(e.clone(), current_unsafe, raw_functions)?;
                }
            }
        }
        Ok(())
    }

    fn wrap_raw_calls_in_expr(
        expr: Expression,
        in_unsafe: bool,
        raw_functions: &HashSet<String>,
    ) -> Result<Expression, String> {
        if let Expression::Block(mut block) = expr.clone() {
            if block.unsafe_block {
                let _ = Self::wrap_raw_calls_in_block(&mut block, true, raw_functions);
                return Ok(Expression::Block(block));
            }
        }

        if in_unsafe {
            return Ok(Self::wrap_raw_calls_in_expr_inner(
                expr,
                in_unsafe,
                raw_functions,
            ));
        }

        if Self::expr_contains_raw_call(&expr, raw_functions) {
            let inner = Self::wrap_raw_calls_in_expr_inner(expr, true, raw_functions);
            return Ok(Expression::Block(Block {
                statements: vec![Statement::Expr(inner)],
                unsafe_block: true,
            }));
        }

        Ok(Self::wrap_raw_calls_in_expr_inner(
            expr,
            in_unsafe,
            raw_functions,
        ))
    }

    fn wrap_raw_calls_in_expr_inner(
        expr: Expression,
        in_unsafe: bool,
        raw_functions: &HashSet<String>,
    ) -> Expression {
        match expr {
            Expression::Call(mut call) => {
                for arg in &mut call.args {
                    *arg =
                        Self::wrap_raw_calls_in_expr_inner(arg.clone(), in_unsafe, raw_functions);
                }
                Expression::Call(call)
            }
            Expression::Binary { op, left, right } => Expression::Binary {
                op,
                left: Box::new(Self::wrap_raw_calls_in_expr_inner(
                    *left,
                    in_unsafe,
                    raw_functions,
                )),
                right: Box::new(Self::wrap_raw_calls_in_expr_inner(
                    *right,
                    in_unsafe,
                    raw_functions,
                )),
            },
            Expression::Ref { mutable, expr } => Expression::Ref {
                mutable,
                expr: Box::new(Self::wrap_raw_calls_in_expr_inner(
                    *expr,
                    in_unsafe,
                    raw_functions,
                )),
            },
            Expression::Block(mut block) => {
                let _ = Self::wrap_raw_calls_in_block(&mut block, in_unsafe, raw_functions);
                Expression::Block(block)
            }
            other => other,
        }
    }

    fn expr_contains_raw_call(expr: &Expression, raw_functions: &HashSet<String>) -> bool {
        match expr {
            Expression::Call(call) => {
                Self::is_raw_operation(&call.func_name, raw_functions)
                    || call
                        .args
                        .iter()
                        .any(|arg| Self::expr_contains_raw_call(arg, raw_functions))
            }
            Expression::Binary { left, right, .. } => {
                Self::expr_contains_raw_call(left, raw_functions)
                    || Self::expr_contains_raw_call(right, raw_functions)
            }
            Expression::Ref { expr, .. } => Self::expr_contains_raw_call(expr, raw_functions),
            Expression::Block(block) => block.statements.iter().any(|stmt| match stmt {
                Statement::Let(l) => Self::expr_contains_raw_call(&l.value, raw_functions),
                Statement::Const(c) => Self::expr_contains_raw_call(&c.value, raw_functions),
                Statement::If(stmt) => {
                    Self::expr_contains_raw_call(&stmt.condition, raw_functions)
                        || Self::expr_contains_raw_call(
                            &Expression::Block(stmt.then_block.clone()),
                            raw_functions,
                        )
                        || stmt.else_block.as_ref().is_some_and(|b| {
                            Self::expr_contains_raw_call(
                                &Expression::Block(b.clone()),
                                raw_functions,
                            )
                        })
                }
                Statement::For(stmt) => {
                    Self::expr_contains_raw_call(&stmt.start, raw_functions)
                        || Self::expr_contains_raw_call(&stmt.end, raw_functions)
                        || Self::expr_contains_raw_call(
                            &Expression::Block(stmt.body.clone()),
                            raw_functions,
                        )
                }
                Statement::Break | Statement::Continue => false,
                Statement::Expr(e) => Self::expr_contains_raw_call(e, raw_functions),
            }),
            _ => false,
        }
    }

    fn verify_unsafe_boundaries(
        block: &Block,
        in_unsafe: bool,
        raw_functions: &HashSet<String>,
    ) -> Result<(), String> {
        let current_unsafe = in_unsafe || block.unsafe_block;

        for stmt in &block.statements {
            match stmt {
                Statement::Let(l) => {
                    Self::verify_unsafe_in_expr(&l.value, current_unsafe, raw_functions)?;
                }
                Statement::Const(c) => {
                    Self::verify_unsafe_in_expr(&c.value, current_unsafe, raw_functions)?;
                }
                Statement::If(stmt) => {
                    Self::verify_unsafe_in_expr(&stmt.condition, current_unsafe, raw_functions)?;
                    Self::verify_unsafe_boundaries(
                        &stmt.then_block,
                        current_unsafe,
                        raw_functions,
                    )?;
                    if let Some(else_block) = &stmt.else_block {
                        Self::verify_unsafe_boundaries(else_block, current_unsafe, raw_functions)?;
                    }
                }
                Statement::For(stmt) => {
                    Self::verify_unsafe_in_expr(&stmt.start, current_unsafe, raw_functions)?;
                    Self::verify_unsafe_in_expr(&stmt.end, current_unsafe, raw_functions)?;
                    Self::verify_unsafe_boundaries(&stmt.body, current_unsafe, raw_functions)?;
                }
                Statement::Break | Statement::Continue => {}
                Statement::Expr(e) => {
                    Self::verify_unsafe_in_expr(e, current_unsafe, raw_functions)?;
                }
            }
        }
        Ok(())
    }

    fn verify_unsafe_in_expr(
        expr: &Expression,
        in_unsafe: bool,
        raw_functions: &HashSet<String>,
    ) -> Result<(), String> {
        match expr {
            Expression::Call(call) => {
                if Self::is_raw_operation(&call.func_name, raw_functions) && !in_unsafe {
                    return Err(format!(
                        "Phase 3 Error: Raw function '{}' called outside unsafe block. Wrap with `unsafe {{ ... }}`",
                        call.func_name
                    ));
                }
                for arg in &call.args {
                    Self::verify_unsafe_in_expr(arg, in_unsafe, raw_functions)?;
                }
            }
            Expression::Binary { left, right, .. } => {
                Self::verify_unsafe_in_expr(left, in_unsafe, raw_functions)?;
                Self::verify_unsafe_in_expr(right, in_unsafe, raw_functions)?;
            }
            Expression::Ref { expr, .. } => {
                Self::verify_unsafe_in_expr(expr, in_unsafe, raw_functions)?;
            }
            Expression::Block(b) => {
                Self::verify_unsafe_boundaries(b, in_unsafe, raw_functions)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn is_raw_operation(name: &str, raw_functions: &HashSet<String>) -> bool {
        name.starts_with("raw_") || name.contains("::raw::") || raw_functions.contains(name)
    }
}
