// SPDX-License-Identifier: MPL-2.0
/*
 * Copyright (C) 2023 Alexander Seifarth
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::lexer::{Lexer, token};

macro_rules! assert_token {
    ($expression: expr, $pos: expr, $tk: pat) => {{
        let result = $expression;
        assert!(result.is_some());
        assert!(result.as_ref().unwrap().is_ok());
        let token: token::Token = result.unwrap().unwrap();
        assert_eq!(token.position, $pos);
        match token.kind {
            $tk => {},
            k => {assert!(false, "Wrong token: \n  left: {:?}\n  right: {:?}", k, stringify!($tk))}
        }
    }};
}

macro_rules! assert_token_float {
    ($expression: expr, $pos: expr, $str: expr, $val: expr) => {{
        let result = $expression;
        assert!(result.is_some());
        assert!(result.as_ref().unwrap().is_ok());
        let token: token::Token = result.unwrap().unwrap();
        assert_eq!(token.position, $pos);
        match token.kind {
            token::TokenKind::Float($str, value) => {
                assert_eq!(value, $val)
            },
            k => {
                assert!(false, "Wrong token: \n  left: {:?}\n", k)
            }
        }
    }};
}

#[test]
fn peek() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("id2 23 4223.3");

    assert_token!(lexer.peek(), 0, Identifier("id2"));
    assert_token!(lexer.peek(), 0, Identifier("id2"));
    assert_token!(lexer.next(), 0, Identifier("id2"));

    assert_token!(lexer.peek(), 4, Integer("23", 23));
    assert_token!(lexer.next(), 4, Integer("23", 23));

    assert_token_float!(lexer.peek(), 7, "4223.3", 4223.3f64);
    assert_token_float!(lexer.next(), 7, "4223.3", 4223.3f64);
    assert!(lexer.peek().is_none());
    assert!(lexer.next().is_none());
}

#[test]
fn identifier() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("id1 _name_23  \n\rAnotherId23fier \tvariable_A ");

    assert_token!(lexer.next(), 0, Identifier("id1"));
    assert_token!(lexer.next(), 4, Identifier("_name_23"));
    assert_token!(lexer.next(), 16, Identifier("AnotherId23fier"));
    assert_token!(lexer.next(), 33, Identifier("variable_A"));
    assert!(lexer.next().is_none());
}

/// checks that identifier is finished when EOF directly follows
#[test]
fn identifier_direct_end() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str(" id1");

    assert_token!(lexer.next(), 1, Identifier("id1"));
    assert!(lexer.next().is_none());
}

#[test]
fn braces() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str(" { a_name \t}{{");

    assert_token!(lexer.next(), 1, BracesLeft);
    assert_token!(lexer.next(), 3, Identifier("a_name"));
    assert_token!(lexer.next(), 11, BracesRight);
    assert_token!(lexer.next(), 12, BracesLeft);
    assert_token!(lexer.next(), 13, BracesLeft);
    assert!(lexer.next().is_none());
}

#[test]
fn exclmark_and_notequal() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("!b != ");

    assert_token!(lexer.next(), 0, ExclMark);
    assert_token!(lexer.next(), 1, Identifier("b"));

    assert_token!(lexer.next(), 3, NotEqual);
    assert!(lexer.next().is_none());
}

#[test]
fn less_and_than() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("x <= b y<c");

    assert_token!(lexer.next(), 0, Identifier("x"));
    assert_token!(lexer.next(), 2, LessThan);
    assert_token!(lexer.next(), 5, Identifier("b"));

    assert_token!(lexer.next(), 7, Identifier("y"));
    assert_token!(lexer.next(), 8, Less);
    assert_token!(lexer.next(), 9, Identifier("c"));
    assert!(lexer.next().is_none())
}

#[test]
fn greater_and_than() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("x >= b y>c");

    assert_token!(lexer.next(), 0, Identifier("x"));
    assert_token!(lexer.next(), 2, GreaterThan);
    assert_token!(lexer.next(), 5, Identifier("b"));

    assert_token!(lexer.next(), 7, Identifier("y"));
    assert_token!(lexer.next(), 8, Greater);
    assert_token!(lexer.next(), 9, Identifier("c"));
    assert!(lexer.next().is_none())
}

#[test]
fn equals_and_assign() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str(" a=b x1== _val ");

    assert_token!(lexer.next(), 1, Identifier("a"));
    assert_token!(lexer.next(), 2, Assign);
    assert_token!(lexer.next(), 3, Identifier("b"));

    assert_token!(lexer.next(), 5, Identifier("x1"));
    assert_token!(lexer.next(), 7, Equals);
    assert_token!(lexer.next(), 10, Identifier("_val"));
    assert!(lexer.next().is_none());
}

#[test]
fn arithmetic_operators() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("+ += - -= * *= / /=");

    assert_token!(lexer.next(), 0, Plus);
    assert_token!(lexer.next(), 2, Increment);
    assert_token!(lexer.next(), 5, Minus);
    assert_token!(lexer.next(), 7, Decrement);
    assert_token!(lexer.next(), 10, Star);
    assert_token!(lexer.next(), 12, Multiply);
    assert_token!(lexer.next(), 15, Slash);
    assert_token!(lexer.next(), 17, Divide);
    assert!(lexer.next().is_none());
}

#[test]
fn logical_operators() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("& && | ||");

    assert_token!(lexer.next(), 0, Ampersand);
    assert_token!(lexer.next(), 2, LogicalAnd);
    assert_token!(lexer.next(), 5, Vert);
    assert_token!(lexer.next(), 7, LogicalOr);
}

#[test]
fn single_tokens() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("%#$\"' ''?,~");

    assert_token!(lexer.next(), 0, PerCent);
    assert_token!(lexer.next(), 1, Hash);
    assert_token!(lexer.next(), 2, Dollar);
    assert_token!(lexer.next(), 3, DoubleQuote);
    assert_token!(lexer.next(), 4, SingleQuote);
    assert_token!(lexer.next(), 6, SingleQuote2);
    assert_token!(lexer.next(), 8, QuestionTag);
    assert_token!(lexer.next(), 9, Comma);
    assert_token!(lexer.next(), 10, Tilde);
}

#[test]
fn dot_and_ranges() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str(". .. ..=");

    assert_token!(lexer.next(), 0, Dot);
    assert_token!(lexer.next(), 2, Range);
    assert_token!(lexer.next(), 5, RangeIncl);
}

#[test]
fn brackets() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("[ [[ ]] ]");

    assert_token!(lexer.next(), 0, BracketLeft);
    assert_token!(lexer.next(), 2, DoubleBracketLeft);

    assert_token!(lexer.next(), 5, DoubleBracketRight);
    assert_token!(lexer.next(), 8, BracketRight);
}


#[test]
fn parentheses() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("()");

    assert_token!(lexer.next(), 0, ParensLeft);
    assert_token!(lexer.next(), 1, ParensRight);
}

#[test]
fn hex_integer() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("0x0 0Xa1 0xfff'123 0xa9'b3'66");

    assert_token!(lexer.next(), 0, Integer("0x0", 0u64));
    assert_token!(lexer.next(), 4, Integer("0Xa1", 161u64));
    assert_token!(lexer.next(), 9, Integer("0xfff'123", 0xfff123));
    assert_token!(lexer.next(), 19, Integer("0xa9'b3'66", 0xa9b366))
}

#[test]
fn hex_integer_failures() {
    let mut lexer = Lexer::new_from_str("0x 0xaf' 0x0123456789abcdef1");

    assert_eq!(lexer.next().unwrap().unwrap_err(), token::Error::IntegerNoValue("0x", 0) );
    assert_eq!(lexer.next().unwrap().unwrap_err(), token::Error::IntegerSeparatorAtEnd("0xaf'", 3));
    assert_eq!(lexer.next().unwrap().unwrap_err(), token::Error::IntegerExceedingLimit("0x0123456789abcdef1", 9));
}

#[test]
fn bin_integer() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("0b1 0B10 0b111'101 0b111'10'01");

    assert_token!(lexer.next(), 0, Integer("0b1", 1));
    assert_token!(lexer.next(), 4, Integer("0B10", 2));
    assert_token!(lexer.next(), 9, Integer("0b111'101", 0b111101));
    assert_token!(lexer.next(), 19, Integer("0b111'10'01", 0b1111001))
}

#[test]
fn bin_integer_failures() {
    let mut lexer = Lexer::new_from_str("0b 0b11'");

    assert_eq!(lexer.next().unwrap().unwrap_err(), token::Error::IntegerNoValue("0b", 0) );
    assert_eq!(lexer.next().unwrap().unwrap_err(), token::Error::IntegerSeparatorAtEnd("0b11'", 3));
}

#[test]
fn dec_integer() {
    use token::TokenKind::*;
    let mut lexer = Lexer::new_from_str("0412 242 90'800'333");

    assert_token!(lexer.next(), 0, Integer("0412", 412));
    assert_token!(lexer.next(), 5, Integer("242", 242));
    assert_token!(lexer.next(), 9, Integer("90'800'333", 90800333))
}

#[test]
fn dec_integer_failures() {
    let mut lexer = Lexer::new_from_str("6'333' 18'446'744'073'709'551'616");

    assert_eq!(lexer.next().unwrap().unwrap_err(), token::Error::IntegerSeparatorAtEnd("6'333'", 0));
    assert_eq!(lexer.next().unwrap().unwrap_err(), token::Error::IntegerExceedingLimit(
        "18'446'744'073'709'551'616", 7))
}

#[test]
fn float() {
    let mut lexer = Lexer::new_from_str("2.1418 0.82 1e22 0.34e-4 1.22e+2");

    assert_token_float!(lexer.next(), 0, "2.1418", 2.1418f64);
    assert_token_float!(lexer.next(), 7, "0.82", 0.82f64);
    assert_token_float!(lexer.next(), 12, "1e22", 1e22f64);
    assert_token_float!(lexer.next(), 17, "0.34e-4", 0.34e-4);
    assert_token_float!(lexer.next(), 25, "1.22e+2", 1.22e2);
}
