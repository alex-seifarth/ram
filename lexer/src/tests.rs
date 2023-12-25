use crate::{Lexer, token};

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

