// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
pub type Result<'a> = std::result::Result<Token<'a>, Error<'a>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error<'a> {
    IntegerNoValue(&'a str, usize),
    IntegerSeparatorAtEnd(&'a str, usize),
    IntegerExceedingLimit(&'a str, usize),
    FloatParsingError(&'a str, usize),

    UnknownToken(&'a str, usize),
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub position: usize,
    pub kind: TokenKind<'a>
}

pub fn make_token(position: usize, kind: TokenKind) -> Token {
    Token{position, kind}
}

#[derive(Clone, Debug)]
pub enum TokenKind<'a> {
    Identifier(&'a str),
    Integer(&'a str, u64),
    Float(&'a str, f64),

    BracesLeft,         // {
    BracesRight,        // }
    Equals,             // ==
    Assign,             // =
    Colon,              // :
    ExclMark,           // !
    NotEqual,           // !=
    Greater,            // >
    GreaterThan,        // >=
    Less,               // <
    LessThan,           // <=
    Plus,               // +
    Increment,          // +=
    Minus,              // -
    Decrement,          // -=
    Star,               // *
    Multiply,           // *=
    Slash,              // /
    Divide,             // /=
    Ampersand,          // &
    LogicalAnd,         // &&
    Vert,               // |
    LogicalOr,          // ||
    PerCent,            // %
    Hash,               // #
    Dollar,             // $
    DoubleQuote,        // "
    SingleQuote,        // '
    SingleQuote2,       // ''
    QuestionTag,        // ?
    Comma,              // ,
    Dot,                // .
    Range,              // ..
    RangeIncl,          // ..=
    Tilde,              // ~
    BracketLeft,        // [
    DoubleBracketLeft,  // [[
    BracketRight,       // ]
    DoubleBracketRight, // ]]
    ParensLeft,         // (
    ParensRight,        // )
}