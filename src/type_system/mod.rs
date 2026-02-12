// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use std::marker::PhantomData;

pub trait SafetyLevel {
    const LEVEL: u8;
}

pub struct HighLevel;
impl SafetyLevel for HighLevel {
    const LEVEL: u8 = 2;
}

pub struct RawLevel;
impl SafetyLevel for RawLevel {
    const LEVEL: u8 = 0;
}

pub struct ValidatedLevel;
impl SafetyLevel for ValidatedLevel {
    const LEVEL: u8 = 1;
}

#[derive(Debug)]
pub struct Typed<T, S: SafetyLevel> {
    value: T,
    _safety: PhantomData<S>,
}

impl<T, S: SafetyLevel> Typed<T, S> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            _safety: PhantomData,
        }
    }

    pub fn unwrap(self) -> T {
        self.value
    }
}

pub type High<T> = Typed<T, HighLevel>;
pub type Raw<T> = Typed<T, RawLevel>;
pub type Validated<T> = Typed<T, ValidatedLevel>;

impl<T> Validated<T> {
    pub fn into_high(self) -> High<T> {
        High::new(self.value)
    }
}

pub fn validate_raw<T>(raw: Raw<T>) -> Validated<T> {
    Validated::new(raw.value)
}

pub fn example_usage() {
    let raw_val: Raw<i32> = Raw::new(10);
    let validated = validate_raw(raw_val);
    let _high = validated.into_high();
}
