// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Option<T> {
    Some(T),
    None,
}

impl<T> Option<T> {
    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Some(v) => v,
            Self::None => panic!("called unwrap on None"),
        }
    }
}

pub fn option_some_u8(value: u8) -> Option<u8> {
    Option::Some(value)
}

pub fn option_none_u8() -> Option<u8> {
    Option::None
}

pub fn option_is_some_u8(value: Option<u8>) -> bool {
    value.is_some()
}

pub fn option_unwrap_u8(value: Option<u8>) -> u8 {
    value.unwrap()
}
