// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawPtr(*mut u8);

impl RawPtr {
    pub(crate) fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }

    pub fn as_ptr(self) -> *mut u8 {
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

fn track_allocation(ptr: RawPtr, size: usize) {
    let key = ptr.addr();
    let mut map = allocations().lock().expect("raw allocations lock poisoned");
    map.insert(key, size);
}

pub fn allocation_size(ptr: RawPtr) -> Option<usize> {
    let key = ptr.addr();
    let map = allocations().lock().expect("raw allocations lock poisoned");
    map.get(&key).copied()
}

fn take_allocation(ptr: RawPtr) -> Option<usize> {
    let key = ptr.addr();
    let mut map = allocations().lock().expect("raw allocations lock poisoned");
    map.remove(&key)
}

/// Allocates a raw byte buffer and returns an unmanaged raw pointer handle.
///
/// # Safety
/// The returned pointer must be passed to [`deallocate`] exactly once.
/// Callers must ensure reads and writes stay within the allocated size.
pub unsafe fn alloc(size: usize) -> RawPtr {
    if size == 0 {
        panic!("raw::alloc size must be > 0");
    }

    let mut buf = vec![0u8; size].into_boxed_slice();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    let raw_ptr = RawPtr::from_ptr(ptr);
    track_allocation(raw_ptr, size);
    raw_ptr
}

/// Deallocates a pointer previously returned by [`alloc`].
///
/// # Safety
/// `ptr` must be non-null, currently allocated by [`alloc`], and must not
/// have been deallocated before. Using `ptr` after this call is invalid.
pub unsafe fn deallocate(ptr: RawPtr) {
    if ptr.is_null() {
        panic!("raw::deallocate ptr must be non-null");
    }

    let Some(len) = take_allocation(ptr) else {
        panic!("raw::deallocate ptr is invalid or already deallocated");
    };

    unsafe {
        let slice = std::slice::from_raw_parts_mut(ptr.as_ptr(), len);
        let _ = Box::from_raw(slice);
    }
}

/// Writes one byte at `offset` within an allocated raw buffer.
///
/// # Safety
/// `ptr` must be a valid, live allocation from [`alloc`] and `offset` must be
/// strictly less than the allocation size.
pub unsafe fn write(ptr: RawPtr, offset: usize, value: u8) {
    if ptr.is_null() {
        panic!("raw::write ptr must be non-null");
    }
    let size = allocation_size(ptr).unwrap_or_else(|| panic!("raw::write ptr is invalid"));
    if offset >= size {
        panic!("raw::write offset out of bounds");
    }
    unsafe { *ptr.as_ptr().add(offset) = value };
}

/// Reads one byte at `offset` within an allocated raw buffer.
///
/// # Safety
/// `ptr` must be a valid, live allocation from [`alloc`] and `offset` must be
/// strictly less than the allocation size.
pub unsafe fn read(ptr: RawPtr, offset: usize) -> u8 {
    if ptr.is_null() {
        panic!("raw::read ptr must be non-null");
    }
    let size = allocation_size(ptr).unwrap_or_else(|| panic!("raw::read ptr is invalid"));
    if offset >= size {
        panic!("raw::read offset out of bounds");
    }
    unsafe { *ptr.as_ptr().add(offset) }
}

pub(crate) fn claim_allocation(ptr: RawPtr) -> Option<usize> {
    take_allocation(ptr)
}
