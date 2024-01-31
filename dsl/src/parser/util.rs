// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::lexer::{Token, TokenKind};
use crate::parser::{Error, Parser};
#[macro_export]
macro_rules! expect_identifier {
    ($expr: expr) => {{
        use crate::lexer::TokenKind;
        use crate::parser::Error;
        match $expr.peek() {
            tk = Some(Ok(TokenKind::Identifier(_))) => {
                $expr.next();
                Ok(tk)
            }
            tk => {
                Err(Error::ExpectedIdentifier(tk))
            }
        }
    }};
}

#[macro_export]
macro_rules! check_token_kind {
    ($parser: expr, $tk: pat) => {{
        if let Some(Ok(token)) = $parser.lexer.peek() {
            match token.kind {
                $tk => {
                    $parser.lexer.next();
                    Some(token)
                },
                _ => None
            }
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! exp_token_kind {
    ($parser: expr, $tk: pat, $errstr: expr) => {{
        match $parser.lexer.peek() {
            None => Err(Error::UnexpectedEndOfFile),
            Some(Err(err)) => Err(Error::from(err)),
            Some(Ok(token)) => {
                match token.kind {
                    $tk => Ok($parser.lexer.next().unwrap().unwrap()),
                    _ => Err(Error::ExpectedToken(token, $errstr))
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! opt_token {
    ($parser: expr, $token: pat) => {{
        use crate::lexer::TokenKind;
        if let Some(Ok(Token{position: pos, kind: $token})) = $parser.lexer.peek() {
            $parser.lexer.next();
            Some(pos)
        } else {
            None
        }
    }};
}

impl<'a> Parser<'a> {

    pub fn expect_keyword(&mut self, keyword: &str) -> crate::parser::Result<usize> {
        match self.expect_identifier() {
            Err(err) => {
                match err {
                    Error::UnexpectedEndOfFile => Err(err),
                    Error::Lexer(_) => Err(err),
                    Error::ExpectedIdentifier(pos, len, s) =>
                        Err(Error::ExpectedKeyword(pos, len,
                                                   format!("Expected keyword '{}' but got token '{}' instead ", keyword, s))),
                    _ => {panic!("unexpected parser error type")}
                }
            }
            Ok((id, pos, token)) => {
                if id == keyword {
                    Ok(pos)
                } else {
                    Err(Error::ExpectedKeyword(pos, token.kind.len(),
                                               format!("Expected keyword '{}' but got identifier '{}' instead ", keyword, token.kind)))
                }
            },
        }
    }

    pub fn expect_identifier(&mut self) -> crate::parser::Result<(&'a str, usize /*pos*/, Token<'a>)> {
        match self.lexer.peek() {
            None => Err(Error::UnexpectedEndOfFile),
            Some(Err(err)) => {
                self.lexer.next();
                Err(Error::from(err))
            },
            Some(Ok(tk)) => {
                match &tk.kind {
                    TokenKind::Identifier(id) => {
                        self.lexer.next();
                        Ok((id, tk.position, tk))
                    }
                    _ => {
                        Err(Error::ExpectedIdentifier(tk.position, tk.kind.len(), format!("{}", tk.kind)))
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod parser_util_tests {
    use crate::parser::{Parser};

    #[test]
    fn exp_token() {
        use crate::lexer::TokenKind;
        let mut parser = Parser::new_from_str("name 12 ! <=\npackage");

        let r = check_token_kind!(parser, TokenKind::Identifier(_));

        assert!(r.is_some())
    }

    macro_rules! assert_id {
        ($expr: expr, $id: expr, $pos: expr) => {{
            use crate::lexer::TokenKind;
            let result = $expr.expect_identifier().expect("Expected OK, got Err");
            assert_eq!(result.0, $id);
            assert_eq!(result.1, $pos);
            assert_eq!(result.2.position, $pos);
            if let TokenKind::Identifier(s) = result.2.kind {
                assert_eq!(s, $id);
            } else {
                assert!(false, "Expected token identifier, got something else {:?}", result.2.kind)
            }
        };
    }}

    #[test]
    fn identifier() {
        let mut parser = Parser::new_from_str("name \nvariable 12");

        assert_id!(parser, "name", 0);
        assert_id!(parser, "variable", 6);
        assert!(parser.expect_identifier().is_err());
    }

    macro_rules! assert_keyword {
        ($expr: expr, $kw: expr, $pos: expr) => {{
            if let Ok(result) = $expr.expect_keyword($kw) {
                assert_eq!(result, $pos)
            } else {
                assert!(false, "Expected keyword '{:?}', got error.", $kw)
            }
        }}
    }

    #[test]
    fn keyword() {
        let mut parser = Parser::new_from_str("package struct 42");

        assert_keyword!(parser, "package", 0);
        assert_keyword!(parser, "struct", 8);
        assert!(parser.expect_keyword("x").is_err());
    }
}