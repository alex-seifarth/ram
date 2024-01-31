// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileLocation {
    pub start: usize,
    pub len: usize,
}

impl FileLocation {
    pub fn make(start: usize, len: usize) -> Self {
        FileLocation{ start, len }
    }
}

/// `Model` :
///     'package' package=FQN   # package name FQN must not be wildcard
pub struct Model<'a> {
    pub package: FQN<'a>,
}

/// `FQN` :
///     Identifier ('.' Identifier)* ('.''*')?
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FQN<'a> {
    pub location: FileLocation,
    pub components: Vec<(&'a str, usize)>,
    pub is_wildcard: bool,
}