// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use super::{TokenKind, tokenize};

#[test]
fn test_lexer_simple() {
    let input = "safe fn main() { let x = 10 }";
    let tokens = tokenize(input).expect("Lexing failed");

    assert_eq!(tokens.len(), 11);
    assert!(matches!(tokens[0].kind, TokenKind::Safe));
    assert!(matches!(tokens[1].kind, TokenKind::Fn));
    assert!(matches!(tokens[2].kind, TokenKind::Identifier(ref s) if s == "main"));
    assert!(matches!(tokens[3].kind, TokenKind::OpenParen));
    assert!(matches!(tokens[4].kind, TokenKind::CloseParen));
    assert!(matches!(tokens[5].kind, TokenKind::OpenBrace));
    assert!(matches!(tokens[6].kind, TokenKind::Let));
    assert!(matches!(tokens[7].kind, TokenKind::Identifier(_)));
    // ...
}

#[test]
fn test_span() {
    let input = " \n let x";
    let tokens = tokenize(input).expect("Lexing failed");
    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert_eq!(tokens[0].span.line, 2);
}

#[test]
fn test_line_comment_is_ignored() {
    let input = "let high_x = 1 // comment\nlet high_y = 2";
    let tokens = tokenize(input).expect("Lexing failed");
    assert!(
        tokens
            .iter()
            .all(|t| !matches!(t.kind, TokenKind::Identifier(ref s) if s == "comment"))
    );
    assert!(
        tokens
            .iter()
            .any(|t| matches!(t.kind, TokenKind::Identifier(ref s) if s == "high_y"))
    );
}

#[test]
fn test_block_comment_is_ignored() {
    let input = "let high_x = 1 /* multi\nline */ let high_y = 2";
    let tokens = tokenize(input).expect("Lexing failed");
    assert!(
        tokens
            .iter()
            .any(|t| matches!(t.kind, TokenKind::Identifier(ref s) if s == "high_y"))
    );
}

#[test]
fn test_new_keywords() {
    let input = "const if else for in break continue true false";
    let tokens = tokenize(input).expect("Lexing failed");
    assert!(matches!(tokens[0].kind, TokenKind::Const));
    assert!(matches!(tokens[1].kind, TokenKind::If));
    assert!(matches!(tokens[2].kind, TokenKind::Else));
    assert!(matches!(tokens[3].kind, TokenKind::For));
    assert!(matches!(tokens[4].kind, TokenKind::In));
    assert!(matches!(tokens[5].kind, TokenKind::Break));
    assert!(matches!(tokens[6].kind, TokenKind::Continue));
    assert!(matches!(tokens[7].kind, TokenKind::True));
    assert!(matches!(tokens[8].kind, TokenKind::False));
}

#[test]
fn test_string_literal_disallows_newline() {
    let input = "let high_x = \"hello\nworld\"";
    let err = tokenize(input).expect_err("lexing should fail");
    assert!(err.contains("newline is not allowed"));
}

#[test]
fn test_raw_string_literal_allows_multiline() {
    let input = "let high_x = r#\"hello\nworld\"#";
    let tokens = tokenize(input).expect("Lexing failed");
    assert!(
        tokens
            .iter()
            .any(|t| matches!(t.kind, TokenKind::StringLiteral(ref s) if s == "hello\nworld"))
    );
}

#[test]
fn test_raw_string_literal_with_extra_hashes() {
    let input = "let high_x = r###\"line1\n\"# inside\"\nline2\"###";
    let tokens = tokenize(input).expect("Lexing failed");
    assert!(tokens.iter().any(|t| matches!(
        t.kind,
        TokenKind::StringLiteral(ref s) if s == "line1\n\"# inside\"\nline2"
    )));
}
