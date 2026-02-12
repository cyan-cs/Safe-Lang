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

use super::{Alias, Function, Struct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceFile {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Function(Function),
    Alias(Alias),
    Struct(Struct),
}
