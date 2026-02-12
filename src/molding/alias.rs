// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use super::Molder;

impl Molder {
    // Phase 1: Alias Expansion
    pub(super) fn phase1_expand_aliases(&mut self) -> Result<(), String> {
        self.load_aliases_from_file()?;
        let mut new_items = Vec::new();

        for item in self.source.items.drain(..) {
            if let Item::Alias(alias) = item {
                Self::validate_alias(&alias)?;

                if self
                    .aliases
                    .insert(alias.name.clone(), alias.target)
                    .is_some()
                {
                    return Err(format!(
                        "Phase 1 Error: Duplicate alias '{}' is not allowed",
                        alias.name
                    ));
                }
            } else {
                new_items.push(item);
            }
        }

        self.source.items = new_items;

        let mut resolved_aliases = HashMap::new();
        for name in self.aliases.keys() {
            let resolved = Self::resolve_alias_target(name, &self.aliases)?;
            resolved_aliases.insert(name.clone(), resolved);
        }

        for item in &mut self.source.items {
            if let Item::Function(func) = item {
                Self::expand_aliases_in_block(&mut func.body, &resolved_aliases);
            }
        }
        Ok(())
    }

    fn load_aliases_from_file(&mut self) -> Result<(), String> {
        let path = Path::new("rules.safe");
        if !path.exists() {
            return Ok(());
        }

        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Phase 1 Error: Failed to read rules.safe: {e}"))?;

        for (line_no, raw_line) in contents.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            let rest = line.strip_prefix("alias ").ok_or_else(|| {
                format!(
                    "Phase 1 Error: Invalid alias syntax at line {}",
                    line_no + 1
                )
            })?;
            let parts: Vec<&str> = rest.split('=').map(|s| s.trim()).collect();
            if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
                return Err(format!(
                    "Phase 1 Error: Invalid alias syntax at line {}",
                    line_no + 1
                ));
            }

            let alias = Alias {
                name: parts[0].to_string(),
                target: parts[1].to_string(),
            };
            Self::validate_alias(&alias)?;

            if self
                .aliases
                .insert(alias.name.clone(), alias.target)
                .is_some()
            {
                return Err(format!(
                    "Phase 1 Error: Duplicate alias '{}' is not allowed",
                    alias.name
                ));
            }
        }

        Ok(())
    }

    fn validate_alias(alias: &Alias) -> Result<(), String> {
        if alias.target.contains("unsafe") {
            return Err(format!(
                "Phase 1 Error: Alias target '{}' cannot include 'unsafe'",
                alias.target
            ));
        }
        Ok(())
    }

    fn resolve_alias_target(
        name: &str,
        aliases: &HashMap<String, String>,
    ) -> Result<String, String> {
        let mut seen = HashSet::new();
        let mut current = name.to_string();

        while let Some(next) = aliases.get(&current) {
            if !seen.insert(current.clone()) {
                return Err(format!(
                    "Phase 1 Error: Alias cycle detected while resolving '{name}'"
                ));
            }
            current = next.clone();
        }

        Ok(current)
    }

    fn expand_aliases_in_block(block: &mut Block, aliases: &HashMap<String, String>) {
        for stmt in &mut block.statements {
            match stmt {
                Statement::Let(parse_let) => {
                    Self::expand_aliases_in_expr(&mut parse_let.value, aliases)
                }
                Statement::Const(parse_const) => {
                    Self::expand_aliases_in_expr(&mut parse_const.value, aliases)
                }
                Statement::If(stmt) => {
                    Self::expand_aliases_in_expr(&mut stmt.condition, aliases);
                    Self::expand_aliases_in_block(&mut stmt.then_block, aliases);
                    if let Some(else_block) = &mut stmt.else_block {
                        Self::expand_aliases_in_block(else_block, aliases);
                    }
                }
                Statement::For(stmt) => {
                    Self::expand_aliases_in_expr(&mut stmt.start, aliases);
                    Self::expand_aliases_in_expr(&mut stmt.end, aliases);
                    Self::expand_aliases_in_block(&mut stmt.body, aliases);
                }
                Statement::Break | Statement::Continue => {}
                Statement::Expr(expr) => Self::expand_aliases_in_expr(expr, aliases),
            }
        }
    }

    fn expand_aliases_in_expr(expr: &mut Expression, aliases: &HashMap<String, String>) {
        match expr {
            Expression::Call(call) => {
                if let Some(target) = aliases.get(&call.func_name) {
                    call.func_name = target.clone();
                }
                for arg in &mut call.args {
                    Self::expand_aliases_in_expr(arg, aliases);
                }
            }
            Expression::Ref { expr, .. } => Self::expand_aliases_in_expr(expr, aliases),
            Expression::Binary { left, right, .. } => {
                Self::expand_aliases_in_expr(left, aliases);
                Self::expand_aliases_in_expr(right, aliases);
            }
            Expression::Block(block) => Self::expand_aliases_in_block(block, aliases),
            _ => {}
        }
    }
}
