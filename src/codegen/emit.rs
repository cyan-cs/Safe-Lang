// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use super::CodeGenerator;

impl CodeGenerator {
    pub(super) fn emit(&mut self, s: &str) {
        self.output.push_str(s);
    }

    pub(super) fn emit_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.emit("    ");
        }
    }
}
