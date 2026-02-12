// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use safe_lang::lexer;
use safe_lang::molding::Molder;
use safe_lang::parser;
use safe_lang::type_checker::TypeChecker;

fn run_pipeline(code: &str) -> Result<(), String> {
    let tokens = lexer::tokenize(code).map_err(|e| format!("Lex: {e}"))?;
    let (rest, source) = parser::parse(&tokens).map_err(|_| "Parse failed".to_string())?;

    if !rest.is_empty() {
        return Err("Unconsumed tokens".to_string());
    }

    let mut molder = Molder::new(source);
    molder.mold().map_err(|e| format!("Mold: {e}"))?;

    let mut checker = TypeChecker::new();
    checker
        .check(molder.get_output())
        .map_err(|e| format!("Type: {e}"))?;

    Ok(())
}

#[test]
fn test_valid_function_with_type_annotation() {
    let code = r#"
safe fn test() {
    let high_x: i32 = 42
}
"#;
    assert!(run_pipeline(code).is_ok());
}

#[test]
fn test_type_mismatch_int_string() {
    let code = r#"
safe fn test() {
    let high_x: i32 = "hello"
}
"#;
    assert!(run_pipeline(code).is_err());
}

#[test]
fn test_numeric_literal_coercion() {
    let code = r#"
safe fn test() {
    let high_x = allocate_buffer(42)
}
"#;
    assert!(run_pipeline(code).is_ok());
}

#[test]
fn test_nested_unsafe_block() {
    let code = r#"
safe fn test() {
    unsafe {
        let raw_x = 42
        unsafe {
            let raw_y = 100
        }
    }
}
"#;
    assert!(run_pipeline(code).is_ok());
}

#[test]
fn test_undefined_variable() {
    let code = r#"
safe fn test() {
    let high_x = high_undefined_var
}
"#;
    assert!(run_pipeline(code).is_err());
}

#[test]
fn test_return_type_mismatch_is_error() {
    let code = r#"
safe fn test() -> i32 {
    "hello"
}
"#;
    assert!(run_pipeline(code).is_err());
}

#[test]
fn test_return_integer_literal_to_compatible_int_type() {
    let code = r#"
safe fn test() -> u64 {
    42
}
"#;
    assert!(run_pipeline(code).is_ok());
}

#[test]
fn test_redefining_builtin_is_error() {
    let code = r#"
safe fn allocate_buffer(high_size: usize) {
}
"#;
    let err = run_pipeline(code).expect_err("builtin redefinition should fail");
    assert!(err.contains("Builtin function"));
}

#[test]
fn test_raw_pointer_type_parses() {
    let code = r#"
raw fn test(raw_p: *i32) {
}
"#;
    assert!(run_pipeline(code).is_ok());
}

#[test]
fn test_option_u8_builtin_flow() {
    let code = r#"
safe fn test() {
    let high_opt: Option<u8> = option_some_u8(7)
    let high_has = option_is_some_u8(high_opt)
}
"#;
    assert!(run_pipeline(code).is_ok());
}

#[test]
fn test_result_u8_i32_builtin_flow() {
    let code = r#"
safe fn test() {
    let high_res: Result<u8, i32> = result_ok_u8_i32(7)
    let high_ok = result_is_ok_u8_i32(high_res)
}
"#;
    assert!(run_pipeline(code).is_ok());
}

#[test]
fn test_unknown_type_is_error() {
    let code = r#"
safe fn test(high_v: MysteryType) {
}
"#;
    let err = run_pipeline(code).expect_err("unknown type should fail");
    assert!(err.contains("Unknown type"));
}

#[test]
fn test_non_option_result_generic_is_error() {
    let code = r#"
safe fn test() {
    let high_x: List<u8> = 1
}
"#;
    let err = run_pipeline(code).expect_err("unsupported generic should fail");
    assert!(err.contains("Option/Result"));
}

#[test]
fn test_if_else_and_const() {
    let code = r#"
safe fn test() {
    const high_flag: bool = true
    if high_flag {
        let high_buf_left = allocate_buffer(1)
        deallocate_buffer(high_buf_left)
    } else {
        let high_buf_right = allocate_buffer(1)
        deallocate_buffer(high_buf_right)
    }
}
"#;
    assert!(run_pipeline(code).is_ok());
}

#[test]
fn test_for_break_continue() {
    let code = r#"
safe fn test() {
    for high_i in 0..3 {
        if high_i == 1 {
            continue
        } else {
            break
        }
    }
}
"#;
    assert!(run_pipeline(code).is_ok());
}

#[test]
fn test_break_outside_loop_is_error() {
    let code = r#"
safe fn test() {
    break
}
"#;
    let err = run_pipeline(code).expect_err("break outside loop should fail");
    assert!(err.contains("break/continue"));
}

#[test]
fn test_break_inside_non_loop_block_is_error() {
    let code = r#"
safe fn test() {
    unsafe {
        break
    }
}
"#;
    let err = run_pipeline(code).expect_err("break in non-loop block should fail");
    assert!(err.contains("break/continue"));
}

#[test]
fn test_continue_inside_non_loop_block_is_error() {
    let code = r#"
safe fn test() {
    unsafe {
        continue
    }
}
"#;
    let err = run_pipeline(code).expect_err("continue in non-loop block should fail");
    assert!(err.contains("break/continue"));
}

#[test]
fn test_comments_are_ignored() {
    let code = r#"
safe fn test() {
    // single line comment
    /* block
       comment */
    let high_x = allocate_buffer(1)
    deallocate_buffer(high_x)
}
"#;
    assert!(run_pipeline(code).is_ok());
}

#[test]
fn test_print_and_printl() {
    let code = r##"
safe fn test() {
    print("hello", " ", 42, " ", false)
    printl("world\nnext", 123, true)
    printl(r#"raw
line"#)
    printl()
}
"##;
    assert!(run_pipeline(code).is_ok());
}

#[test]
fn test_print_rejects_unsupported_type() {
    let code = r#"
safe fn test() {
    let high_list = list_new()
    print(high_list)
}
"#;
    let err = run_pipeline(code).expect_err("print should reject non-printable type");
    assert!(err.contains("print/printl does not support type"));
}

#[test]
fn test_for_body_in_unsafe_block_is_type_checked() {
    let code = r#"
safe fn test() {
    unsafe {
        for raw_i in 0..1 {
            let raw_x = raw_missing
        }
    }
}
"#;
    let err = run_pipeline(code).expect_err("for body should be type checked");
    assert!(err.contains("Undefined variable"));
}

#[test]
fn test_for_loop_var_type_follows_non_literal_bound_type() {
    let code = r#"
safe fn test() {
    let high_s = "abc"
    let high_end = string_len(&high_s)
    for high_i in 0..high_end {
        let high_copy: usize = high_i
    }
}
"#;
    assert!(run_pipeline(code).is_ok());
}
