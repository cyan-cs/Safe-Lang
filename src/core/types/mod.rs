// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

mod list;
mod option;
mod print;
mod result;
mod string;

pub use list::{
    List, list_get_u8, list_is_empty, list_len, list_new, list_push_bytes, list_push_u8,
};
pub use option::{Option, option_is_some_u8, option_none_u8, option_some_u8, option_unwrap_u8};
pub use print::{Printable, format_printable, print, print_any, printl, printl_any};
pub use result::{
    Result, result_err_u8_i32, result_is_ok_u8_i32, result_ok_u8_i32, result_unwrap_err_u8_i32,
    result_unwrap_u8_i32,
};
pub use string::{
    String, StringList, StringSplit, string_append_bytes, string_clear, string_clear_with_capacity,
    string_clone, string_concat, string_contains, string_ends_with, string_eq, string_from_list,
    string_insert_bytes, string_is_empty, string_len, string_list_get, string_list_is_empty,
    string_list_len, string_new, string_pop, string_pop_n, string_push, string_push_bytes,
    string_push_str, string_remove, string_remove_range, string_replace, string_split_all,
    string_split_found, string_split_left, string_split_n, string_split_once, string_split_right,
    string_starts_with, string_substr, string_to_list, string_trim, string_trim_end,
    string_trim_start,
};
