// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use safe_lang::ast::{Expression, Item, Statement};
use safe_lang::lexer;
use safe_lang::molding::Molder;
use safe_lang::parser;

fn run_molding_output(code: &str) -> Result<safe_lang::ast::SourceFile, String> {
    let tokens = lexer::tokenize(code).map_err(|e| format!("Lex: {e}"))?;
    let (rest, source) = parser::parse(&tokens).map_err(|_| "Parse failed".to_string())?;
    if !rest.is_empty() {
        return Err("Unconsumed tokens".to_string());
    }

    let mut molder = Molder::new(source);
    molder.mold()?;
    Ok(molder.get_output().clone())
}

fn run_molding(code: &str) -> Result<(), String> {
    let tokens = lexer::tokenize(code).map_err(|e| format!("Lex: {e}"))?;
    let (rest, source) = parser::parse(&tokens).map_err(|_| "Parse failed".to_string())?;
    if !rest.is_empty() {
        return Err("Unconsumed tokens".to_string());
    }

    let mut molder = Molder::new(source);
    molder.mold()
}

#[test]
fn test_molding_rejects_alias_cycle() {
    let code = r#"
alias a = b
alias b = a

safe fn test() {
    let high_x = a(1)
}
"#;

    let err = run_molding(code).expect_err("alias cycle should be rejected by molding");
    assert!(err.contains("Alias cycle detected"));
}

#[test]
fn test_molding_rejects_alias_target_with_unsafe_keyword() {
    let code = r#"
alias call_bridge = unsafe_bridge

safe fn test() {
    let high_x = call_bridge(1)
}
"#;

    let err = run_molding(code).expect_err("unsafe-containing alias target should fail");
    assert!(err.contains("cannot include 'unsafe'"));
}

#[test]
fn test_molding_wraps_raw_call_outside_unsafe() {
    let code = r#"
raw fn raw_helper() {
    let raw_tmp = 1
}

safe fn test() {
    let high_x = raw_helper()
}
"#;

    let source = run_molding_output(code).expect("raw call should be wrapped");
    let Item::Function(func) = &source.items[1] else {
        panic!("expected function item");
    };
    let Statement::Let(first_let) = &func.body.statements[0] else {
        panic!("expected let statement");
    };
    match &first_let.value {
        Expression::Block(block) => assert!(block.unsafe_block),
        _ => panic!("expected unsafe block wrapping raw call"),
    }
}

#[test]
fn test_molding_rejects_duplicate_names_across_scopes() {
    let code = r#"
safe fn test() {
    let high_x: i32 = 1
    unsafe {
        let raw_y = 2
    }
    let high_x: i32 = 3
}
"#;

    let err = run_molding(code).expect_err("global uniqueness should reject duplicate variable");
    assert!(err.contains("Rule 4 Violation"));
}

#[test]
fn test_molding_rejects_duplicate_names_with_different_types() {
    let code = r#"
safe fn test() {
    let high_x: i32 = 1
    let high_x: String = "ok"
}
"#;

    let err = run_molding(code).expect_err("duplicate name should be rejected regardless of type");
    assert!(err.contains("Rule 4 Violation"));
}

#[test]
fn test_molding_rejects_wrong_prefix_in_unsafe_block() {
    let code = r#"
safe fn test() {
    unsafe {
        let high_x = 1
    }
}
"#;

    let err = run_molding(code).expect_err("unsafe block variable prefix should be raw_");
    assert!(err.contains("Rule 6 Violation"));
}

#[test]
fn test_molding_requires_validated_for_into_high() {
    let code = r#"
safe fn test() {
    unsafe {
        let high_x = into_high(raw_x)
    }
}
"#;

    let err = run_molding(code).expect_err("into_high requires validated_ var");
    assert!(err.contains("validated_"));
}

#[test]
fn test_molding_allows_into_high_with_validated() {
    let code = r#"
safe fn test() {
    unsafe {
        let validated_x = validate_raw(raw_x)
        let high_x = into_high(validated_x)
    }
}
"#;

    assert!(run_molding(code).is_ok());
}

#[test]
fn test_molding_normalizes_builtin_calls_to_fully_qualified_names() {
    let code = r#"
safe fn test() {
    let high_ptr = allocate_buffer(1)
    unsafe {
        let raw_ptr = raw_alloc(1)
        let raw_res = raw_write(raw_ptr, 0, 1)
    }
}
"#;

    let source = run_molding_output(code).expect("molding should succeed");
    let Item::Function(func) = &source.items[0] else {
        panic!("expected function item");
    };

    let Statement::Let(first_let) = &func.body.statements[0] else {
        panic!("expected first let statement");
    };
    let Expression::Call(first_call) = &first_let.value else {
        panic!("expected first statement call");
    };
    assert_eq!(first_call.func_name, "core::memory::safe::allocate_buffer");

    let Statement::Expr(Expression::Block(unsafe_block)) = &func.body.statements[1] else {
        panic!("expected unsafe block expression");
    };
    let Statement::Let(raw_let) = &unsafe_block.statements[0] else {
        panic!("expected let in unsafe block");
    };
    let Expression::Call(raw_call) = &raw_let.value else {
        panic!("expected raw call");
    };
    assert_eq!(raw_call.func_name, "core::memory::raw::alloc");

    let Statement::Let(raw_write_let) = &unsafe_block.statements[1] else {
        panic!("expected second let in unsafe block");
    };
    let Expression::Call(raw_write_call) = &raw_write_let.value else {
        panic!("expected raw write call");
    };
    assert_eq!(raw_write_call.func_name, "core::memory::raw::write");
}
