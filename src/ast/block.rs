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

use super::{Expression, Type};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub unsafe_block: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
    If(IfStatement),
    For(ForStatement),
    Break,
    Continue,
    Expr(Expression),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetStatement {
    pub name: String,
    pub ty: Option<Type>,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstStatement {
    pub name: String,
    pub ty: Option<Type>,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStatement {
    pub var_name: String,
    pub start: Expression,
    pub end: Expression,
    pub inclusive: bool,
    pub body: Block,
}
