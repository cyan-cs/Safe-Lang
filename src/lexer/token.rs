// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Fn,
    Let,
    Const,
    Safe,
    Raw,
    Unsafe,
    Alias,
    If,
    Else,
    For,
    In,
    Break,
    Continue,
    True,
    False,

    // Symbols
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    Colon,        // :
    Equal,        // =
    Arrow,        // ->
    Comma,        // ,
    Ampersand,    // &
    Star,         // *
    DotDot,       // ..
    DotDotEqual,  // ..=

    // Comparison & Generics
    LessThan,     // <
    GreaterThan,  // >
    LessEqual,    // <=
    GreaterEqual, // >=
    NotEqual,     // !=
    EqualEqual,   // ==

    // Literals & Identifiers
    Identifier(String),
    Integer(String),
    StringLiteral(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
