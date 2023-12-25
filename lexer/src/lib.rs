// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
pub mod chscan;
mod token;

#[cfg(test)]
mod tests;

use chscan::*;

///
pub struct Lexer<'a> {
    scanner: ChScanner<'a>,
    peeked: Option<token::Result<'a>>
}

impl<'a> Lexer<'a> {

    /// Creates a new Lexer for the given text.
    pub fn new_from_str(text: &'a str) -> Self {
        Lexer{
            scanner: ChScanner::new_from_str(text),
            peeked: None
        }
    }

    fn scan_token(&mut self) -> Option<token::Result<'a>> {
        loop {
            self.scanner.set_marker();
            match self.scanner.next() {
                None => return None,
                Some(ch) => {
                    match ch {
                        ' ' | '\r' | '\n' | '\t' => continue,
                        _ => return self.scan_init_char(ch)
                    }
                }
            }
        }
    }

    fn scan_init_char(&mut self, ch: char) -> Option<token::Result<'a>> {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' | '^' => self.scan_identifier(),
            _ => Some(Err(token::Error::Internal))
        }
    }

    fn scan_identifier(&mut self) -> Option<token::Result<'a>> {
        loop {
            if let Some(ch) = self.scanner.peek() {
                match ch {
                    'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                        self.scanner.next();
                        continue
                    },
                    _ => break
                }
            }
        }
        Some( Ok(
            token::make_token(self.scanner.pos_marker(),
            token::TokenKind::Identifier(self.scanner.str_from_marker()))
        ))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = token::Result<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}