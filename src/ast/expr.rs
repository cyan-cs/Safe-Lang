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

use super::Block;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Integer(i64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Call(CallExpr),
    Variable(String),
    Literal(Literal),
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Ref {
        mutable: bool,
        expr: Box<Expression>,
    },
    Block(Block),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpr {
    pub func_name: String,
    pub args: Vec<Expression>,
}
