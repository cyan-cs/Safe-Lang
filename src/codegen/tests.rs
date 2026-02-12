// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use super::CodeGenerator;
use crate::{lexer, molding::Molder, parser, type_checker::TypeChecker};

#[test]
fn test_generate_simple_function() {
    let code = r#"
safe fn test() {
    let high_x = 42
}
"#;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut molder = Molder::new(source);
    molder.mold().expect("mold");

    let mut checker = TypeChecker::new();
    checker.check(molder.get_output()).expect("type check");

    let mut generator = CodeGenerator::new();
    let rust_code = generator.generate(molder.get_output()).expect("codegen");

    assert!(rust_code.contains("pub fn test()"));
    assert!(rust_code.contains("let high_x = 42;"));
}

#[test]
fn test_codegen_expands_alias_in_calls() {
    let code = r#"
alias h_alloc = allocate_buffer

safe fn process() {
    let high_ptr = h_alloc(1)
}
"#;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut generator = CodeGenerator::new();
    let rust_code = generator.generate(&source).expect("codegen");

    assert!(rust_code.contains("allocate_buffer(1)"));
    assert!(!rust_code.contains("h_alloc(1)"));
}

#[test]
fn test_codegen_expands_recursive_alias_chain() {
    let code = r#"
alias a = b
alias b = allocate_buffer

safe fn process() {
    let high_ptr = a(1)
}
"#;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut generator = CodeGenerator::new();
    let rust_code = generator.generate(&source).expect("codegen");

    assert!(rust_code.contains("allocate_buffer(1)"));
    assert!(!rust_code.contains("a(1)"));
    assert!(!rust_code.contains("b(1)"));
}

#[test]
fn test_codegen_reports_alias_cycles() {
    let code = r#"
alias a = b
alias b = a

safe fn process() {
    let high_ptr = a(1)
}
"#;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut generator = CodeGenerator::new();
    let err = generator
        .generate(&source)
        .expect_err("alias cycle should fail");

    assert!(err.contains("Alias cycle detected"));
}

#[test]
fn test_codegen_reports_unknown_function_calls() {
    let code = r#"
safe fn process() {
    let high_ptr = definitely_unknown(1)
}
"#;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut generator = CodeGenerator::new();
    let err = generator
        .generate(&source)
        .expect_err("unknown call should fail");

    assert!(err.contains("Unknown function 'definitely_unknown'"));
}

#[test]
fn test_generate_tail_expression_without_semicolon_for_returning_function() {
    let code = r#"
safe fn val() -> i32 {
    42
}
"#;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut molder = Molder::new(source);
    molder.mold().expect("mold");

    let mut checker = TypeChecker::new();
    checker.check(molder.get_output()).expect("type check");

    let mut generator = CodeGenerator::new();
    let rust_code = generator.generate(molder.get_output()).expect("codegen");

    assert!(rust_code.contains("pub fn val() -> i32"));
    assert!(rust_code.contains("\n    42\n}"));
    assert!(!rust_code.contains("\n    42;\n}"));
}

#[test]
fn test_block_tail_expression_has_no_semicolon() {
    let code = r#"
safe fn val() -> i32 {
    unsafe {
        42
    }
}
"#;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut molder = Molder::new(source);
    molder.mold().expect("mold");

    let mut checker = TypeChecker::new();
    checker.check(molder.get_output()).expect("type check");

    let mut generator = CodeGenerator::new();
    let rust_code = generator.generate(molder.get_output()).expect("codegen");

    assert!(rust_code.contains("unsafe {\n        42\n    }"));
    assert!(!rust_code.contains("unsafe {\n        42;\n    }"));
}

#[test]
fn test_codegen_accepts_fully_qualified_builtin_calls() {
    let code = r#"
safe fn process() {
    let high_ptr = allocate_buffer(1)
    unsafe {
        let raw_ptr = raw_alloc(1)
        let raw_res = raw_write(raw_ptr, 0, 1)
    }
}
"#;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut molder = Molder::new(source);
    molder.mold().expect("mold");

    let mut checker = TypeChecker::new();
    checker.check(molder.get_output()).expect("type check");

    let mut generator = CodeGenerator::new();
    let rust_code = generator.generate(molder.get_output()).expect("codegen");

    assert!(rust_code.contains("core::memory::safe::allocate_buffer(1)"));
    assert!(rust_code.contains("core::memory::raw::alloc(1)"));
    assert!(rust_code.contains("core::memory::raw::write(raw_ptr, 0, 1)"));
}

#[test]
fn test_codegen_escapes_string_literals() {
    let code = r#"
safe fn test() {
    let high_s = "a\\b"
}
"#;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut generator = CodeGenerator::new();
    let rust_code = generator.generate(&source).expect("codegen");

    assert!(rust_code.contains("let high_s = safe_lang::core::types::String::from(\"a\\\\b\");"));
}

#[test]
fn test_codegen_reference_expression() {
    let code = r#"
safe fn len(high_s: &String) -> usize {
    string_len(high_s)
}
"#;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut molder = Molder::new(source);
    molder.mold().expect("mold");

    let mut checker = TypeChecker::new();
    checker.check(molder.get_output()).expect("type check");

    let mut generator = CodeGenerator::new();
    let rust_code = generator.generate(molder.get_output()).expect("codegen");

    assert!(rust_code.contains("pub fn len(high_s: &safe_lang::core::types::String) -> usize"));
    assert!(rust_code.contains("string_len(high_s)"));
}

#[test]
fn test_codegen_if_for_break_continue() {
    let code = r#"
safe fn test() {
    const high_flag: bool = true
    if high_flag {
        for high_i in 0..=3 {
            if high_i == 2 {
                continue
            } else {
                break
            }
        }
    } else {
        let high_buf = allocate_buffer(1)
        deallocate_buffer(high_buf)
    }
}
"#;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut molder = Molder::new(source);
    molder.mold().expect("mold");

    let mut checker = TypeChecker::new();
    checker.check(molder.get_output()).expect("type check");

    let mut generator = CodeGenerator::new();
    let rust_code = generator.generate(molder.get_output()).expect("codegen");

    assert!(rust_code.contains("let high_flag: bool = true;"));
    assert!(rust_code.contains("if high_flag {"));
    assert!(rust_code.contains("for high_i in 0..=3 {"));
    assert!(rust_code.contains("continue;"));
    assert!(rust_code.contains("break;"));
}

#[test]
fn test_codegen_print_variadic_and_printl() {
    let code = r##"
safe fn test() {
    print("a", 1, false)
    printl("b", 2)
    printl()
}
"##;
    let tokens = lexer::tokenize(code).expect("tokenize");
    let (rest, source) = parser::parse(&tokens).expect("parse");
    assert!(rest.is_empty());

    let mut molder = Molder::new(source);
    molder.mold().expect("mold");

    let mut checker = TypeChecker::new();
    checker.check(molder.get_output()).expect("type check");

    let mut generator = CodeGenerator::new();
    let rust_code = generator.generate(molder.get_output()).expect("codegen");

    assert!(rust_code.contains("safe_lang::core::types::print_any(&("));
    assert!(rust_code.contains("std::println!();"));
}
