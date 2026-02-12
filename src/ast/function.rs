// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

// cyan-cs
//
// Copyright (c) 2026 cyan-cs
//
// This software is licensed under the MIT License.
// See: https://opensource.org/licenses/MIT

use serde::{Deserialize, Serialize};

use super::{Block, Type};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub safety: SafetyLevel,
    pub args: Vec<Arg>,
    pub ret_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arg {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SafetyLevel {
    Safe,
    Raw,
}
