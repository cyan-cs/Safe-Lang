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
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::preceded;

use super::expr::parse_expression;
use super::helpers::{Input, expect_token, identifier};
use super::types::parse_type;

fn parse_let(input: Input) -> IResult<Input, Statement> {
    let (input, _) = expect_token(TokenKind::Let)(input)?;
    let (input, name) = identifier(input)?;

    let (input, ty) = opt(preceded(expect_token(TokenKind::Colon), parse_type))(input)?;

    let (input, _) = expect_token(TokenKind::Equal)(input)?;
    let (input, value) = parse_expression(input)?;

    Ok((input, Statement::Let(LetStatement { name, ty, value })))
}

fn parse_const(input: Input) -> IResult<Input, Statement> {
    let (input, _) = expect_token(TokenKind::Const)(input)?;
    let (input, name) = identifier(input)?;

    let (input, ty) = opt(preceded(expect_token(TokenKind::Colon), parse_type))(input)?;

    let (input, _) = expect_token(TokenKind::Equal)(input)?;
    let (input, value) = parse_expression(input)?;

    Ok((input, Statement::Const(ConstStatement { name, ty, value })))
}

fn parse_if_statement(input: Input) -> IResult<Input, IfStatement> {
    let (input, _) = expect_token(TokenKind::If)(input)?;
    let (input, condition) = parse_expression(input)?;
    let (input, then_block) = parse_block(input)?;

    let (input, else_block) = if let Ok((input, _)) = expect_token(TokenKind::Else)(input) {
        if let Ok((input_after_if, nested_if)) = parse_if_statement(input) {
            (
                input_after_if,
                Some(Block {
                    statements: vec![Statement::If(nested_if)],
                    unsafe_block: false,
                }),
            )
        } else {
            let (input, block) = parse_block(input)?;
            (input, Some(block))
        }
    } else {
        (input, None)
    };

    Ok((
        input,
        IfStatement {
            condition,
            then_block,
            else_block,
        },
    ))
}

fn parse_if(input: Input) -> IResult<Input, Statement> {
    let (input, stmt) = parse_if_statement(input)?;
    Ok((input, Statement::If(stmt)))
}

fn parse_for(input: Input) -> IResult<Input, Statement> {
    let (input, _) = expect_token(TokenKind::For)(input)?;
    let (input, var_name) = identifier(input)?;
    let (input, _) = expect_token(TokenKind::In)(input)?;

    let (input, start) = parse_expression(input)?;
    let (input, inclusive) = if let Ok((input, _)) = expect_token(TokenKind::DotDotEqual)(input) {
        (input, true)
    } else {
        let (input, _) = expect_token(TokenKind::DotDot)(input)?;
        (input, false)
    };
    let (input, end) = parse_expression(input)?;
    let (input, body) = parse_block(input)?;

    Ok((
        input,
        Statement::For(ForStatement {
            var_name,
            start,
            end,
            inclusive,
            body,
        }),
    ))
}

fn parse_break(input: Input) -> IResult<Input, Statement> {
    let (input, _) = expect_token(TokenKind::Break)(input)?;
    Ok((input, Statement::Break))
}

fn parse_continue(input: Input) -> IResult<Input, Statement> {
    let (input, _) = expect_token(TokenKind::Continue)(input)?;
    Ok((input, Statement::Continue))
}

fn parse_block(input: Input) -> IResult<Input, Block> {
    let (input, _) = expect_token(TokenKind::OpenBrace)(input)?;
    let (input, statements) = parse_block_content(input)?;
    let (input, _) = expect_token(TokenKind::CloseBrace)(input)?;
    Ok((
        input,
        Block {
            statements,
            unsafe_block: false,
        },
    ))
}

fn parse_statement(input: Input) -> IResult<Input, Statement> {
    alt((
        parse_const,
        parse_let,
        parse_if,
        parse_for,
        parse_break,
        parse_continue,
        map(parse_expression, Statement::Expr),
    ))(input)
}

pub fn parse_block_content(input: Input) -> IResult<Input, Vec<Statement>> {
    many0(parse_statement)(input)
}
