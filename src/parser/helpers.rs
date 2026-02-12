// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::lexer::{Token, TokenKind};
use nom::IResult;

// Input type is now a slice of Tokens
pub type Input<'a> = &'a [Token];

// Helper to match a specific token kind
pub fn expect_token(expected: TokenKind) -> impl FnMut(Input) -> IResult<Input, &Token> {
    move |input: Input| {
        if input.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Eof,
            )));
        }

        // Simple comparison
        if input[0].kind == expected {
            Ok((&input[1..], &input[0]))
        } else {
            // Silenced error printing to avoid backtracking noise
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    }
}

// Helper to extract identifier string
pub fn identifier(input: Input) -> IResult<Input, String> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    }
    match &input[0].kind {
        TokenKind::Identifier(name) => Ok((&input[1..], name.clone())),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

// Helper to extract integer literal
pub fn integer_literal(input: Input) -> IResult<Input, String> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    }
    match &input[0].kind {
        TokenKind::Integer(val) => Ok((&input[1..], val.clone())),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

// Helper to extract string literal
pub fn string_literal(input: Input) -> IResult<Input, String> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    }
    match &input[0].kind {
        TokenKind::StringLiteral(val) => Ok((&input[1..], val.clone())),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}
