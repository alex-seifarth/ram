// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use std::str::Chars;

/// `ChScanner` wraps a string slice reference and provides a character iterator
/// similar to `std::str::Chars` with additional position and a marker.
#[derive(Clone, Debug)]
pub struct ChScanner<'a> {
    iter: Chars<'a>,
    peeked: Option<(Option<char>, Chars<'a>)>,
    position: usize,
    marker: Option<( Chars<'a>, usize )>
}

impl<'a> ChScanner<'a> {

    /// Creates a new initialized `ChScanner` with the given text.
    pub fn new_from_str(text: &'a str) -> Self {
        ChScanner {iter: text.chars(), peeked: None, position: 0usize, marker: None}
    }

    /// Returns the index of the next to be scanned UTF code point.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Scans the next character and returns it or `None` if the file is at its end.
    /// In contrast to `next()` this method does not consume scanned character and does not advance the position.
    /// So calling `peek()` multiple times without interleaving calls to `next()` returns the same result.
    pub fn peek(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            let iter_clone = self.iter.clone();
            self.peeked = Some(( self.iter.next(), iter_clone ))
        }
        self.peeked.as_ref().map(|(a,_)| a).unwrap().clone()
    }

    fn iter_peek_free(&self) -> Chars<'a> {
        if self.peeked.is_some() {
            self.peeked.as_ref().map(|(_,a)| a).unwrap().clone()
        } else {
            self.iter.clone()
        }
    }

    /// Sets the marker at the current position.
    pub fn set_marker(&mut self) {
        self.marker = Some((self.iter_peek_free(), self.position))
    }

    /// Clears the marker - has no effect when no marker is set.
    pub fn clr_marker(&mut self) {
        self.marker = None
    }

    /// Returns `true` when a marker is currently set - otherwise `false`.
    pub fn is_marker_set(&self) -> bool {
        self.marker.is_some()
    }

    /// Returns a string slice from the marker up to (not including) the current position.
    /// # Panics
    /// Panics when no marker is set.
    pub fn str_from_marker(&self) -> &'a str {
        assert!(self.marker.is_some());
        let ms = self.marker.as_ref().unwrap().0.as_str().as_ptr();
        let me = self.iter_peek_free().as_str().as_ptr();

        // now to some raw pointer arithmetic - everything should be ok, because the pointers
        // arise from safe operations and rust's UTF8 operations
        // -> there are safer ways to implement it, but I think for performance reasons we should do it like this,
        //    as this method is called for every identifier later in the lexer.
        unsafe {
            let size = me.offset_from(ms) as usize;
            let bytes = std::ptr::slice_from_raw_parts::<u8>(ms, size).as_ref().unwrap();
            return std::str::from_utf8_unchecked(bytes)
        }
    }

    pub fn pos_marker(&self) -> usize {
        assert!(self.marker.is_some());
        return self.marker.as_ref().map(|(_,a)| a).unwrap().clone()
    }
}

impl<'a> Iterator for ChScanner<'a> {
    type Item = char;

    /// Returns the next scanned character or `None`.
    /// This method consumer the returned character and advances the position to the next character to be scanned.
    fn next(&mut self) -> Option<Self::Item> {
        let ch =
            if self.peeked.is_some() {
                self.peeked.take().map(|(a,_)| a).unwrap()
            } else {
                self.iter.next()
            };
        if ch.is_some() {
            self.position += 1
        }
        ch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text() {
        let mut sc = ChScanner::new_from_str("");
        assert_eq!(sc.position(), 0usize);
        assert_eq!(sc.peek(), None);
        assert_eq!(sc.next(), None);
        assert_eq!(sc.position(), 0usize);
        assert_eq!(sc.peek(), None);
        assert_eq!(sc.next(), None);
    }

    #[test]
    fn string1() {
        let mut sc = ChScanner::new_from_str("abc12\u{2018}");
        assert_eq!(sc.position(), 0usize);
        assert_eq!(sc.next(), Some('a'));

        assert_eq!(sc.position(), 1usize);
        assert_eq!(sc.peek(), Some('b'));
        assert_eq!(sc.position(), 1usize);
        assert_eq!(sc.next(), Some('b'));

        assert_eq!(sc.position(), 2usize);
        assert_eq!(sc.peek(), Some('c'));
        assert_eq!(sc.position(), 2usize);
        assert_eq!(sc.peek(), Some('c'));
        assert_eq!(sc.position(), 2usize);
        assert_eq!(sc.next(), Some('c'));

        assert_eq!(sc.next(), Some('1'));
        assert_eq!(sc.next(), Some('2'));
        assert_eq!(sc.next(), Some('\u{2018}'));

        assert_eq!(sc.position(), 6);
        assert_eq!(sc.peek(), None);
        assert_eq!(sc.position(), 6);
        assert_eq!(sc.peek(), None);
        assert_eq!(sc.next(), None);
        assert_eq!(sc.position(), 6);
        assert_eq!(sc.peek(), None);
        assert_eq!(sc.next(), None);
    }

    #[test]
    fn test_marker() {
        let mut scanner = ChScanner::new_from_str("A\ntest.");
        assert_eq!(scanner.next(), Some('A'));
        assert_eq!(scanner.next(), Some('\n'));
        assert!(!scanner.is_marker_set());
        scanner.set_marker();
        assert!(scanner.is_marker_set());
        scanner.next();
        scanner.next();
        scanner.next();
        scanner.next();
        assert_eq!(scanner.str_from_marker(), "test");
    }

    #[test]
    fn test_marker_peek_end() {
        let mut scanner = ChScanner::new_from_str("A\ntest.");
        assert_eq!(scanner.next(), Some('A'));
        assert_eq!(scanner.next(), Some('\n'));
        assert!(!scanner.is_marker_set());
        scanner.set_marker();
        assert!(scanner.is_marker_set());
        scanner.next();
        scanner.next();
        scanner.next();
        scanner.next();
        scanner.peek();
        assert_eq!(scanner.str_from_marker(), "test");
    }

    #[test]
    fn test_marker_peek_start() {
        let mut scanner = ChScanner::new_from_str("A\ntest.");
        assert_eq!(scanner.next(), Some('A'));
        assert_eq!(scanner.next(), Some('\n'));
        assert!(!scanner.is_marker_set());
        scanner.set_marker();
        scanner.peek();
        assert!(scanner.is_marker_set());
        scanner.next();
        scanner.next();
        scanner.next();
        scanner.next();
        assert_eq!(scanner.str_from_marker(), "test");
    }

    #[test]
    fn test_marker_empty() {
        let mut scanner = ChScanner::new_from_str("A\ntest.");
        scanner.next();
        scanner.next();
        scanner.set_marker();
        assert_eq!(scanner.str_from_marker(), "");
    }
}
