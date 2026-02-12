// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::core::memory::safe::{self, HighPtr};
use std::cmp;

use super::list::list_from_bytes;
use super::{List, Option};

#[derive(Debug)]
pub struct String {
    ptr: HighPtr,
    len: usize,
    cap: usize,
}

impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}

impl String {
    pub fn new() -> Self {
        Self::from_bytes(&[])
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let cap = cmp::max(1, bytes.len());
        let ptr = safe::allocate_buffer(cap);

        if !bytes.is_empty() {
            safe::write_bytes(ptr, 0, bytes);
        }

        Self {
            ptr,
            len: bytes.len(),
            cap,
        }
    }

    pub fn from_text(value: &str) -> Self {
        let bytes = value.as_bytes();
        let cap = cmp::max(1, bytes.len());
        let ptr = safe::allocate_buffer(cap);

        if !bytes.is_empty() {
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.as_ptr(), bytes.len());
            }
        }

        Self {
            ptr,
            len: bytes.len(),
            cap,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_high_ptr(&self) -> HighPtr {
        self.ptr
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        safe::read_bytes(self.ptr, 0, self.len)
    }

    pub fn to_std_string(&self) -> std::string::String {
        std::string::String::from_utf8_lossy(&self.as_bytes()).into_owned()
    }

    pub fn push_str(&mut self, suffix: &str) {
        let bytes = suffix.as_bytes();
        self.push_bytes(bytes);
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn clear_with_capacity(&mut self) {
        safe::deallocate_buffer(self.ptr);
        self.ptr = safe::allocate_buffer(1);
        self.len = 0;
        self.cap = 1;
    }

    pub fn pop_byte(&mut self) -> Option<u8> {
        if self.len == 0 {
            return Option::None;
        }
        let idx = self.len - 1;
        let value = safe::read_byte(self.ptr, idx);
        self.len -= 1;
        Option::Some(value)
    }

    pub fn remove_byte(&mut self, index: usize) -> Option<u8> {
        if index >= self.len {
            return Option::None;
        }
        let mut bytes = safe::read_bytes(self.ptr, 0, self.len);
        let value = bytes.remove(index);
        if !bytes.is_empty() {
            safe::write_bytes(self.ptr, 0, &bytes);
        }
        self.len = bytes.len();
        Option::Some(value)
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }

        self.reserve(bytes.len());
        safe::write_bytes(self.ptr, self.len, bytes);
        self.len += bytes.len();
    }

    fn reserve(&mut self, additional: usize) {
        let required = self.len.saturating_add(additional);
        if required <= self.cap {
            return;
        }

        let mut next_cap = self.cap;
        while next_cap < required {
            next_cap = cmp::max(next_cap * 2, 1);
        }

        let next_ptr = safe::allocate_buffer(next_cap);
        if self.len > 0 {
            let data = safe::read_bytes(self.ptr, 0, self.len);
            safe::write_bytes(next_ptr, 0, &data);
        }

        safe::deallocate_buffer(self.ptr);
        self.ptr = next_ptr;
        self.cap = next_cap;
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        Self::from_bytes(&self.as_bytes())
    }
}

impl From<&str> for String {
    fn from(value: &str) -> Self {
        Self::from_text(value)
    }
}

impl Drop for String {
    fn drop(&mut self) {
        safe::deallocate_buffer(self.ptr);
    }
}

pub fn string_new() -> String {
    String::new()
}

pub fn string_clone(value: &String) -> String {
    value.clone()
}

pub fn string_len(value: &String) -> usize {
    value.len()
}

pub fn string_is_empty(value: &String) -> bool {
    value.is_empty()
}

pub fn string_concat(left: &String, right: &String) -> String {
    let mut out = String::from_bytes(&left.as_bytes());
    out.push_bytes(&right.as_bytes());
    out
}

pub fn string_eq(left: &String, right: &String) -> bool {
    left.as_bytes() == right.as_bytes()
}

pub fn string_substr(value: &String, start: usize, len: usize) -> String {
    let bytes = value.as_bytes();
    let end = start
        .checked_add(len)
        .unwrap_or_else(|| panic!("string_substr overflow"));
    if start > bytes.len() || end > bytes.len() {
        panic!("string_substr out of bounds");
    }
    String::from_bytes(&bytes[start..end])
}

pub fn string_starts_with(value: &String, prefix: &String) -> bool {
    let bytes = value.as_bytes();
    let prefix_bytes = prefix.as_bytes();
    bytes.starts_with(&prefix_bytes)
}

