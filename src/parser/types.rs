// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::Type;
use crate::lexer::TokenKind;
use nom::IResult;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::preceded;

use super::helpers::{Input, expect_token, identifier};

pub fn parse_type(input: Input) -> IResult<Input, Type> {
    parse_type_recursive(input)
}

fn parse_type_recursive(input: Input) -> IResult<Input, Type> {
    // Check for Raw Pointer `*`
    if let Ok((rest, _)) = expect_token(TokenKind::Star)(input) {
        let (rest, inner_ty) = parse_type_recursive(rest)?;
        return Ok((rest, Type::RawPtr(Box::new(inner_ty))));
    }

    // Check for Reference `&` or `&mut`
    if let Ok((rest, _)) = expect_token(TokenKind::Ampersand)(input) {
        let (rest, mutable) =
            if let Ok((rest, _)) = expect_token(TokenKind::Identifier("mut".to_string()))(rest) {
                (rest, true)
            } else {
                (rest, false)
            };

        if let Ok((inner_rest, _)) = expect_token(TokenKind::OpenBracket)(rest) {
            let (inner_rest, inner_ty) = parse_type_recursive(inner_rest)?;
            let (inner_rest, _) = expect_token(TokenKind::CloseBracket)(inner_rest)?;
            let type_str = if mutable {
                format!("&mut [{}]", type_to_string(&inner_ty))
            } else {
                format!("&[{}]", type_to_string(&inner_ty))
            };
            return Ok((inner_rest, Type::Path(type_str)));
        } else {
            let (inner_rest, inner_ty) = parse_type_recursive(rest)?;
            return Ok((
                inner_rest,
                Type::Ref {
                    mutable,
                    inner: Box::new(inner_ty),
                },
            ));
        }
    }

    // Check for `[` (Slice/Array)
    if let Ok((rest, _)) = expect_token(TokenKind::OpenBracket)(input) {
        let (rest, inner_ty) = parse_type_recursive(rest)?;
        let (rest, _) = expect_token(TokenKind::CloseBracket)(rest)?;
        let type_str = format!("[{}]", type_to_string(&inner_ty));
        return Ok((rest, Type::Path(type_str)));
    }

    // Identifier (Path)
    let (input, name) = identifier(input)?;

    // Check for Generics <...>
    if let Ok((rest, _)) = expect_token(TokenKind::LessThan)(input) {
        let (rest, args) =
            separated_list0(expect_token(TokenKind::Comma), parse_type_recursive)(rest)?;
        let (rest, _) = expect_token(TokenKind::GreaterThan)(rest)?;

        // Reconstruct as path string for now
        let args_str = args
            .iter()
            .map(type_to_string)
            .collect::<Vec<_>>()
            .join(", ");
        let full_type = format!("{name}<{args_str}>");

        Ok((rest, Type::Path(full_type)))
    } else {
        Ok((input, Type::Path(name)))
    }
}

// Helper to convert Type back to string (since AST uses Type::Path(String))
fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::Path(s) => s.clone(),
        Type::RawPtr(inner) => format!("*{}", type_to_string(inner)),
        Type::Ref { mutable, inner } => {
            if *mutable {
                format!("&mut {}", type_to_string(inner))
            } else {
                format!("&{}", type_to_string(inner))
            }
        }
    }
}

pub fn parse_optional_type(input: Input) -> IResult<Input, Option<Type>> {
    opt(preceded(expect_token(TokenKind::Arrow), parse_type))(input)
}
