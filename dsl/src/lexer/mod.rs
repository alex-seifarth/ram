// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
mod chscan;
mod token;

#[cfg(test)]
mod tests;

use std::str::FromStr;
use chscan::*;
pub use token::TokenKind;

pub type LexerError<'a> = token::Error<'a>;

///
pub struct Lexer<'a> {
    scanner: ChScanner<'a>,
    peeked: Option< Option<token::Result<'a>>>
}

impl<'a> Lexer<'a> {

    /// Creates a new Lexer for the given text.
    pub fn new_from_str(text: &'a str) -> Self {
        Lexer{
            scanner: ChScanner::new_from_str(text),
            peeked: None
        }
    }

    pub fn peek(&mut self) -> Option<token::Result<'a>> {
        if self.peeked.is_none() {
            self.peeked = Some(self.scan_token());
        }
        self.peeked.as_ref().unwrap().clone()
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
            '{' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::BracesLeft))),
            '}' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::BracesRight))),
            ':' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Colon))),
            '=' => {
                match self.scanner.peek() {
                    Some('=') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::Equals)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Assign)))
                }
            }
            '!' => {
                match self.scanner.peek() {
                    Some('=') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::NotEqual)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::ExclMark)))
                }
            }
            '>' => {
                match self.scanner.peek() {
                    Some('=') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::GreaterThan)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Greater)))
                }
            }
            '<' => {
                match self.scanner.peek() {
                    Some('=') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::LessThan)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Less)))
                }
            },
            '+' => {
                match self.scanner.peek() {
                    Some('=') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::Increment)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Plus)))
                }
            },
            '-' => {
                match self.scanner.peek() {
                    Some('=') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::Decrement)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Minus)))
                }
            },
            '*' => {
                match self.scanner.peek() {
                    Some('=') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::Multiply)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Star)))
                }
            },
            '/' => {
                match self.scanner.peek() {
                    Some('=') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::Divide)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Slash)))
                }
            },
            '&' => {
                match self.scanner.peek() {
                    Some('&') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::LogicalAnd)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Ampersand)))
                }
            },
            '|' => {
                match self.scanner.peek() {
                    Some('|') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::LogicalOr)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Vert)))
                }
            },
            '%' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::PerCent))),
            '#' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Hash))),
            '$' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Dollar))),
            '"' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::DoubleQuote))),
            '\'' =>  {
                match self.scanner.peek() {
                    Some('\'') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::SingleQuote2)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::SingleQuote)))
                }
            },
            '?' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::QuestionTag))),
            ',' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Comma))),
            '.' => {
                match self.scanner.peek() {
                    Some('.') => {
                        self.next();
                        match self.scanner.peek() {
                            Some('=') => {
                                self.next();
                                Some(Ok( token::make_token(self.scanner.position() - 3, TokenKind::RangeIncl)))
                            }
                            _ => Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::Range)))
                        }
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Dot)))
                }
            }
            '~' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::Tilde))),
            '[' =>  {
                match self.scanner.peek() {
                    Some('[') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::DoubleBracketLeft)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::BracketLeft)))
                }
            },
            ']' =>  {
                match self.scanner.peek() {
                    Some(']') => {
                        self.next();
                        Some(Ok( token::make_token(self.scanner.position() - 2, TokenKind::DoubleBracketRight)))
                    }
                    _ => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::BracketRight)))
                }
            },
            '(' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::ParensLeft))),
            ')' => Some(Ok( token::make_token(self.scanner.position() - 1, TokenKind::ParensRight))),
            '0' => {
                match self.scanner.peek().as_ref().unwrap_or(&'\0') {
                    'x' | 'X' => self.scan_hex_integer(),
                    'b' | 'B' => self.scan_bin_integer(),
                    _ => self.scan_dec(ch)
                }
            }
            '1'..='9' => self.scan_dec(ch),
            _ => Some(Err(token::Error::UnknownToken( self.scanner.str_from_marker(), self.scanner.pos_marker() )))
        }
    }

    fn scan_dec(&mut self, initial_digit: char) -> Option<token::Result<'a>> {
        let mut buf = String::new();
        buf.push(initial_digit);
        loop {
            if let Some(ch) = self.scanner.peek() {
                match ch {
                    '0'..='9' => {
                        buf.push(ch);
                        self.scanner.next();
                        continue
                    }
                    '\'' => {
                        self.scanner.next();
                        continue
                    }
                    '.' => {
                        self.scanner.next();
                        return self.scan_float()
                    }
                    'e' | 'E' => {
                        return self.scan_float()
                    }
                    _ => {}
                }
            }
            break
        }
        let string = self.scanner.str_from_marker();
        return if string.ends_with('\'') {
            Some(Err(token::Error::IntegerSeparatorAtEnd(string, self.scanner.pos_marker())))
        } else {
            match u64::from_str_radix(&buf, 10) {
                Ok(v) => Some(Ok(token::make_token(
                    self.scanner.pos_marker(),
                    TokenKind::Integer(self.scanner.str_from_marker(), v)
                ))),
                Err(e) => {
                    assert!(e.kind().eq(&std::num::IntErrorKind::PosOverflow));
                    Some(Err(token::Error::IntegerExceedingLimit(string, self.scanner.pos_marker())))
                }
            }
        }
    }

    fn scan_float(&mut self) -> Option<token::Result<'a>> {
        // we enter here after the . is consumed - so we're in the decimals part
        let mut in_exp = false;
        loop {
            if let Some(ch) = self.scanner.peek() {
                match ch {
                    '0'..='9' => {
                        self.scanner.next();
                        continue
                    }
                    'e' | 'E' => {
                        if !in_exp {
                            in_exp = true;
                            self.scanner.next();
                            match self.scanner.peek() {
                                Some('-') | Some('+') => {self.scanner.next();},
                                _ => {}
                            }
                            continue
                        }
                    }
                    _ => {}
                }
            }
            break
        }
        let string = self.scanner.str_from_marker();
        return match f64::from_str(string) {
            Ok(v) => Some(Ok( token::make_token(
                self.scanner.pos_marker(),
                TokenKind::Float(string, v)
            ))),
            Err(_) => Some(Err(
                token::Error::FloatParsingError(string, self.scanner.pos_marker())
            ))
        }
    }

    fn scan_bin_integer(&mut self) -> Option<token::Result<'a>> {
        self.scanner.next(); // consumes the b or B from prefix 0b / 0B
        let mut val = 0u64;
        let mut len = 0usize;
        loop {
            if let Some(ch) = self.scanner.peek() {
                match ch {
                    '0'..='1' => {
                        val = (val << 1) | (ch.to_digit(2).unwrap() as u64);
                        len += 1;
                        self.scanner.next();
                        continue
                    }
                    '\'' => {
                        self.scanner.next();
                        continue
                    }
                    _ => {}
                }
            }
            break;
        }
        let string = self.scanner.str_from_marker();
        if len == 0 {
            Some(Err(token::Error::IntegerNoValue(string, self.scanner.pos_marker())))
        } else if len > 64*8 {
            Some(Err(token::Error::IntegerExceedingLimit(string, self.scanner.pos_marker())))
        } else if string.ends_with('\'') {
            Some(Err(token::Error::IntegerSeparatorAtEnd(string, self.scanner.pos_marker())))
        } else {
            Some(Ok( token::make_token(
                self.scanner.pos_marker(),
                TokenKind::Integer(string, val)
            ) ))
        }
    }

    fn scan_hex_integer(&mut self) -> Option<token::Result<'a>> {
        self.scanner.next(); // consumes the x or X from prefix 0x / 0X
        let mut val = 0u64;
        let mut len = 0usize;
        loop {
            if let Some(ch) = self.scanner.peek() {
                match ch {
                    'a'..='f' | 'A'..='F' | '0'..='9' => {
                        val = (val << 4) | (ch.to_digit(16).unwrap() as u64);
                        len += 1;
                        self.scanner.next();
                        continue
                    }
                    '\'' => {
                        self.scanner.next();
                        continue
                    }
                    _ => {}
                }
            }
            break
        }
        let string = self.scanner.str_from_marker();
        if len == 0 {
            Some(Err(token::Error::IntegerNoValue(string, self.scanner.pos_marker())))
        } else if len > 16 {
            Some(Err(token::Error::IntegerExceedingLimit(string, self.scanner.pos_marker())))
        } else if string.ends_with('\'') {
            Some(Err(token::Error::IntegerSeparatorAtEnd(string, self.scanner.pos_marker())))
        } else {
            Some(Ok( token::make_token(
                self.scanner.pos_marker(),
                TokenKind::Integer(string, val)
            ) ))
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
            } else {
                break
            }
        }
        Some( Ok(
            token::make_token(self.scanner.pos_marker(),
            TokenKind::Identifier(self.scanner.str_from_marker()))
        ))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = token::Result<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.peeked.take() {
            p
        } else {
            self.scan_token()
        }
    }
}