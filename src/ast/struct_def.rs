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

use super::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
}
