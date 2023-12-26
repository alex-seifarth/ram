// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// `Model` :
///     'package' package=FQN   # package name FQN must not be wildcard
pub struct Model<'a> {
    pub package: FQN<'a>,
}

/// `FQN` :
///     Identifier ('.' Identifier)* ('.''*')?
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FQN<'a> {
    pub components: Vec<&'a str>,
    pub is_wildcard: bool,
}