pub fn string_ends_with(value: &String, suffix: &String) -> bool {
    let bytes = value.as_bytes();
    let suffix_bytes = suffix.as_bytes();
    bytes.ends_with(&suffix_bytes)
}

pub fn string_contains(value: &String, needle: &String) -> bool {
    let bytes = value.as_bytes();
    let needle_bytes = needle.as_bytes();
    if needle_bytes.is_empty() {
        return true;
    }
    bytes.windows(needle_bytes.len()).any(|w| w == needle_bytes)
}

pub fn string_push(value: &mut String, byte: u8) {
    value.push_bytes(&[byte]);
}

pub fn string_push_bytes(value: &mut String, bytes: &List) {
    string_append_bytes(value, bytes);
}

pub fn string_push_str(value: &mut String, suffix: &String) {
    value.push_bytes(&suffix.as_bytes());
}

pub fn string_clear(value: &mut String) {
    value.clear();
}

pub fn string_clear_with_capacity(value: &mut String) {
    value.clear_with_capacity();
}

pub fn string_append_bytes(value: &mut String, bytes: &List) {
    let data = bytes.to_vec();
    value.push_bytes(&data);
}

pub fn string_pop(value: &mut String) -> Option<u8> {
    value.pop_byte()
}

pub fn string_pop_n(value: &mut String, count: usize) -> List {
    let take = std::cmp::min(count, value.len());
    if take == 0 {
        return List::new();
    }
    let start = value.len() - take;
    let bytes = safe::read_bytes(value.ptr, start, take);
    value.len -= take;
    list_from_bytes(&bytes)
}

pub fn string_remove(value: &mut String, index: usize) -> Option<u8> {
    value.remove_byte(index)
}

pub fn string_remove_range(value: &mut String, start: usize, len: usize) -> List {
    let end = start
        .checked_add(len)
        .unwrap_or_else(|| panic!("string_remove_range overflow"));
    if start > value.len || end > value.len {
        panic!("string_remove_range out of bounds");
    }
    if len == 0 {
        return List::new();
    }
    let bytes = safe::read_bytes(value.ptr, 0, value.len);
    let removed = bytes[start..end].to_vec();

    let mut remaining = Vec::with_capacity(bytes.len() - len);
    remaining.extend_from_slice(&bytes[..start]);
    remaining.extend_from_slice(&bytes[end..]);

    if remaining.len() > value.cap {
        let mut next_cap = value.cap;
        while next_cap < remaining.len() {
            next_cap = std::cmp::max(next_cap * 2, 1);
        }
        let next_ptr = safe::allocate_buffer(next_cap);
        if !remaining.is_empty() {
            safe::write_bytes(next_ptr, 0, &remaining);
        }
        safe::deallocate_buffer(value.ptr);
        value.ptr = next_ptr;
        value.cap = next_cap;
    } else if !remaining.is_empty() {
        safe::write_bytes(value.ptr, 0, &remaining);
    }

    value.len = remaining.len();
    list_from_bytes(&removed)
}

pub fn string_insert_bytes(value: &mut String, index: usize, bytes: &List) {
    if index > value.len {
        panic!("string_insert_bytes out of bounds");
    }
    let mut data = safe::read_bytes(value.ptr, 0, value.len);
    let insert = bytes.to_vec();
    if !insert.is_empty() {
        data.splice(index..index, insert);
    }

    if data.len() > value.cap {
        let mut next_cap = value.cap;
        while next_cap < data.len() {
            next_cap = std::cmp::max(next_cap * 2, 1);
        }
        let next_ptr = safe::allocate_buffer(next_cap);
        if !data.is_empty() {
            safe::write_bytes(next_ptr, 0, &data);
        }
        safe::deallocate_buffer(value.ptr);
        value.ptr = next_ptr;
        value.cap = next_cap;
    } else if !data.is_empty() {
        safe::write_bytes(value.ptr, 0, &data);
    }
    value.len = data.len();
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> std::option::Option<usize> {
    if needle.is_empty() {
        return std::option::Option::Some(0);
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}

pub fn string_replace(value: &String, needle: &String, replacement: &String) -> String {
    let bytes = value.as_bytes();
    let needle_bytes = needle.as_bytes();
    let repl_bytes = replacement.as_bytes();

    if needle_bytes.is_empty() {
        return value.clone();
    }

    let mut out: Vec<u8> = Vec::new();
    let mut cursor = 0usize;
    while let Some(pos) = find_bytes(&bytes[cursor..], &needle_bytes) {
        let start = cursor + pos;
        out.extend_from_slice(&bytes[cursor..start]);
        out.extend_from_slice(&repl_bytes);
        cursor = start + needle_bytes.len();
    }
    out.extend_from_slice(&bytes[cursor..]);
    String::from_bytes(&out)
}

fn is_trim_byte(value: u8) -> bool {
    matches!(value, b' ' | b'\n' | b'\r' | b'\t')
}

pub fn string_trim(value: &String) -> String {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return String::new();
    }
    let mut start = 0usize;
    let mut end = bytes.len();

    while start < end && is_trim_byte(bytes[start]) {
        start += 1;
    }
    while end > start && is_trim_byte(bytes[end - 1]) {
        end -= 1;
    }
    String::from_bytes(&bytes[start..end])
}

pub fn string_trim_start(value: &String) -> String {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return String::new();
    }
    let mut start = 0usize;
    while start < bytes.len() && is_trim_byte(bytes[start]) {
        start += 1;
    }
    String::from_bytes(&bytes[start..])
}

