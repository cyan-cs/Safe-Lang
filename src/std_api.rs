// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::ast::Type;

#[derive(Debug, Clone, Copy)]
pub struct ApiFunction {
    pub name: &'static str,
    pub canonical: &'static str,
    pub args: &'static [&'static str],
    pub ret: Option<&'static str>,
}

const API_FUNCTIONS: &[ApiFunction] = &[
    ApiFunction {
        name: "allocate_buffer",
        canonical: "core::memory::safe::allocate_buffer",
        args: &["usize"],
        ret: Some("core::memory::safe::HighPtr"),
    },
    ApiFunction {
        name: "deallocate_buffer",
        canonical: "core::memory::safe::deallocate_buffer",
        args: &["core::memory::safe::HighPtr"],
        ret: None,
    },
    ApiFunction {
        name: "raw_alloc",
        canonical: "core::memory::raw::alloc",
        args: &["usize"],
        ret: Some("core::memory::raw::RawPtr"),
    },
    ApiFunction {
        name: "raw_deallocate",
        canonical: "core::memory::raw::deallocate",
        args: &["core::memory::raw::RawPtr"],
        ret: None,
    },
    ApiFunction {
        name: "raw_write",
        canonical: "core::memory::raw::write",
        args: &["core::memory::raw::RawPtr", "usize", "u8"],
        ret: None,
    },
    ApiFunction {
        name: "raw_read",
        canonical: "core::memory::raw::read",
        args: &["core::memory::raw::RawPtr", "usize"],
        ret: Some("u8"),
    },
    ApiFunction {
        name: "validate_raw",
        canonical: "core::memory::safe::validate_raw",
        args: &["core::memory::raw::RawPtr"],
        ret: Some("core::memory::safe::ValidatedPtr"),
    },
    ApiFunction {
        name: "into_high",
        canonical: "core::memory::safe::into_high",
        args: &["core::memory::safe::ValidatedPtr"],
        ret: Some("core::memory::safe::HighPtr"),
    },
    ApiFunction {
        name: "option_some_u8",
        canonical: "core::types::option_some_u8",
        args: &["u8"],
        ret: Some("core::types::Option<u8>"),
    },
    ApiFunction {
        name: "option_none_u8",
        canonical: "core::types::option_none_u8",
        args: &[],
        ret: Some("core::types::Option<u8>"),
    },
    ApiFunction {
        name: "option_is_some_u8",
        canonical: "core::types::option_is_some_u8",
        args: &["core::types::Option<u8>"],
        ret: Some("bool"),
    },
    ApiFunction {
        name: "option_unwrap_u8",
        canonical: "core::types::option_unwrap_u8",
        args: &["core::types::Option<u8>"],
        ret: Some("u8"),
    },
    ApiFunction {
        name: "result_ok_u8_i32",
        canonical: "core::types::result_ok_u8_i32",
        args: &["u8"],
        ret: Some("core::types::Result<u8, i32>"),
    },
    ApiFunction {
        name: "result_err_u8_i32",
        canonical: "core::types::result_err_u8_i32",
        args: &["i32"],
        ret: Some("core::types::Result<u8, i32>"),
    },
    ApiFunction {
        name: "result_is_ok_u8_i32",
        canonical: "core::types::result_is_ok_u8_i32",
        args: &["core::types::Result<u8, i32>"],
        ret: Some("bool"),
    },
    ApiFunction {
        name: "result_unwrap_u8_i32",
        canonical: "core::types::result_unwrap_u8_i32",
        args: &["core::types::Result<u8, i32>"],
        ret: Some("u8"),
    },
    ApiFunction {
        name: "result_unwrap_err_u8_i32",
        canonical: "core::types::result_unwrap_err_u8_i32",
        args: &["core::types::Result<u8, i32>"],
        ret: Some("i32"),
    },
    ApiFunction {
        name: "string_new",
        canonical: "core::types::string_new",
        args: &[],
        ret: Some("core::types::String"),
    },
    ApiFunction {
        name: "string_clone",
        canonical: "core::types::string_clone",
        args: &["&core::types::String"],
        ret: Some("core::types::String"),
    },
    ApiFunction {
        name: "string_len",
        canonical: "core::types::string_len",
        args: &["&core::types::String"],
        ret: Some("usize"),
    },
    ApiFunction {
        name: "string_is_empty",
        canonical: "core::types::string_is_empty",
        args: &["&core::types::String"],
        ret: Some("bool"),
    },
    ApiFunction {
        name: "string_concat",
        canonical: "core::types::string_concat",
        args: &["&core::types::String", "&core::types::String"],
        ret: Some("core::types::String"),
    },
    ApiFunction {
        name: "string_eq",
        canonical: "core::types::string_eq",
        args: &["&core::types::String", "&core::types::String"],
        ret: Some("bool"),
    },
    ApiFunction {
        name: "string_substr",
        canonical: "core::types::string_substr",
        args: &["&core::types::String", "usize", "usize"],
        ret: Some("core::types::String"),
    },
    ApiFunction {
        name: "string_starts_with",
        canonical: "core::types::string_starts_with",
        args: &["&core::types::String", "&core::types::String"],
        ret: Some("bool"),
    },
    ApiFunction {
        name: "string_ends_with",
        canonical: "core::types::string_ends_with",
        args: &["&core::types::String", "&core::types::String"],
        ret: Some("bool"),
    },
    ApiFunction {
        name: "string_contains",
        canonical: "core::types::string_contains",
        args: &["&core::types::String", "&core::types::String"],
        ret: Some("bool"),
    },
    ApiFunction {
        name: "string_push",
        canonical: "core::types::string_push",
        args: &["&mut core::types::String", "u8"],
        ret: None,
    },
    ApiFunction {
        name: "string_push_bytes",
        canonical: "core::types::string_push_bytes",
        args: &["&mut core::types::String", "&core::types::List"],
        ret: None,
    },
    ApiFunction {
        name: "string_push_str",
        canonical: "core::types::string_push_str",
        args: &["&mut core::types::String", "&core::types::String"],
        ret: None,
    },
    ApiFunction {
        name: "string_clear",
        canonical: "core::types::string_clear",
        args: &["&mut core::types::String"],
        ret: None,
    },
    ApiFunction {
        name: "string_clear_with_capacity",
        canonical: "core::types::string_clear_with_capacity",
        args: &["&mut core::types::String"],
        ret: None,
    },
    ApiFunction {
        name: "string_append_bytes",
        canonical: "core::types::string_append_bytes",
        args: &["&mut core::types::String", "&core::types::List"],
        ret: None,
    },
    ApiFunction {
        name: "string_pop",
        canonical: "core::types::string_pop",
        args: &["&mut core::types::String"],
        ret: Some("core::types::Option<u8>"),
    },
    ApiFunction {
        name: "string_pop_n",
        canonical: "core::types::string_pop_n",
        args: &["&mut core::types::String", "usize"],
        ret: Some("core::types::List"),
    },
    ApiFunction {
        name: "string_remove",
        canonical: "core::types::string_remove",
        args: &["&mut core::types::String", "usize"],
        ret: Some("core::types::Option<u8>"),
    },
    ApiFunction {
        name: "string_remove_range",
        canonical: "core::types::string_remove_range",
        args: &["&mut core::types::String", "usize", "usize"],
        ret: Some("core::types::List"),
    },
    ApiFunction {
        name: "string_insert_bytes",
        canonical: "core::types::string_insert_bytes",
        args: &["&mut core::types::String", "usize", "&core::types::List"],
        ret: None,
    },
    ApiFunction {
        name: "string_replace",
        canonical: "core::types::string_replace",
        args: &[
            "&core::types::String",
            "&core::types::String",
            "&core::types::String",
        ],
        ret: Some("core::types::String"),
    },
    ApiFunction {
        name: "string_trim",
        canonical: "core::types::string_trim",
        args: &["&core::types::String"],
        ret: Some("core::types::String"),
    },
    ApiFunction {
        name: "string_trim_start",
        canonical: "core::types::string_trim_start",
        args: &["&core::types::String"],
        ret: Some("core::types::String"),
    },
    ApiFunction {
        name: "string_trim_end",
        canonical: "core::types::string_trim_end",
        args: &["&core::types::String"],
        ret: Some("core::types::String"),
    },
    ApiFunction {
        name: "string_split_once",
        canonical: "core::types::string_split_once",
        args: &["&core::types::String", "&core::types::String"],
        ret: Some("core::types::StringSplit"),
    },
    ApiFunction {
        name: "string_split_all",
        canonical: "core::types::string_split_all",
        args: &["&core::types::String", "&core::types::String"],
        ret: Some("core::types::StringList"),
    },
    ApiFunction {
        name: "string_split_n",
        canonical: "core::types::string_split_n",
        args: &["&core::types::String", "&core::types::String", "usize"],
        ret: Some("core::types::StringList"),
    },
    ApiFunction {
        name: "string_split_found",
        canonical: "core::types::string_split_found",
        args: &["&core::types::StringSplit"],
        ret: Some("bool"),
    },
    ApiFunction {
        name: "string_split_left",
        canonical: "core::types::string_split_left",
        args: &["&core::types::StringSplit"],
        ret: Some("core::types::String"),
    },
    ApiFunction {
        name: "string_split_right",
        canonical: "core::types::string_split_right",
        args: &["&core::types::StringSplit"],
        ret: Some("core::types::String"),
    },
    ApiFunction {
        name: "string_list_len",
        canonical: "core::types::string_list_len",
        args: &["&core::types::StringList"],
        ret: Some("usize"),
    },
    ApiFunction {
        name: "string_list_is_empty",
        canonical: "core::types::string_list_is_empty",
        args: &["&core::types::StringList"],
        ret: Some("bool"),
    },
    ApiFunction {
        name: "string_list_get",
        canonical: "core::types::string_list_get",
        args: &["&core::types::StringList", "usize"],
        ret: Some("core::types::Option<core::types::String>"),
    },
    ApiFunction {
        name: "string_from_list",
        canonical: "core::types::string_from_list",
        args: &["&core::types::List"],
        ret: Some("core::types::String"),
    },
    ApiFunction {
        name: "string_to_list",
        canonical: "core::types::string_to_list",
        args: &["&core::types::String"],
        ret: Some("core::types::List"),
    },
    ApiFunction {
        name: "list_new",
        canonical: "core::types::list_new",
        args: &[],
        ret: Some("core::types::List"),
    },
    ApiFunction {
        name: "list_len",
        canonical: "core::types::list_len",
        args: &["&core::types::List"],
        ret: Some("usize"),
    },
    ApiFunction {
        name: "list_is_empty",
        canonical: "core::types::list_is_empty",
        args: &["&core::types::List"],
        ret: Some("bool"),
    },
    ApiFunction {
        name: "list_push_u8",
        canonical: "core::types::list_push_u8",
        args: &["&mut core::types::List", "u8"],
        ret: None,
    },
    ApiFunction {
        name: "list_get_u8",
        canonical: "core::types::list_get_u8",
        args: &["&core::types::List", "usize"],
        ret: Some("core::types::Option<u8>"),
    },
    ApiFunction {
        name: "list_push_bytes",
        canonical: "core::types::list_push_bytes",
        args: &["&mut core::types::List", "&core::types::List"],
        ret: None,
    },
];

