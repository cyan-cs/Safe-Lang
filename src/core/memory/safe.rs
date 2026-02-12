// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HighPtr(*mut u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValidatedPtr(*mut u8);

impl HighPtr {
    pub(crate) fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }

    pub(crate) fn as_ptr(self) -> *mut u8 {
        self.0
    }

    pub fn addr(self) -> usize {
        self.0 as usize
    }

    pub fn is_null(self) -> bool {
        self.0.is_null()
    }
}

impl ValidatedPtr {
    pub(crate) fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }

    pub(crate) fn as_ptr(self) -> *mut u8 {
        self.0
    }

    pub fn addr(self) -> usize {
        self.0 as usize
    }

    pub fn is_null(self) -> bool {
        self.0.is_null()
    }
}

fn allocations() -> &'static Mutex<HashMap<usize, usize>> {
    static ALLOCS: OnceLock<Mutex<HashMap<usize, usize>>> = OnceLock::new();
    ALLOCS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn track_allocation(ptr: HighPtr, size: usize) {
    let key = ptr.addr();
    let mut map = allocations().lock().expect("allocations lock poisoned");
    map.insert(key, size);
}

pub fn allocation_size(ptr: HighPtr) -> Option<usize> {
    let key = ptr.addr();
    let map = allocations().lock().expect("allocations lock poisoned");
    map.get(&key).copied()
}

fn require_valid_range(ptr: HighPtr, offset: usize, len: usize) {
    if ptr.is_null() {
        panic!("high ptr must be non-null");
    }

    let size = allocation_size(ptr).unwrap_or_else(|| panic!("high ptr is invalid"));
    let end = offset
        .checked_add(len)
        .unwrap_or_else(|| panic!("high ptr range overflow"));
    if end > size {
        panic!("high ptr range out of bounds");
    }
}

pub(crate) fn write_bytes(ptr: HighPtr, offset: usize, src: &[u8]) {
    if src.is_empty() {
        return;
    }
    require_valid_range(ptr, offset, src.len());
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), ptr.as_ptr().add(offset), src.len());
    }
}

pub(crate) fn read_bytes(ptr: HighPtr, offset: usize, len: usize) -> Vec<u8> {
    if len == 0 {
        return Vec::new();
    }
    require_valid_range(ptr, offset, len);
    unsafe { std::slice::from_raw_parts(ptr.as_ptr().add(offset), len) }.to_vec()
}

pub(crate) fn write_byte(ptr: HighPtr, offset: usize, value: u8) {
    require_valid_range(ptr, offset, 1);
    unsafe {
        *ptr.as_ptr().add(offset) = value;
    }
}

pub(crate) fn read_byte(ptr: HighPtr, offset: usize) -> u8 {
    require_valid_range(ptr, offset, 1);
    unsafe { *ptr.as_ptr().add(offset) }
}

pub fn allocate_buffer(size: usize) -> HighPtr {
    if size == 0 {
        panic!("allocate_buffer size must be > 0");
    }

    let mut buf = vec![0u8; size].into_boxed_slice();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    let high_ptr = HighPtr::from_ptr(ptr);
    track_allocation(high_ptr, size);
    high_ptr
}

pub fn deallocate_buffer(ptr: HighPtr) {
    if ptr.is_null() {
        panic!("deallocate_buffer ptr must be non-null");
    }

    let key = ptr.addr();
    let size = {
        let mut map = allocations().lock().expect("allocations lock poisoned");
        map.remove(&key)
    };

    let Some(len) = size else {
        panic!("deallocate_buffer ptr is invalid or already deallocated");
    };

    unsafe {
        let slice = std::slice::from_raw_parts_mut(ptr.as_ptr(), len);
        let _ = Box::from_raw(slice);
    }
}

pub fn validate_raw(raw_ptr: super::raw::RawPtr) -> ValidatedPtr {
    if raw_ptr.is_null() {
        panic!("validate_raw ptr must be non-null");
    }
    if super::raw::allocation_size(raw_ptr).is_none() {
        panic!("validate_raw ptr is invalid");
    }
    ValidatedPtr::from_ptr(raw_ptr.as_ptr())
}

pub fn into_high(validated_ptr: ValidatedPtr) -> HighPtr {
    if validated_ptr.is_null() {
        panic!("into_high ptr must be non-null");
    }

    let size = super::raw::claim_allocation(super::raw::RawPtr::from_ptr(validated_ptr.as_ptr()))
        .unwrap_or_else(|| panic!("into_high ptr is invalid"));
    let high_ptr = HighPtr::from_ptr(validated_ptr.as_ptr());
    track_allocation(high_ptr, size);
    high_ptr
}
