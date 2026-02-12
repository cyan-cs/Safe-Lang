// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(v) => v,
            Self::Err(_) => panic!("called unwrap on Err"),
        }
    }

    pub fn unwrap_err(self) -> E {
        match self {
            Self::Ok(_) => panic!("called unwrap_err on Ok"),
            Self::Err(e) => e,
        }
    }
}

pub fn result_ok_u8_i32(value: u8) -> Result<u8, i32> {
    Result::Ok(value)
}

pub fn result_err_u8_i32(error: i32) -> Result<u8, i32> {
    Result::Err(error)
}

pub fn result_is_ok_u8_i32(value: Result<u8, i32>) -> bool {
    value.is_ok()
}

pub fn result_unwrap_u8_i32(value: Result<u8, i32>) -> u8 {
    value.unwrap()
}

pub fn result_unwrap_err_u8_i32(value: Result<u8, i32>) -> i32 {
    value.unwrap_err()
}
