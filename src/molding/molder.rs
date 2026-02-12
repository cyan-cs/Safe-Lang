// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::*;
use std::collections::{HashMap, HashSet};

pub struct Molder {
    pub(super) source: SourceFile,
    pub(super) aliases: HashMap<String, String>,
    pub(super) raw_functions: HashSet<String>,
}

impl Molder {
    pub fn new(source: SourceFile) -> Self {
        Self {
            source,
            aliases: HashMap::new(),
            raw_functions: HashSet::new(),
        }
    }

    pub fn mold(&mut self) -> Result<(), String> {
        for item in &self.source.items {
            if let Item::Function(func) = item {
                if let SafetyLevel::Raw = func.safety {
                    self.raw_functions.insert(func.name.clone());
                }
            }
        }

        self.phase1_expand_aliases()?;
        self.phase2_normalize_types()?;
        self.phase3_explicit_unsafe()?;
        self.phase4_verify_rules()?;
        Ok(())
    }

    pub fn get_output(&self) -> &SourceFile {
        &self.source
    }
}
