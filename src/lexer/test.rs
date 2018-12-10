use super::*;
use crate::token;
#[test]
fn test_next_token() {
    let input = AsciiString::from_ascii(
        r###"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            "###
            .to_string(),
    )
        .unwrap();

    #[derive(Debug)]
    struct Expected {
        pub expected_type: token::TokenType,
        pub expected_literal: &'static str,
    }

    impl Expected {
        fn new(expected_type: token::TokenType, expected_literal: &'static str) -> Expected {
            Expected {
                expected_type,
                expected_literal,
            }
        }
    }

    let expected_results = [
        Expected::new(token::LET, "let"),
        Expected::new(token::IDENT, "five"),
        Expected::new(token::ASSIGN, "="),
        Expected::new(token::INT, "5"),
        Expected::new(token::SEMICOLON, ";"),
        Expected::new(token::LET, "let"),
        Expected::new(token::IDENT, "ten"),
        Expected::new(token::ASSIGN, "="),
        Expected::new(token::INT, "10"),
        Expected::new(token::SEMICOLON, ";"),
        Expected::new(token::LET, "let"),
        Expected::new(token::IDENT, "add"),
        Expected::new(token::ASSIGN, "="),
        Expected::new(token::FUNCTION, "fn"),
        Expected::new(token::LPAREN, "("),
        Expected::new(token::IDENT, "x"),
        Expected::new(token::COMMA, ","),
        Expected::new(token::IDENT, "y"),
        Expected::new(token::RPAREN, ")"),
        Expected::new(token::LBRACE, "{"),
        Expected::new(token::IDENT, "x"),
        Expected::new(token::PLUS, "+"),
        Expected::new(token::IDENT, "y"),
        Expected::new(token::SEMICOLON, ";"),
        Expected::new(token::RBRACE, "}"),
        Expected::new(token::SEMICOLON, ";"),
        Expected::new(token::LET, "let"),
        Expected::new(token::IDENT, "result"),
        Expected::new(token::ASSIGN, "="),
        Expected::new(token::IDENT, "add"),
        Expected::new(token::LPAREN, "("),
        Expected::new(token::IDENT, "five"),
        Expected::new(token::COMMA, ","),
        Expected::new(token::IDENT, "ten"),
        Expected::new(token::RPAREN, ")"),
        Expected::new(token::SEMICOLON, ";"),
        Expected::new(token::BANG, "!"),
        Expected::new(token::MINUS, "-"),
        Expected::new(token::SLASH, "/"),
        Expected::new(token::ASTERISK, "*"),
        Expected::new(token::INT, "5"),
        Expected::new(token::SEMICOLON, ";"),
        Expected::new(token::INT, "5"),
        Expected::new(token::LT, "<"),
        Expected::new(token::INT, "10"),
        Expected::new(token::GT, ">"),
        Expected::new(token::INT, "5"),
        Expected::new(token::SEMICOLON, ";"),
        Expected::new(token::IF, "if"),
        Expected::new(token::LPAREN, "("),
        Expected::new(token::INT, "5"),
        Expected::new(token::LT, "<"),
        Expected::new(token::INT, "10"),
        Expected::new(token::RPAREN, ")"),
        Expected::new(token::LBRACE, "{"),
        Expected::new(token::RETURN, "return"),
        Expected::new(token::TRUE, "true"),
        Expected::new(token::SEMICOLON, ";"),
        Expected::new(token::RBRACE, "}"),
        Expected::new(token::ELSE, "else"),
        Expected::new(token::LBRACE, "{"),
        Expected::new(token::RETURN, "return"),
        Expected::new(token::FALSE, "false"),
        Expected::new(token::SEMICOLON, ";"),
        Expected::new(token::RBRACE, "}"),
        Expected::new(token::INT, "10"),
        Expected::new(token::EQ, "=="),
        Expected::new(token::INT, "10"),
        Expected::new(token::SEMICOLON, ";"),
        Expected::new(token::INT, "10"),
        Expected::new(token::NOT_EQ, "!="),
        Expected::new(token::INT, "9"),
        Expected::new(token::SEMICOLON, ";"),
        Expected::new(token::EOF, "\0"),
    ];

    let mut l = Lexer::new(input);

    for expected in expected_results.iter() {
        let tok = l.next_token();

        assert_eq!(tok.token_type, expected.expected_type);
        assert_eq!(tok.literal, expected.expected_literal);
    }
}
