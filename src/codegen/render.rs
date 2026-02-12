// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use super::CodeGenerator;
use crate::ast::Type;

impl CodeGenerator {
    pub(super) fn type_to_rust(ty: &Type) -> String {
        match ty {
            Type::Path(s) => Self::type_path_to_rust(s),
            Type::RawPtr(inner) => format!("*mut {}", Self::type_to_rust(inner)),
            Type::Ref { mutable, inner } => {
                if *mutable {
                    format!("&mut {}", Self::type_to_rust(inner))
                } else {
                    format!("&{}", Self::type_to_rust(inner))
                }
            }
        }
    }

    pub(super) fn type_path_to_rust(name: &str) -> String {
        if let Some(inner) = name.strip_prefix("&mut [") {
            let inner = inner.strip_suffix(']').unwrap_or(inner);
            return format!("&mut [{}]", Self::type_path_to_rust(inner));
        }
        if let Some(inner) = name.strip_prefix("&[") {
            let inner = inner.strip_suffix(']').unwrap_or(inner);
            return format!("&[{}]", Self::type_path_to_rust(inner));
        }
        if let Some(inner) = name.strip_prefix("&mut ") {
            return format!("&mut {}", Self::type_path_to_rust(inner));
        }
        if let Some(inner) = name.strip_prefix('&') {
            return format!("&{}", Self::type_path_to_rust(inner));
        }
        if let Some(inner) = name.strip_prefix('[') {
            let inner = inner.strip_suffix(']').unwrap_or(inner);
            return format!("[{}]", Self::type_path_to_rust(inner));
        }

        match name {
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "usize" | "isize"
            | "bool" | "char" | "()" => name.to_string(),
            "List" => "safe_lang::core::types::List".to_string(),
            "String" | "core::types::String" => "safe_lang::core::types::String".to_string(),
            "StringSplit" | "core::types::StringSplit" => {
                "safe_lang::core::types::StringSplit".to_string()
            }
            "StringList" | "core::types::StringList" => {
                "safe_lang::core::types::StringList".to_string()
            }
            "core::types::List" => "safe_lang::core::types::List".to_string(),
            "HighPtr" => "safe_lang::core::memory::safe::HighPtr".to_string(),
            "ValidatedPtr" => "safe_lang::core::memory::safe::ValidatedPtr".to_string(),
            "RawPtr" => "safe_lang::core::memory::raw::RawPtr".to_string(),
            "core::memory::safe::HighPtr"
            | "core::memory::safe::ValidatedPtr"
            | "core::memory::raw::RawPtr" => Self::render_type_path(name),
            other if other.starts_with("Option<") => {
                let inner = other.trim_start_matches("Option<").trim_end_matches('>');
                format!(
                    "safe_lang::core::types::Option<{}>",
                    Self::type_path_to_rust(inner)
                )
            }
            other if other.starts_with("Result<") => {
                let inner = other.trim_start_matches("Result<").trim_end_matches('>');
                let mut parts = inner.split(',').map(|p| p.trim());
                let ok_ty = parts.next().unwrap_or("()");
                let err_ty = parts.next().unwrap_or("()");
                format!(
                    "safe_lang::core::types::Result<{}, {}>",
                    Self::type_path_to_rust(ok_ty),
                    Self::type_path_to_rust(err_ty)
                )
            }
            other if other.starts_with("core::types::Option<") => {
                let inner = other
                    .trim_start_matches("core::types::Option<")
                    .trim_end_matches('>');
                format!(
                    "safe_lang::core::types::Option<{}>",
                    Self::type_path_to_rust(inner)
                )
            }
            other if other.starts_with("core::types::Result<") => {
                let inner = other
                    .trim_start_matches("core::types::Result<")
                    .trim_end_matches('>');
                let mut parts = inner.split(',').map(|p| p.trim());
                let ok_ty = parts.next().unwrap_or("()");
                let err_ty = parts.next().unwrap_or("()");
                format!(
                    "safe_lang::core::types::Result<{}, {}>",
                    Self::type_path_to_rust(ok_ty),
                    Self::type_path_to_rust(err_ty)
                )
            }
            other => Self::render_type_path(other),
        }
    }

    pub(super) fn escape_string(input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for ch in input.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                _ => out.push(ch),
            }
        }
        out
    }

    pub(super) fn render_function_name(name: &str) -> String {
        match name {
            "validate_raw" => "safe_lang::validate_raw".to_string(),
            "into_high" => "safe_lang::into_high".to_string(),
            other if other.starts_with("core::") => format!("safe_lang::{other}"),
            other => other.to_string(),
        }
    }

    pub(super) fn render_type_path(name: &str) -> String {
        if name.starts_with("core::") {
            format!("safe_lang::{name}")
        } else {
            name.to_string()
        }
    }
}
