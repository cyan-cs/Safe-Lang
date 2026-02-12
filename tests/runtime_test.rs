// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use safe_lang::{core, into_high, validate_raw};

#[test]
fn test_allocate_buffer_returns_non_null_for_positive_size() {
    let ptr = core::memory::safe::allocate_buffer(8);
    assert!(!ptr.is_null());
}

#[test]
fn test_raw_write_updates_memory() {
    let ptr = unsafe { core::memory::raw::alloc(4) };
    unsafe {
        core::memory::raw::write(ptr, 1, 42);
        let value = core::memory::raw::read(ptr, 1);
        assert_eq!(value, 42u8);
        core::memory::raw::deallocate(ptr);
    }
}

#[test]
#[should_panic]
fn test_raw_write_rejects_out_of_bounds() {
    let ptr = unsafe { core::memory::raw::alloc(1) };
    unsafe {
        core::memory::raw::write(ptr, 2, 1);
    }
}

#[test]
fn test_validate_raw_into_high_roundtrip() {
    let raw = unsafe { core::memory::raw::alloc(1) };
    let validated = validate_raw(raw);
    let high = into_high(validated);
    assert_eq!(raw.addr(), high.addr());
    core::memory::safe::deallocate_buffer(high);
}

#[test]
#[should_panic]
fn test_deallocate_buffer_releases_allocation() {
    let ptr = core::memory::safe::allocate_buffer(2);
    core::memory::safe::deallocate_buffer(ptr);
    core::memory::safe::deallocate_buffer(ptr);
}
