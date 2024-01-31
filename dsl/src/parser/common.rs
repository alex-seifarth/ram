use crate::lexer::{Token, TokenKind};
use crate::opt_token;
// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::parser::{Parser, Error, Result, ast, util};
use crate::parser::ast::FileLocation;

impl<'a> Parser<'a> {

    pub fn parse_package(&mut self) -> Result<()> { //ast::FQN<'a>> {
        let pos_start = self.expect_keyword("package")?;

        // exp_token_kind!(self, TokenKind::Comma, "".to_string())?;

        Ok(())
        // expect_identifier!(self.lexer?)
    }

    pub fn parse_fqn(&mut self) -> Result<ast::FQN<'a>> {
        let mut components = Vec::new();
        let mut is_wildcard = false;
        let start = self.lexer.curr_pos();
        let mut end = start;
        loop {
            let (id, pos, _) = self.expect_identifier()?;
            components.push((id, pos));
            end = pos + id.len();
            if opt_token!(self, TokenKind::Dot).is_some() {
                if let Some(p) = opt_token!(self, TokenKind::Star) {
                    is_wildcard = true;
                    end = p + 1;
                    break;
                }
                continue;
            };
            break;
        }
        Ok(ast::FQN{ location: FileLocation::make(start, end-start), components, is_wildcard })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{ast, Parser};
    use crate::parser::ast::FileLocation;

    #[test]
    fn fqn() {
        let mut parser = Parser::new_from_str("ab.def._2.f de.titnc.*");

        assert_eq!(parser.parse_fqn().expect("FQN #1 returned err"), ast::FQN{
            is_wildcard: false,
            components: vec![("ab", 0), ("def", 3), ("_2", 7), ("f", 10)],
            location: FileLocation{start: 0, len: 11} });
        assert_eq!(parser.parse_fqn().expect("FQN #2 returned err"), ast::FQN {
            is_wildcard: true,
            components: vec![("de", 12),("titnc", 15)],
            location: FileLocation{start: 12, len: 10} })
    }

    #[test]
    fn package() {
        let mut parser = Parser::new_from_str("package ab.def._2.f");

    }

}