// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::*;
use crate::std_api;
use std::collections::{HashMap, HashSet};

pub struct TypeChecker {
    pub(super) functions: HashMap<String, (Vec<Type>, Option<Type>)>,
    pub(super) builtins: HashSet<String>,
    pub(super) known_types: HashSet<String>,
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        let mut builtins = HashSet::new();
        let mut known_types = HashSet::new();

        for ty in [
            "i8", "i16", "i32", "i64", "isize", "u8", "u16", "u32", "u64", "usize", "bool", "char",
            "String", "()",
        ] {
            known_types.insert(ty.to_string());
        }
        for ty in std_api::known_type_names() {
            known_types.insert(ty.to_string());
        }

        for func in std_api::api_functions() {
            let arg_types = func
                .args
                .iter()
                .map(|name| std_api::type_from_str(name))
                .collect::<Vec<_>>();
            let ret_type = func.ret.map(std_api::type_from_str);

            Self::register_builtin(
                &mut functions,
                &mut builtins,
                func.name,
                &arg_types,
                ret_type.clone(),
            );
            if func.canonical != func.name {
                Self::register_builtin(
                    &mut functions,
                    &mut builtins,
                    func.canonical,
                    &arg_types,
                    ret_type,
                );
            }
        }

        for name in std_api::variadic_print_function_names() {
            builtins.insert((*name).to_string());
        }

        Self {
            functions,
            builtins,
            known_types,
        }
    }

    fn register_builtin(
        functions: &mut HashMap<String, (Vec<Type>, Option<Type>)>,
        builtins: &mut HashSet<String>,
        name: &str,
        arg_types: &[Type],
        ret_type: Option<Type>,
    ) {
        functions.insert(name.to_string(), (arg_types.to_vec(), ret_type));
        builtins.insert(name.to_string());
    }

    pub fn check(&mut self, source: &SourceFile) -> Result<(), String> {
        for item in &source.items {
            if let Item::Struct(s) = item {
                self.known_types.insert(s.name.clone());
            }
        }

        for item in &source.items {
            if let Item::Function(func) = item {
                if self.builtins.contains(&func.name) {
                    return Err(format!(
                        "Builtin function '{}' cannot be redefined",
                        func.name
                    ));
                }
                if self.functions.contains_key(&func.name) {
                    return Err(format!("Duplicate function definition '{}'", func.name));
                }
                for arg in &func.args {
                    self.validate_type(&arg.ty)?;
                }
                if let Some(ret) = &func.ret_type {
                    self.validate_type(ret)?;
                }
                let arg_types = func.args.iter().map(|arg| arg.ty.clone()).collect();
                self.functions
                    .insert(func.name.clone(), (arg_types, func.ret_type.clone()));
            }
        }

        for item in &source.items {
            if let Item::Function(func) = item {
                self.check_function(func)?;
            }
        }
        Ok(())
    }

    fn check_function(&self, func: &Function) -> Result<(), String> {
        let mut symbols = HashMap::new();
        for arg in &func.args {
            symbols.insert(arg.name.clone(), arg.ty.clone());
        }

        self.check_block(&func.body, &mut symbols, 0)?;

        let inferred_return = self.infer_block_expr(&func.body, &symbols)?;
        let expected_return = func
            .ret_type
            .clone()
            .unwrap_or_else(|| Type::Path("()".to_string()));

        if !Self::types_equal(&inferred_return, &expected_return) {
            if Self::is_compatible_integer_return(&func.body, &expected_return) {
                return Ok(());
            }

            return Err(format!(
                "Return Type Mismatch in '{}': expected {}, got {}",
                func.name,
                Self::type_display(&expected_return),
                Self::type_display(&inferred_return)
            ));
        }

        Ok(())
    }
}
