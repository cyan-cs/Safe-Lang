// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::*;
use crate::std_api;

use super::TypeChecker;

impl TypeChecker {
    pub(super) fn type_display(ty: &Type) -> String {
        match ty {
            Type::Path(s) => s.clone(),
            Type::RawPtr(inner) => format!("*{}", Self::type_display(inner)),
            Type::Ref { mutable, inner } => {
                if *mutable {
                    format!("&mut {}", Self::type_display(inner))
                } else {
                    format!("&{}", Self::type_display(inner))
                }
            }
        }
    }

    pub(super) fn types_equal(lhs: &Type, rhs: &Type) -> bool {
        match (lhs, rhs) {
            (Type::RawPtr(l), Type::RawPtr(r)) => Self::types_equal(l, r),
            (
                Type::Ref {
                    mutable: lm,
                    inner: l,
                },
                Type::Ref {
                    mutable: rm,
                    inner: r,
                },
            ) => lm == rm && Self::types_equal(l, r),
            (Type::Path(l), Type::Path(r)) => {
                Self::canonicalize_type_path(l) == Self::canonicalize_type_path(r)
            }
            (Type::Ref { .. }, Type::Path(p)) => {
                let left = Self::canonicalize_type_path(&Self::type_display(lhs));
                let right = Self::canonicalize_type_path(p);
                left == right
            }
            (Type::Path(p), Type::Ref { .. }) => {
                let left = Self::canonicalize_type_path(p);
                let right = Self::canonicalize_type_path(&Self::type_display(rhs));
                left == right
            }
            _ => false,
        }
    }

    pub(super) fn canonicalize_type_path(name: &str) -> String {
        if let Some(inner) = name.strip_prefix("&mut [") {
            let inner = inner.strip_suffix(']').unwrap_or(inner);
            return format!("&mut [{}]", Self::canonicalize_type_path(inner));
        }
        if let Some(inner) = name.strip_prefix("&[") {
            let inner = inner.strip_suffix(']').unwrap_or(inner);
            return format!("&[{}]", Self::canonicalize_type_path(inner));
        }
        if let Some(inner) = name.strip_prefix("&mut ") {
            return format!("&mut {}", Self::canonicalize_type_path(inner));
        }
        if let Some(inner) = name.strip_prefix('&') {
            return format!("&{}", Self::canonicalize_type_path(inner));
        }
        if let Some(inner) = name.strip_prefix('[') {
            let inner = inner.strip_suffix(']').unwrap_or(inner);
            return format!("[{}]", Self::canonicalize_type_path(inner));
        }

        if let Some(start) = name.find('<') {
            if name.ends_with('>') {
                let base = name[..start].trim();
                let inner = &name[start + 1..name.len() - 1];
                if let Ok(args) = Self::split_generic_args(inner) {
                    let normalized_args = args
                        .iter()
                        .map(|arg| Self::canonicalize_type_path(arg))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let normalized_base = std_api::normalize_type_name(base);
                    return format!("{normalized_base}<{normalized_args}>");
                }
            }
        }

        std_api::normalize_type_name(name)
    }

    pub(super) fn is_compatible_int_target(ty: &Type) -> bool {
        matches!(
            ty,
            Type::Path(name)
                if matches!(
                    name.as_str(),
                    "i8" | "i16" | "i32" | "i64" | "isize" |
                    "u8" | "u16" | "u32" | "u64" | "usize"
                )
        )
    }

    pub(super) fn is_compatible_integer_return(block: &Block, expected: &Type) -> bool {
        if !Self::is_compatible_int_target(expected) {
            return false;
        }

        matches!(
            block.statements.last(),
            Some(Statement::Expr(Expression::Literal(Literal::Integer(_))))
        )
    }

    pub(super) fn validate_type(&self, ty: &Type) -> Result<(), String> {
        match ty {
            Type::RawPtr(inner) => self.validate_type(inner),
            Type::Ref { inner, .. } => self.validate_type(inner),
            Type::Path(name) => self.validate_type_path(name),
        }
    }

    fn validate_type_path(&self, name: &str) -> Result<(), String> {
        if let Some(inner) = name.strip_prefix("&mut [") {
            let inner = inner
                .strip_suffix(']')
                .ok_or_else(|| format!("Malformed type '{name}'"))?;
            return self.validate_type_path(inner);
        }
        if let Some(inner) = name.strip_prefix("&[") {
            let inner = inner
                .strip_suffix(']')
                .ok_or_else(|| format!("Malformed type '{name}'"))?;
            return self.validate_type_path(inner);
        }
        if let Some(inner) = name.strip_prefix("&mut ") {
            return self.validate_type_path(inner);
        }
        if let Some(inner) = name.strip_prefix('&') {
            return self.validate_type_path(inner);
        }
        if let Some(inner) = name.strip_prefix('[') {
            let inner = inner
                .strip_suffix(']')
                .ok_or_else(|| format!("Malformed type '{name}'"))?;
            return self.validate_type_path(inner);
        }
        if let Some((base, args)) = Self::parse_generic_type(name)? {
            let canonical_base = std_api::normalize_type_name(base);
            if canonical_base != "core::types::Option" && canonical_base != "core::types::Result" {
                return Err(format!(
                    "Generic type syntax is not supported in v0.1 except Option/Result: '{name}'"
                ));
            }
            let expected = if canonical_base == "core::types::Option" {
                1
            } else {
                2
            };
            if args.len() != expected {
                return Err(format!(
                    "Type '{}' expects {} generic argument(s), got {}",
                    base,
                    expected,
                    args.len()
                ));
            }
            for arg in args {
                self.validate_type_path(arg)?;
            }
            return Ok(());
        }

        let canonical = std_api::normalize_type_name(name);
        if self.known_types.contains(&canonical) || self.known_types.contains(name) {
            Ok(())
        } else {
            Err(format!("Unknown type '{name}'"))
        }
    }

    fn parse_generic_type(name: &str) -> Result<Option<(&str, Vec<&str>)>, String> {
        let Some(start) = name.find('<') else {
            return Ok(None);
        };
        if !name.ends_with('>') {
            return Err(format!("Malformed type '{name}'"));
        }
        let base = name[..start].trim();
        let inner = &name[start + 1..name.len() - 1];
        let args = Self::split_generic_args(inner)?;
        Ok(Some((base, args)))
    }

    fn split_generic_args(input: &str) -> Result<Vec<&str>, String> {
        let mut out = Vec::new();
        let mut depth = 0usize;
        let mut start = 0usize;
        for (idx, ch) in input.char_indices() {
            match ch {
                '<' => depth += 1,
                '>' => {
                    if depth == 0 {
                        return Err(format!("Malformed generic args '{input}'"));
                    }
                    depth -= 1;
                }
                ',' if depth == 0 => {
                    out.push(input[start..idx].trim());
                    start = idx + 1;
                }
                _ => {}
            }
        }
        if depth != 0 {
            return Err(format!("Malformed generic args '{input}'"));
        }
        let tail = input[start..].trim();
        if !tail.is_empty() {
            out.push(tail);
        }
        Ok(out)
    }
}
