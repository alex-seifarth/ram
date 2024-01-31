// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
pub mod ast;
mod common;
mod util;

use crate::lexer::{LexerError, Lexer,  TokenKind, Token};

#[derive(Debug, Clone)]
pub enum Error {
    Lexer(LexerError),
    UnexpectedEndOfFile,
    ExpectedIdentifier(usize /*pos*/, usize /*len*/, String /*error description*/),
    ExpectedKeyword(usize /*pos*/, usize /*len*/, String /*error description */),
    ExpectedToken(usize /*pos*/, String),
}

impl From<LexerError> for Error {
    fn from(value: LexerError) -> Self {
        Error::Lexer(value)
    }
}

pub type Result<'a, T> = std::result::Result<T, Error>;

/// Parser for .ram file.
/// The parser constructs the abstract syntax tree for single .ram string.
pub struct Parser<'a> {
    lexer: Lexer<'a>,

}

impl<'a> Parser<'a> {

    /// Constructs a new parser from the text string slice.
    pub fn new_from_str(text: &'a str) -> Self {
        Parser {
            lexer: Lexer::new_from_str(text)
        }
    }



    fn next(&mut self) -> Option<i64> {
        
        None
    }
}