const API_TYPES: &[(&str, &str)] = &[
    ("String", "core::types::String"),
    ("StringSplit", "core::types::StringSplit"),
    ("StringList", "core::types::StringList"),
    ("List", "core::types::List"),
    ("Option", "core::types::Option"),
    ("Result", "core::types::Result"),
    ("HighPtr", "core::memory::safe::HighPtr"),
    ("ValidatedPtr", "core::memory::safe::ValidatedPtr"),
    ("RawPtr", "core::memory::raw::RawPtr"),
    ("core::types::String", "core::types::String"),
    ("core::types::StringSplit", "core::types::StringSplit"),
    ("core::types::StringList", "core::types::StringList"),
    ("core::types::List", "core::types::List"),
    ("core::memory::safe::HighPtr", "core::memory::safe::HighPtr"),
    (
        "core::memory::safe::ValidatedPtr",
        "core::memory::safe::ValidatedPtr",
    ),
    ("core::memory::raw::RawPtr", "core::memory::raw::RawPtr"),
];

const VARIADIC_PRINT_FUNCTIONS: &[&str] = &["print", "core::types::print"];
const VARIADIC_PRINTL_FUNCTIONS: &[&str] = &["printl", "core::types::printl"];
const VARIADIC_PRINT_ALL: &[&str] = &[
    "print",
    "core::types::print",
    "printl",
    "core::types::printl",
];

