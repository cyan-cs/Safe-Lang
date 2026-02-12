// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::*;
use crate::std_api;
use std::collections::{HashMap, HashSet};

pub struct CodeGenerator {
    pub(super) indent_level: usize,
    pub(super) output: String,
    pub(super) aliases: HashMap<String, String>,
    pub(super) known_functions: HashSet<String>,
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            output: String::new(),
            aliases: HashMap::new(),
            known_functions: HashSet::new(),
        }
    }

    pub fn generate(&mut self, source: &SourceFile) -> Result<String, String> {
        self.output.clear();
        self.indent_level = 0;
        self.aliases.clear();
        self.known_functions.clear();

        for func in std_api::api_functions() {
            self.known_functions.insert(func.name.to_string());
            if func.canonical != func.name {
                self.known_functions.insert(func.canonical.to_string());
            }
        }
        for name in std_api::variadic_print_function_names() {
            self.known_functions.insert((*name).to_string());
        }

        for item in &source.items {
            if let Item::Alias(alias) = item {
                self.aliases
                    .insert(alias.name.clone(), alias.target.clone());
            }
            if let Item::Function(func) = item {
                self.known_functions.insert(func.name.clone());
            }
        }

        self.emit("// Generated SAFE? Code\n");
        self.emit("// This is a Rust transpilation of SAFE? source\n");
        self.emit("// Requires the `safe_lang` runtime crate.\n\n");

        for item in &source.items {
            match item {
                Item::Function(func) => {
                    self.generate_function(func)?;
                    self.emit("\n");
                }
                Item::Alias(_) => {}
                Item::Struct(s) => {
                    self.generate_struct(s);
                    self.emit("\n");
                }
            }
        }

        Ok(self.output.clone())
    }

    fn generate_function(&mut self, func: &Function) -> Result<(), String> {
        let safety = match func.safety {
            SafetyLevel::Safe => "pub fn",
            SafetyLevel::Raw => "pub unsafe fn",
        };

        self.emit(&format!("{} {}(", safety, func.name));
        for (i, arg) in func.args.iter().enumerate() {
            if i > 0 {
                self.emit(", ");
            }
            self.emit(&format!("{}: {}", arg.name, Self::type_to_rust(&arg.ty)));
        }
        self.emit(")");

        if let Some(ret) = &func.ret_type {
            self.emit(&format!(" -> {}", Self::type_to_rust(ret)));
        }

        self.emit(" {\n");
        self.indent_level += 1;

        for (idx, stmt) in func.body.statements.iter().enumerate() {
            let is_last = idx + 1 == func.body.statements.len();
            let trailing_semicolon =
                !(is_last && matches!(stmt, Statement::Expr(_)) && func.ret_type.is_some());
            self.generate_statement(stmt, trailing_semicolon)?;
        }

        self.indent_level -= 1;
        self.emit("}\n");
        Ok(())
    }

    fn generate_statement(&mut self, stmt: &Statement, semicolon: bool) -> Result<(), String> {
        self.emit_indent();
        match stmt {
            Statement::Let(l) => {
                if let Some(ty) = &l.ty {
                    self.emit(&format!("let {}: {} = ", l.name, Self::type_to_rust(ty)));
                } else {
                    self.emit(&format!("let {} = ", l.name));
                }
                self.generate_expression(&l.value)?;
                self.emit(";\n");
            }
            Statement::Const(c) => {
                if let Some(ty) = &c.ty {
                    self.emit(&format!("let {}: {} = ", c.name, Self::type_to_rust(ty)));
                } else {
                    self.emit(&format!("let {} = ", c.name));
                }
                self.generate_expression(&c.value)?;
                self.emit(";\n");
            }
            Statement::If(stmt) => {
                self.emit("if ");
                self.generate_expression(&stmt.condition)?;
                self.emit(" {\n");
                self.indent_level += 1;
                for inner in &stmt.then_block.statements {
                    self.generate_statement(inner, true)?;
                }
                self.indent_level -= 1;
                self.emit_indent();
                self.emit("}");
                if let Some(else_block) = &stmt.else_block {
                    self.emit(" else {\n");
                    self.indent_level += 1;
                    for inner in &else_block.statements {
                        self.generate_statement(inner, true)?;
                    }
                    self.indent_level -= 1;
                    self.emit_indent();
                    self.emit("}");
                }
                self.emit("\n");
            }
            Statement::For(stmt) => {
                self.emit(&format!("for {} in ", stmt.var_name));
                self.generate_expression(&stmt.start)?;
                if stmt.inclusive {
                    self.emit("..=");
                } else {
                    self.emit("..");
                }
                self.generate_expression(&stmt.end)?;
                self.emit(" {\n");
                self.indent_level += 1;
                for inner in &stmt.body.statements {
                    self.generate_statement(inner, true)?;
                }
                self.indent_level -= 1;
                self.emit_indent();
                self.emit("}\n");
            }
            Statement::Break => self.emit("break;\n"),
            Statement::Continue => self.emit("continue;\n"),
            Statement::Expr(e) => {
                self.generate_expression(e)?;
                if semicolon {
                    self.emit(";\n");
                } else {
                    self.emit("\n");
                }
            }
        }
        Ok(())
    }

    fn generate_expression(&mut self, expr: &Expression) -> Result<(), String> {
        match expr {
            Expression::Literal(lit) => match lit {
                Literal::Integer(i) => self.emit(&i.to_string()),
                Literal::String(s) => self.emit(&format!(
                    "safe_lang::core::types::String::from(\"{}\")",
                    Self::escape_string(s)
                )),
                Literal::Bool(value) => self.emit(if *value { "true" } else { "false" }),
            },
            Expression::Variable(name) => self.emit(name),
            Expression::Binary { op, left, right } => {
                self.generate_expression(left)?;
                let op_str = match op {
                    BinaryOp::Equal => " == ",
                    BinaryOp::NotEqual => " != ",
                    BinaryOp::LessThan => " < ",
                    BinaryOp::LessEqual => " <= ",
                    BinaryOp::GreaterThan => " > ",
                    BinaryOp::GreaterEqual => " >= ",
                };
                self.emit(op_str);
                self.generate_expression(right)?;
            }
            Expression::Ref { mutable, expr } => {
                if *mutable {
                    self.emit("&mut ");
                } else {
                    self.emit("&");
                }
                self.generate_expression(expr)?;
            }
            Expression::Call(call) => {
                let func_name = self.resolve_alias_chain(&call.func_name)?;
                self.ensure_known_function(&func_name)?;
                if std_api::is_print_function(&func_name) || std_api::is_printl_function(&func_name)
                {
                    self.generate_print_call(&func_name, &call.args)?;
                    return Ok(());
                }
                let rendered = Self::render_function_name(&func_name);
                self.emit(&format!("{rendered}("));
                for (i, arg) in call.args.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.generate_expression(arg)?;
                }
                self.emit(")");
            }
            Expression::Block(block) => {
                if block.unsafe_block {
                    self.emit("unsafe ");
                }
                self.emit("{\n");
                self.indent_level += 1;

                for (idx, stmt) in block.statements.iter().enumerate() {
                    let is_last = idx + 1 == block.statements.len();
                    let trailing_semicolon = !is_last;
                    self.generate_statement(stmt, trailing_semicolon)?;
                }

                self.indent_level -= 1;
                self.emit_indent();
                self.emit("}");
            }
        }
        Ok(())
    }

    fn generate_print_call(&mut self, func_name: &str, args: &[Expression]) -> Result<(), String> {
        self.emit("{ ");
        for arg in args {
            self.emit("safe_lang::core::types::print_any(&(");
            self.generate_expression(arg)?;
            self.emit(")); ");
        }
        if std_api::is_printl_function(func_name) {
            self.emit("std::println!(); ");
        }
        self.emit("}");
        Ok(())
    }

    fn generate_struct(&mut self, s: &Struct) {
        self.emit(&format!("pub struct {} {{\n", s.name));
        self.indent_level += 1;
        for f in &s.fields {
            self.emit_indent();
            self.emit(&format!("pub {}: {},\n", f.name, Self::type_to_rust(&f.ty)));
        }
        self.indent_level -= 1;
        self.emit("}\n");
    }
}
