// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::core::memory::safe::{self, HighPtr};

use super::Option;

#[derive(Debug)]
pub struct List {
    ptr: HighPtr,
    len: usize,
    cap: usize,
}

impl List {
    pub fn new() -> Self {
        let cap = 4;
        Self {
            ptr: safe::allocate_buffer(cap),
            len: 0,
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

    pub fn push(&mut self, value: u8) {
        self.reserve(1);
        safe::write_byte(self.ptr, self.len, value);
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> std::option::Option<u8> {
        if index >= self.len {
            return std::option::Option::None;
        }
        std::option::Option::Some(safe::read_byte(self.ptr, index))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        safe::read_bytes(self.ptr, 0, self.len)
    }

    fn reserve(&mut self, additional: usize) {
        let required = self.len.saturating_add(additional);
        if required <= self.cap {
            return;
        }

        let mut next_cap = self.cap;
        while next_cap < required {
            next_cap *= 2;
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

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for List {
    fn drop(&mut self) {
        safe::deallocate_buffer(self.ptr);
    }
}

pub fn list_new() -> List {
    List::new()
}

pub fn list_len(list: &List) -> usize {
    list.len()
}

pub fn list_is_empty(list: &List) -> bool {
    list.is_empty()
}

pub fn list_push_u8(list: &mut List, value: u8) {
    list.push(value);
}

pub fn list_get_u8(list: &List, index: usize) -> Option<u8> {
    match list.get(index) {
        std::option::Option::Some(value) => Option::Some(value),
        std::option::Option::None => Option::None,
    }
}

pub fn list_push_bytes(list: &mut List, bytes: &List) {
    for value in bytes.to_vec() {
        list.push(value);
    }
}

pub(super) fn list_from_bytes(bytes: &[u8]) -> List {
    let mut list = List::new();
    for value in bytes {
        list.push(*value);
    }
    list
}
