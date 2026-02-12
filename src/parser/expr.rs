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
use nom::combinator::{map, map_res};
use nom::multi::separated_list0;

use super::helpers::{Input, expect_token, identifier, integer_literal, string_literal};
use super::stmt::parse_block_content;
use super::types::parse_type;

pub fn parse_expression(input: Input) -> IResult<Input, Expression> {
    parse_comparison(input)
}

fn parse_comparison(input: Input) -> IResult<Input, Expression> {
    let (mut input, mut expr) = parse_primary_expression(input)?;

    loop {
        let (next_input, op) =
            if let Ok((next_input, _)) = expect_token(TokenKind::EqualEqual)(input) {
                (next_input, BinaryOp::Equal)
            } else if let Ok((next_input, _)) = expect_token(TokenKind::NotEqual)(input) {
                (next_input, BinaryOp::NotEqual)
            } else if let Ok((next_input, _)) = expect_token(TokenKind::LessEqual)(input) {
                (next_input, BinaryOp::LessEqual)
            } else if let Ok((next_input, _)) = expect_token(TokenKind::GreaterEqual)(input) {
                (next_input, BinaryOp::GreaterEqual)
            } else if let Ok((next_input, _)) = expect_token(TokenKind::LessThan)(input) {
                (next_input, BinaryOp::LessThan)
            } else if let Ok((next_input, _)) = expect_token(TokenKind::GreaterThan)(input) {
                (next_input, BinaryOp::GreaterThan)
            } else {
                break;
            };

        let (after_rhs, rhs) = parse_primary_expression(next_input)?;
        expr = Expression::Binary {
            op,
            left: Box::new(expr),
            right: Box::new(rhs),
        };
        input = after_rhs;
    }

    Ok((input, expr))
}

fn parse_primary_expression(input: Input) -> IResult<Input, Expression> {
    alt((
        parse_ref_expr,
        parse_unsafe_block_expr,
        parse_call,
        parse_variable,
        parse_literal,
    ))(input)
}

fn parse_literal(input: Input) -> IResult<Input, Expression> {
    alt((
        map(expect_token(TokenKind::True), |_| {
            Expression::Literal(Literal::Bool(true))
        }),
        map(expect_token(TokenKind::False), |_| {
            Expression::Literal(Literal::Bool(false))
        }),
        map_res(integer_literal, |s| {
            s.parse::<i64>()
                .map(|val| Expression::Literal(Literal::Integer(val)))
        }),
        map(string_literal, |s| Expression::Literal(Literal::String(s))),
    ))(input)
}

fn parse_variable(input: Input) -> IResult<Input, Expression> {
    map(identifier, Expression::Variable)(input)
}

fn parse_call_args(input: Input) -> IResult<Input, Vec<Expression>> {
    separated_list0(expect_token(TokenKind::Comma), parse_expression)(input)
}

fn parse_call(input: Input) -> IResult<Input, Expression> {
    let (input, func_name) = identifier(input)?;
    let (input, _) = expect_token(TokenKind::OpenParen)(input)?;
    let (input, args) = parse_call_args(input)?;
    let (input, _) = expect_token(TokenKind::CloseParen)(input)?;
    Ok((input, Expression::Call(CallExpr { func_name, args })))
}

fn parse_unsafe_block_expr(input: Input) -> IResult<Input, Expression> {
    let (input, _) = expect_token(TokenKind::Unsafe)(input)?;
    let (input, _) = expect_token(TokenKind::OpenBrace)(input)?;
    let (input, statements) = parse_block_content(input)?;
    let (input, _) = expect_token(TokenKind::CloseBrace)(input)?;
    Ok((
        input,
        Expression::Block(Block {
            statements,
            unsafe_block: true,
        }),
    ))
}

fn parse_ref_expr(input: Input) -> IResult<Input, Expression> {
    let (input, _) = expect_token(TokenKind::Ampersand)(input)?;
    let (input, mutable) =
        if let Ok((input, _)) = expect_token(TokenKind::Identifier("mut".to_string()))(input) {
            (input, true)
        } else {
            (input, false)
        };
    let (input, expr) = parse_expression(input)?;
    Ok((
        input,
        Expression::Ref {
            mutable,
            expr: Box::new(expr),
        },
    ))
}

pub fn parse_arg(input: Input) -> IResult<Input, Arg> {
    let (input, name) = identifier(input)?;
    let (input, _) = expect_token(TokenKind::Colon)(input)?;
    let (input, ty) = parse_type(input)?;
    Ok((input, Arg { name, ty }))
}
