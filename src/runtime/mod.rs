// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use crate::core::memory;
pub use crate::type_system::*;

pub fn validate_raw(raw: memory::raw::RawPtr) -> memory::safe::ValidatedPtr {
    memory::safe::validate_raw(raw)
}

pub fn into_high(validated: memory::safe::ValidatedPtr) -> memory::safe::HighPtr {
    memory::safe::into_high(validated)
}
