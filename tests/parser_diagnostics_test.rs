// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use safe_lang::lexer;
use safe_lang::parser;

#[test]
fn test_parse_with_diagnostics_reports_line_and_column() {
    let code = "safe fn test( {";
    let tokens = lexer::tokenize(code).expect("tokenize");

    let err = parser::parse_with_diagnostics(&tokens).expect_err("parse should fail");
    assert!(err.contains("line"));
    assert!(err.contains("column"));
}

#[test]
fn test_lexer_error_reports_line_and_column() {
    let code = "safe fn test() {\n    let high_x = @\n}";
    let err = lexer::tokenize(code).expect_err("lex should fail");

    assert!(err.contains("line"));
    assert!(err.contains("column"));
}

#[test]
fn test_integer_literal_overflow_is_error() {
    let code = "safe fn test() { let high_x = 9999999999999999999999999999 }";
    let tokens = lexer::tokenize(code).expect("tokenize");
    let err = parser::parse_with_diagnostics(&tokens).expect_err("parse should fail");
    assert!(err.contains("Parse error"));
}
