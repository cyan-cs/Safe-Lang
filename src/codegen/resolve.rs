// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use super::CodeGenerator;
use std::collections::HashSet;

impl CodeGenerator {
    pub(super) fn resolve_alias_chain(&self, name: &str) -> Result<String, String> {
        let mut seen = HashSet::new();
        let mut current = name.to_string();

        while let Some(next) = self.aliases.get(&current) {
            if !seen.insert(current.clone()) {
                return Err(format!(
                    "Alias cycle detected during code generation: '{current}'"
                ));
            }
            current = next.clone();
        }

        Ok(current)
    }

    pub(super) fn ensure_known_function(&self, name: &str) -> Result<(), String> {
        if self.known_functions.contains(name) {
            Ok(())
        } else {
            Err(format!(
                "Unknown function '{name}' encountered during code generation"
            ))
        }
    }
}
