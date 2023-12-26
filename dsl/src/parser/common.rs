// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::parser::{Parser, Error, Result, ast::*, ast};
use crate::expect_identifier;

impl<'a> Parser<'a> {

    pub fn parse_package(&mut self) -> Result<ast::FQN<'a>> {
        
        // expect_identifier!(self.lexer)
    }
}