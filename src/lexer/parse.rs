// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::{map, recognize};
use nom::multi::{many0, many1};
use nom::sequence::pair;

use super::token::TokenKind;

pub fn symbol(input: &str) -> IResult<&str, TokenKind> {
    alt((
        map(tag("..="), |_| TokenKind::DotDotEqual),
        map(tag(".."), |_| TokenKind::DotDot),
        map(tag("->"), |_| TokenKind::Arrow),
        map(tag("<="), |_| TokenKind::LessEqual),
        map(tag(">="), |_| TokenKind::GreaterEqual),
        map(tag("!="), |_| TokenKind::NotEqual),
        map(tag("=="), |_| TokenKind::EqualEqual),
        map(tag("<"), |_| TokenKind::LessThan),
        map(tag(">"), |_| TokenKind::GreaterThan),
        map(tag("("), |_| TokenKind::OpenParen),
        map(tag(")"), |_| TokenKind::CloseParen),
        map(tag("{"), |_| TokenKind::OpenBrace),
        map(tag("}"), |_| TokenKind::CloseBrace),
        map(tag("["), |_| TokenKind::OpenBracket),
        map(tag("]"), |_| TokenKind::CloseBracket),
        map(tag(":"), |_| TokenKind::Colon),
        map(tag("="), |_| TokenKind::Equal),
        map(tag(","), |_| TokenKind::Comma),
        map(tag("&"), |_| TokenKind::Ampersand),
        map(tag("*"), |_| TokenKind::Star),
    ))(input)
}

pub fn keyword_or_identifier(input: &str) -> IResult<&str, TokenKind> {
    let (input, name) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)?;

    let kind = match name {
        "fn" => TokenKind::Fn,
        "let" => TokenKind::Let,
        "const" => TokenKind::Const,
        "safe" => TokenKind::Safe,
        "raw" => TokenKind::Raw,
        "unsafe" => TokenKind::Unsafe,
        "alias" => TokenKind::Alias,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "for" => TokenKind::For,
        "in" => TokenKind::In,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        _ => TokenKind::Identifier(name.to_string()),
    };
    Ok((input, kind))
}

pub fn literal(input: &str) -> IResult<&str, TokenKind> {
    alt((
        map(parse_raw_string_literal, TokenKind::StringLiteral),
        map(
            recognize(many1(take_while1(|c: char| c.is_ascii_digit()))),
            |s: &str| TokenKind::Integer(s.to_string()),
        ),
        map(parse_string_literal, TokenKind::StringLiteral),
    ))(input)
}

pub fn parse_string_literal(input: &str) -> IResult<&str, String> {
    if !input.starts_with('"') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }

    let mut out = String::new();
    let mut idx = 1;
    while idx < input.len() {
        let ch = input[idx..].chars().next().unwrap();
        match ch {
            '"' => {
                let rest = &input[idx + 1..];
                return Ok((rest, out));
            }
            '\n' | '\r' => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    &input[idx..],
                    nom::error::ErrorKind::Escaped,
                )));
            }
            '\\' => {
                let next_idx = idx + ch.len_utf8();
                let esc = input[next_idx..].chars().next().ok_or_else(|| {
                    nom::Err::Error(nom::error::Error::new(
                        &input[idx..],
                        nom::error::ErrorKind::Escaped,
                    ))
                })?;
                match esc {
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    '0' => out.push('\0'),
                    _ => {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            &input[idx..],
                            nom::error::ErrorKind::Escaped,
                        )));
                    }
                }
                idx = next_idx + esc.len_utf8();
            }
            _ => {
                out.push(ch);
                idx += ch.len_utf8();
            }
        }
    }

    Err(nom::Err::Error(nom::error::Error::new(
        &input[idx..],
        nom::error::ErrorKind::Eof,
    )))
}

pub fn parse_raw_string_literal(input: &str) -> IResult<&str, String> {
    let Some(rest) = input.strip_prefix('r') else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };

    let hash_count = rest.chars().take_while(|ch| *ch == '#').count();
    let after_hashes = &rest[hash_count..];
    let Some(content_start) = after_hashes.strip_prefix('"') else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };

    let closing = format!("\"{}", "#".repeat(hash_count));
    let Some(end) = content_start.find(&closing) else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    };

    let content = &content_start[..end];
    let remain = &content_start[end + closing.len()..];
    Ok((remain, content.to_string()))
}
