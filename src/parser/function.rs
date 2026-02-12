// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::*;
use crate::lexer::TokenKind;
use nom::IResult;
use nom::branch::alt;
use nom::multi::{many0, separated_list0};

use super::expr::parse_arg;
use super::helpers::{Input, expect_token, identifier};
use super::stmt::parse_block_content;
use super::types::parse_optional_type;

fn parse_alias(input: Input) -> IResult<Input, Item> {
    let (input, _) = expect_token(TokenKind::Alias)(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = expect_token(TokenKind::Equal)(input)?;
    let (input, target) = identifier(input)?;
    Ok((input, Item::Alias(Alias { name, target })))
}

pub fn parse_function(input: Input) -> IResult<Input, Item> {
    // Optional safety qualifier. If omitted, default to safe.
    let (input, safety) = match expect_token(TokenKind::Safe)(input) {
        Ok((i, _)) => (i, SafetyLevel::Safe),
        Err(_) => match expect_token(TokenKind::Raw)(input) {
            Ok((i, _)) => (i, SafetyLevel::Raw),
            Err(_) => (input, SafetyLevel::Safe), // Default, do not consume
        },
    };

    let (input, _) = expect_token(TokenKind::Fn)(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = expect_token(TokenKind::OpenParen)(input)?;
    let (input, args) = separated_list0(expect_token(TokenKind::Comma), parse_arg)(input)?;
    let (input, _) = expect_token(TokenKind::CloseParen)(input)?;

    // Return type -> Type
    let (input, ret_type) = parse_optional_type(input)?;

    let (input, _) = expect_token(TokenKind::OpenBrace)(input)?;
    let (input, statements) = parse_block_content(input)?;
    let (input, _) = expect_token(TokenKind::CloseBrace)(input)?;

    Ok((
        input,
        Item::Function(Function {
            name,
            safety,
            args,
            ret_type,
            body: Block {
                statements,
                unsafe_block: false,
            },
        }),
    ))
}

pub fn parse(input: Input) -> IResult<Input, SourceFile> {
    let (input, items) = many0(alt((parse_alias, parse_function)))(input)?;

    Ok((input, SourceFile { items }))
}