pub fn string_trim_end(value: &String) -> String {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return String::new();
    }
    let mut end = bytes.len();
    while end > 0 && is_trim_byte(bytes[end - 1]) {
        end -= 1;
    }
    String::from_bytes(&bytes[..end])
}

#[derive(Debug, Clone)]
pub struct StringSplit {
    left: String,
    right: String,
    found: bool,
}

pub fn string_split_once(value: &String, needle: &String) -> StringSplit {
    let bytes = value.as_bytes();
    let needle_bytes = needle.as_bytes();
    if needle_bytes.is_empty() {
        return StringSplit {
            left: value.clone(),
            right: String::new(),
            found: false,
        };
    }

    if let Some(pos) = find_bytes(&bytes, &needle_bytes) {
        let left = String::from_bytes(&bytes[..pos]);
        let right = String::from_bytes(&bytes[pos + needle_bytes.len()..]);
        StringSplit {
            left,
            right,
            found: true,
        }
    } else {
        StringSplit {
            left: value.clone(),
            right: String::new(),
            found: false,
        }
    }
}

pub fn string_split_found(split: &StringSplit) -> bool {
    split.found
}

pub fn string_split_left(split: &StringSplit) -> String {
    split.left.clone()
}

pub fn string_split_right(split: &StringSplit) -> String {
    split.right.clone()
}

#[derive(Debug, Clone)]
pub struct StringList {
    items: Vec<String>,
}

impl Default for StringList {
    fn default() -> Self {
        Self::new()
    }
}

impl StringList {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn push(&mut self, value: String) {
        self.items.push(value);
    }

    pub fn get(&self, index: usize) -> Option<String> {
        match self.items.get(index) {
            std::option::Option::Some(value) => Option::Some(value.clone()),
            std::option::Option::None => Option::None,
        }
    }
}

pub fn string_split_all(value: &String, needle: &String) -> StringList {
    let bytes = value.as_bytes();
    let needle_bytes = needle.as_bytes();

    let mut list = StringList::new();
    if needle_bytes.is_empty() {
        list.push(value.clone());
        return list;
    }

    let mut cursor = 0usize;
    while let Some(pos) = find_bytes(&bytes[cursor..], &needle_bytes) {
        let start = cursor + pos;
        list.push(String::from_bytes(&bytes[cursor..start]));
        cursor = start + needle_bytes.len();
    }
    list.push(String::from_bytes(&bytes[cursor..]));
    list
}

pub fn string_split_n(value: &String, needle: &String, max_parts: usize) -> StringList {
    let bytes = value.as_bytes();
    let needle_bytes = needle.as_bytes();

    let mut list = StringList::new();
    if max_parts == 0 {
        return list;
    }
    if needle_bytes.is_empty() {
        list.push(value.clone());
        return list;
    }
    if max_parts == 1 {
        list.push(value.clone());
        return list;
    }

    let mut cursor = 0usize;
    while list.len() + 1 < max_parts {
        let Some(pos) = find_bytes(&bytes[cursor..], &needle_bytes) else {
            break;
        };
        let start = cursor + pos;
        list.push(String::from_bytes(&bytes[cursor..start]));
        cursor = start + needle_bytes.len();
    }
    list.push(String::from_bytes(&bytes[cursor..]));
    list
}

pub fn string_list_len(list: &StringList) -> usize {
    list.len()
}

pub fn string_list_is_empty(list: &StringList) -> bool {
    list.is_empty()
}

pub fn string_list_get(list: &StringList, index: usize) -> Option<String> {
    list.get(index)
}

pub fn string_from_list(list: &List) -> String {
    String::from_bytes(&list.to_vec())
}

pub fn string_to_list(value: &String) -> List {
    let mut list = List::new();
    for byte in value.as_bytes() {
        list.push(byte);
    }
    list
}
