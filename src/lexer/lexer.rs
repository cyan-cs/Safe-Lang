// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use nom::IResult;
use nom::Offset;
use nom::branch::alt;
use nom::character::complete::multispace0;

use super::parse::{keyword_or_identifier, literal, symbol};
use super::position::{build_line_starts, line_col_from_offset};
use super::token::{Span, Token};

fn lex_token<'a>(
    original_input: &'a str,
    line_starts: &'a [usize],
) -> impl FnMut(&str) -> IResult<&str, Token> + 'a {
    move |input: &str| {
        let (input, _) = multispace0(input)?;

        if input.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Eof,
            )));
        }

        let start_offset = original_input.offset(input);
        let (input, kind) = alt((symbol, literal, keyword_or_identifier))(input)?;
        let end_offset = original_input.offset(input);

        let (line, column) = line_col_from_offset(original_input, line_starts, start_offset);

        Ok((
            input,
            Token {
                kind,
                span: Span {
                    start: start_offset,
                    end: end_offset,
                    line,
                    column,
                },
            },
        ))
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut current_input = input;
    let line_starts = build_line_starts(input);
    let mut parse_next = lex_token(input, &line_starts);

    loop {
        current_input = skip_whitespace_and_comments(current_input).map_err(|near| {
            let offset = input.offset(near);
            let (line, column) = line_col_from_offset(input, &line_starts, offset);
            format!("Lexing error at line {line}, column {column} near: '{near}'")
        })?;

        if current_input.is_empty() {
            break;
        }

        match parse_next(current_input) {
            Ok((next_input, token)) => {
                tokens.push(token);
                current_input = next_input;
            }
            Err(nom::Err::Error(_)) | Err(nom::Err::Failure(_)) => {
                let offset = input.offset(current_input);
                let (line, column) = line_col_from_offset(input, &line_starts, offset);
                if let Some(detail) = detect_string_newline_error(current_input) {
                    return Err(format!(
                        "Lexing error at line {line}, column {column}: {detail}"
                    ));
                }
                return Err(format!(
                    "Lexing error at line {line}, column {column} near: '{current_input}'"
                ));
            }
            Err(nom::Err::Incomplete(_)) => break,
        }
    }
    Ok(tokens)
}

fn skip_whitespace_and_comments(mut input: &str) -> Result<&str, &str> {
    loop {
        let before = input;
        input = input.trim_start();

        if let Some(rest) = input.strip_prefix("//") {
            if let Some((idx, _)) = rest.char_indices().find(|(_, ch)| *ch == '\n') {
                input = &rest[idx + 1..];
            } else {
                input = "";
            }
            continue;
        }

        if let Some(rest) = input.strip_prefix("/*") {
            if let Some(idx) = rest.find("*/") {
                input = &rest[idx + 2..];
                continue;
            }
            return Err(input);
        }

        if input == before {
            break;
        }
    }
    Ok(input)
}

fn detect_string_newline_error(input: &str) -> Option<&'static str> {
    if !input.starts_with('"') {
        return None;
    }

    let mut escaped = false;
    for ch in input[1..].chars() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '"' => return None,
            '\n' | '\r' => {
                return Some(
                    "newline is not allowed in normal string literal; use \\n or raw string r#\"...\"#",
                );
            }
            _ => {}
        }
    }
    None
}
