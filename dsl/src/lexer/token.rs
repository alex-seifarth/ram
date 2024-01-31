use std::fmt;
use std::fmt::Formatter;

// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
pub type Result<'a> = std::result::Result<Token<'a>, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    IntegerNoValue(String, usize),
    IntegerSeparatorAtEnd(String, usize),
    IntegerExceedingLimit(String, usize),
    FloatParsingError(String, usize),

    UnknownToken(String, usize),
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

impl<'a> TokenKind<'a> {

    pub fn len(&self) -> usize {
        match self {
            TokenKind::Identifier(id) => id.len(),
            TokenKind::Integer(s, _) => s.len(),
            TokenKind::Float(s, _) => s.len(),
            TokenKind::BracesLeft => 1usize,
            TokenKind::BracesRight => 1usize,
            TokenKind::Equals => 2usize,
            TokenKind::Assign => 1usize,
            TokenKind::Colon => 1usize,
            TokenKind::ExclMark => 1usize,
            TokenKind::NotEqual => 2usize,
            TokenKind::Greater => 1usize,
            TokenKind::GreaterThan => 2usize,
            TokenKind::Less => 1usize,
            TokenKind::LessThan => 2usize,
            TokenKind::Plus => 1usize,
            TokenKind::Increment => 2usize,
            TokenKind::Minus => 1usize,
            TokenKind::Decrement => 2usize,
            TokenKind::Star => 1usize,
            TokenKind::Multiply => 2usize,
            TokenKind::Slash => 1usize,
            TokenKind::Divide => 2usize,
            TokenKind::Ampersand => 1usize,
            TokenKind::LogicalAnd => 2usize,
            TokenKind::Vert => 1usize,
            TokenKind::LogicalOr => 2usize,
            TokenKind::PerCent => 1usize,
            TokenKind::Hash => 1usize,
            TokenKind::Dollar => 1usize,
            TokenKind::DoubleQuote => 2usize,
            TokenKind::SingleQuote => 1usize,
            TokenKind::SingleQuote2 => 3usize,
            TokenKind::QuestionTag => 1usize,
            TokenKind::Comma => 1usize,
            TokenKind::Dot => 1usize,
            TokenKind::Range => 2usize,
            TokenKind::RangeIncl => 3usize,
            TokenKind::Tilde => 1usize,
            TokenKind::BracketLeft => 1usize,
            TokenKind::DoubleBracketLeft => 2usize,
            TokenKind::BracketRight => 1usize,
            TokenKind::DoubleBracketRight => 2usize,
            TokenKind::ParensLeft => 1usize,
            TokenKind::ParensRight => 1usize,
        }
    }
}

impl<'a> fmt::Display for TokenKind<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       match self {
           TokenKind::Identifier(id) => write!(f, "Identifier '{}'", id),
           TokenKind::Integer(s, _) => write!(f, "Integer '{}'", s),
           TokenKind::Float(s, _) => write!(f, "Float '{}'", s),
           TokenKind::BracesLeft => write!(f, "'{{'"),
           TokenKind::BracesRight => write!(f, "'}}'"),
           TokenKind::Equals => write!(f, "'=='"),
           TokenKind::Assign => write!(f, "'='"),
           TokenKind::Colon => write!(f, "':'"),
           TokenKind::ExclMark => write!(f, "'!'"),
           TokenKind::NotEqual => write!(f, "'!='"),
           TokenKind::Greater => write!(f, "'>'"),
           TokenKind::GreaterThan => write!(f, "'>='"),
           TokenKind::Less => write!(f, "'<'"),
           TokenKind::LessThan => write!(f, "'<='"),
           TokenKind::Plus => write!(f, "'+'"),
           TokenKind::Increment => write!(f, "'+='"),
           TokenKind::Minus => write!(f, "'-'"),
           TokenKind::Decrement => write!(f, "'-='"),
           TokenKind::Star => write!(f, "'*'"),
           TokenKind::Multiply => write!(f, "'*='"),
           TokenKind::Slash => write!(f, "'/'"),
           TokenKind::Divide => write!(f, "'/='"),
           TokenKind::Ampersand => write!(f, "'&'"),
           TokenKind::LogicalAnd => write!(f, "'&&'"),
           TokenKind::Vert => write!(f, "'|'"),
           TokenKind::LogicalOr => write!(f, "'||'"),
           TokenKind::PerCent => write!(f, "'%'"),
           TokenKind::Hash => write!(f, "'#'"),
           TokenKind::Dollar => write!(f, "'$'"),
           TokenKind::DoubleQuote => write!(f, "'\"\"'"),
           TokenKind::SingleQuote => write!(f, "'''"),
           TokenKind::SingleQuote2 => write!(f, "''''"),
           TokenKind::QuestionTag => write!(f, "'?'"),
           TokenKind::Comma => write!(f, "','"),
           TokenKind::Dot => write!(f, "'.'"),
           TokenKind::Range => write!(f, "'..'"),
           TokenKind::RangeIncl => write!(f, "'..='"),
           TokenKind::Tilde => write!(f, "'~'"),
           TokenKind::BracketLeft => write!(f, "'['"),
           TokenKind::DoubleBracketLeft => write!(f, "'[['"),
           TokenKind::BracketRight => write!(f, "']'"),
           TokenKind::DoubleBracketRight => write!(f, "']]'"),
           TokenKind::ParensLeft => write!(f, "'('"),
           TokenKind::ParensRight => write!(f, "')'"),
       }
    }
}