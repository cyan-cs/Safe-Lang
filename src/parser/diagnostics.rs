// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::SourceFile;

use super::function::parse;
use super::helpers::Input;

pub fn parse_with_diagnostics(input: Input) -> Result<SourceFile, String> {
    match parse(input) {
        Ok((rest, source)) => {
            if rest.is_empty() {
                Ok(source)
            } else {
                let token = &rest[0];
                Err(format!(
                    "Parse error: unconsumed token {:?} at line {}, column {}",
                    token.kind, token.span.line, token.span.column
                ))
            }
        }
        Err(nom::Err::Error(err)) | Err(nom::Err::Failure(err)) => {
            if let Some(token) = err.input.first() {
                Err(format!(
                    "Parse error near token {:?} at line {}, column {}",
                    token.kind, token.span.line, token.span.column
                ))
            } else {
                Err("Parse error at end of input".to_string())
            }
        }
        Err(nom::Err::Incomplete(_)) => Err("Parse error: incomplete input".to_string()),
    }
}