pub fn api_functions() -> &'static [ApiFunction] {
    API_FUNCTIONS
}

pub fn variadic_print_function_names() -> &'static [&'static str] {
    VARIADIC_PRINT_ALL
}

pub fn is_print_function(name: &str) -> bool {
    VARIADIC_PRINT_FUNCTIONS.contains(&name)
}

pub fn is_printl_function(name: &str) -> bool {
    VARIADIC_PRINTL_FUNCTIONS.contains(&name)
}

pub fn canonical_name(name: &str) -> Option<&'static str> {
    for func in API_FUNCTIONS {
        if func.name == name {
            return Some(func.canonical);
        }
    }
    None
}

pub fn type_from_str(name: &str) -> Type {
    Type::Path(name.to_string())
}

pub fn known_type_names() -> &'static [&'static str] {
    &[
        "String",
        "core::types::String",
        "StringSplit",
        "core::types::StringSplit",
        "StringList",
        "core::types::StringList",
        "List",
        "Option",
        "Result",
        "core::types::List",
        "core::types::Option",
        "core::types::Result",
        "HighPtr",
        "ValidatedPtr",
        "RawPtr",
        "core::memory::safe::HighPtr",
        "core::memory::safe::ValidatedPtr",
        "core::memory::raw::RawPtr",
    ]
}

pub fn canonical_type_name(name: &str) -> Option<&'static str> {
    for (alias, canonical) in API_TYPES {
        if *alias == name {
            return Some(canonical);
        }
    }
    None
}

pub fn normalize_type_name(name: &str) -> String {
    canonical_type_name(name).unwrap_or(name).to_string()
}
