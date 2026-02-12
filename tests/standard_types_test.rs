// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use safe_lang::core::types::{
    List, String as SafeString, list_get_u8, list_is_empty, list_len, list_new, list_push_bytes,
    list_push_u8, string_append_bytes, string_clear, string_clear_with_capacity, string_clone,
    string_concat, string_contains, string_ends_with, string_eq, string_from_list,
    string_insert_bytes, string_is_empty, string_len, string_list_get, string_list_is_empty,
    string_list_len, string_new, string_pop, string_pop_n, string_push, string_push_bytes,
    string_push_str, string_remove, string_remove_range, string_replace, string_split_all,
    string_split_found, string_split_left, string_split_n, string_split_once, string_split_right,
    string_starts_with, string_substr, string_to_list, string_trim, string_trim_end,
    string_trim_start,
};

#[test]
fn test_safe_string_roundtrip_and_append() {
    let mut s = SafeString::from("safe");
    assert_eq!(s.len(), 4);
    assert!(!s.as_high_ptr().is_null());

    s.push_str("? lang");
    assert_eq!(s.to_std_string(), "safe? lang");
}

#[test]
fn test_safe_list_push_and_get() {
    let mut list = List::new();
    assert!(list.is_empty());
    assert!(!list.as_high_ptr().is_null());

    for v in [1u8, 2u8, 3u8, 4u8, 5u8] {
        list.push(v);
    }

    assert_eq!(list.len(), 5);
    assert_eq!(list.get(0), Some(1));
    assert_eq!(list.get(4), Some(5));
    assert_eq!(list.get(5), None);
    assert_eq!(list.to_vec(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_safe_string_helpers() {
    let empty = string_new();
    assert!(string_is_empty(&empty));

    let a = SafeString::from("safe");
    let b = SafeString::from("? lang");
    let joined = string_concat(&a, &b);
    assert_eq!(joined.to_std_string(), "safe? lang");
    assert_eq!(string_len(&string_clone(&joined)), 10);
    assert!(string_eq(
        &string_clone(&joined),
        &SafeString::from("safe? lang")
    ));

    let base = SafeString::from("safe? lang");
    assert!(string_starts_with(
        &string_clone(&base),
        &SafeString::from("safe")
    ));
    assert!(string_ends_with(
        &string_clone(&base),
        &SafeString::from("lang")
    ));
    assert!(string_contains(
        &string_clone(&base),
        &SafeString::from("? ")
    ));

    let sub = string_substr(&base, 6, 4);
    assert_eq!(sub.to_std_string(), "lang");
}

#[test]
fn test_safe_string_mut_helpers() {
    let mut s = SafeString::from("hi");
    string_push(&mut s, b'!');
    assert_eq!(s.to_std_string(), "hi!");

    let suffix = SafeString::from(" ok");
    string_push_str(&mut s, &suffix);
    assert_eq!(s.to_std_string(), "hi! ok");

    string_clear(&mut s);
    assert_eq!(s.to_std_string(), "");
    assert!(string_is_empty(&s));
}

#[test]
fn test_safe_string_replace_trim_split_and_convert() {
    let base = SafeString::from("  safe? lang  ");
    let trimmed = string_trim(&base);
    assert_eq!(trimmed.to_std_string(), "safe? lang");
    assert_eq!(string_trim_start(&base).to_std_string(), "safe? lang  ");
    assert_eq!(string_trim_end(&base).to_std_string(), "  safe? lang");

    let replaced = string_replace(
        &trimmed,
        &SafeString::from("lang"),
        &SafeString::from("world"),
    );
    assert_eq!(replaced.to_std_string(), "safe? world");

    let split = string_split_once(&replaced, &SafeString::from("? "));
    assert!(string_split_found(&split));
    assert_eq!(string_split_left(&split).to_std_string(), "safe");
    assert_eq!(string_split_right(&split).to_std_string(), "world");

    let list = string_to_list(&replaced);
    let roundtrip = string_from_list(&list);
    assert_eq!(roundtrip.to_std_string(), "safe? world");
}

#[test]
fn test_safe_string_split_all_and_n() {
    let base = SafeString::from("a,b,c,d");
    let split_all = string_split_all(&base, &SafeString::from(","));
    assert_eq!(string_list_len(&split_all), 4);
    assert!(!string_list_is_empty(&split_all));
    assert_eq!(string_list_get(&split_all, 2).unwrap().to_std_string(), "c");

    let split_n = string_split_n(&base, &SafeString::from(","), 2);
    assert_eq!(string_list_len(&split_n), 2);
    assert_eq!(
        string_list_get(&split_n, 1).unwrap().to_std_string(),
        "b,c,d"
    );
}

#[test]
fn test_safe_list_api_helpers() {
    let mut list = list_new();
    assert!(list_is_empty(&list));
    list_push_u8(&mut list, 10);
    list_push_u8(&mut list, 20);
    assert_eq!(list_len(&list), 2);
    assert_eq!(list_get_u8(&list, 0).unwrap(), 10);
    assert!(list_get_u8(&list, 5).is_none());
}

#[test]
fn test_safe_list_push_bytes_and_string_append() {
    let mut a = list_new();
    list_push_u8(&mut a, 65);
    list_push_u8(&mut a, 66);

    let mut b = list_new();
    list_push_u8(&mut b, 67);
    list_push_u8(&mut b, 68);

    list_push_bytes(&mut a, &b);
    assert_eq!(list_len(&a), 4);
    assert_eq!(list_get_u8(&a, 2).unwrap(), 67);

    let mut s = SafeString::from("AB");
    string_append_bytes(&mut s, &b);
    assert_eq!(s.to_std_string(), "ABCD");
}

#[test]
fn test_safe_string_pop_and_remove() {
    let mut s = SafeString::from("xyz");
    assert_eq!(string_pop(&mut s).unwrap(), b'z');
    assert_eq!(s.to_std_string(), "xy");

    assert_eq!(string_remove(&mut s, 0).unwrap(), b'x');
    assert_eq!(s.to_std_string(), "y");

    assert!(string_pop(&mut s).is_some());
    assert!(string_pop(&mut s).is_none());
    assert!(string_remove(&mut s, 0).is_none());
}

#[test]
fn test_safe_string_push_bytes_insert_remove_range_and_clear_cap() {
    let mut list = list_new();
    list_push_u8(&mut list, 97);
    list_push_u8(&mut list, 98);

    let mut s = SafeString::from("XY");
    string_push_bytes(&mut s, &list);
    assert_eq!(s.to_std_string(), "XYab");

    string_insert_bytes(&mut s, 2, &list);
    assert_eq!(s.to_std_string(), "XYabab");

    let removed = string_remove_range(&mut s, 2, 2);
    assert_eq!(string_from_list(&removed).to_std_string(), "ab");
    assert_eq!(s.to_std_string(), "XYab");

    let popped = string_pop_n(&mut s, 3);
    assert_eq!(string_from_list(&popped).to_std_string(), "Yab");
    assert_eq!(s.to_std_string(), "X");

    string_clear_with_capacity(&mut s);
    assert!(string_is_empty(&s));
}
