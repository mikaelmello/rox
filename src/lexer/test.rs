mod test {
    use std::io::Cursor;

    use super::super::location::Location;
    use super::super::scanner::Scanner;
    use super::super::token::{Token, TokenType};

    macro_rules! token {
        ($type:expr,$lexeme:expr) => {
            token!($type, $lexeme, Location::default())
        };

        ($type:expr,$lexeme:expr,$loc:expr) => {
            Token::new($type, $lexeme, Location::from($loc))
        };
    }

    macro_rules! test {
        ($name:ident,$input:expr,$output:expr) => {
            #[test]
            fn $name() {
                let cursor = Cursor::new($input);

                let tokens = Scanner::from(cursor)
                    .and_then(Scanner::scan_tokens)
                    .unwrap();

                for (idx, t) in tokens.0.iter().enumerate() {
                    let ot = &$output[idx];
                    assert_eq!(ot, t);
                }
            }
        };
    }

    test!(
        single_chars,
        "(){},-.;+*",
        vec![
            token!(TokenType::LeftParen, "(", (0, 0)),
            token!(TokenType::RightParen, ")", (0, 1)),
            token!(TokenType::LeftBrace, "{", (0, 2)),
            token!(TokenType::RightBrace, "}", (0, 3)),
            token!(TokenType::Comma, ",", (0, 4)),
            token!(TokenType::Minus, "-", (0, 5)),
            token!(TokenType::Dot, ".", (0, 6)),
            token!(TokenType::Semicolon, ";", (0, 7)),
            token!(TokenType::Plus, "+", (0, 8)),
            token!(TokenType::Star, "*", (0, 9)),
        ]
    );

    test!(
        single_chars_repeated,
        "(){}**--,-.;+*",
        vec![
            token!(TokenType::LeftParen, "(", (0, 0)),
            token!(TokenType::RightParen, ")", (0, 1)),
            token!(TokenType::LeftBrace, "{", (0, 2)),
            token!(TokenType::RightBrace, "}", (0, 3)),
            token!(TokenType::Star, "*", (0, 4)),
            token!(TokenType::Star, "*", (0, 5)),
            token!(TokenType::Minus, "-", (0, 6)),
            token!(TokenType::Minus, "-", (0, 7)),
            token!(TokenType::Comma, ",", (0, 8)),
            token!(TokenType::Minus, "-", (0, 9)),
            token!(TokenType::Dot, ".", (0, 10)),
            token!(TokenType::Semicolon, ";", (0, 11)),
            token!(TokenType::Plus, "+", (0, 12)),
            token!(TokenType::Star, "*", (0, 13)),
        ]
    );

    test!(
        strings,
        "\"singleword\"",
        vec![token!(
            TokenType::String("singleword".into()),
            "\"singleword\"",
            (0, 0)
        ),]
    );

    test!(
        strings_and_chars_and_lines,
        "var x = \"singleword\"\n{}",
        vec![
            token!(TokenType::Var, "var", (0, 0)),
            token!(TokenType::Identifier("x".into()), "x", (0, 4)),
            token!(TokenType::Equal, "=", (0, 6)),
            token!(
                TokenType::String("singleword".into()),
                "\"singleword\"",
                (0, 8)
            ),
            token!(TokenType::LeftBrace, "{", (1, 0)),
            token!(TokenType::RightBrace, "}", (1, 1)),
        ]
    );

    test!(
        strings_and_chars_and_lines_extended,
        "var x = \"singleword\";\nvar y = 2 + 3.;\nif (y >= 5.42 or y < 0.0000) quit();",
        vec![
            token!(TokenType::Var, "var", (0, 0)),
            token!(TokenType::Identifier("x".into()), "x", (0, 4)),
            token!(TokenType::Equal, "=", (0, 6)),
            token!(
                TokenType::String("singleword".into()),
                "\"singleword\"",
                (0, 8)
            ),
            token!(TokenType::Semicolon, ";", (0, 20)),
            token!(TokenType::Var, "var", (1, 0)),
            token!(TokenType::Identifier("y".into()), "y", (1, 4)),
            token!(TokenType::Equal, "=", (1, 6)),
            token!(TokenType::Number(2f64), "2", (1, 8)),
            token!(TokenType::Plus, "+", (1, 10)),
            token!(TokenType::Number(3f64), "3.", (1, 12)),
            token!(TokenType::Semicolon, ";", (1, 14)),
            token!(TokenType::If, "if", (2, 0)),
            token!(TokenType::LeftParen, "(", (2, 3)),
            token!(TokenType::Identifier("y".into()), "y", (2, 4)),
            token!(TokenType::GreaterEqual, ">=", (2, 6)),
            token!(TokenType::Number(5.42f64), "5.42", (2, 9)),
            token!(TokenType::Or, "or", (2, 14)),
            token!(TokenType::Identifier("y".into()), "y", (2, 17)),
            token!(TokenType::Less, "<", (2, 19)),
            token!(TokenType::Number(0f64), "0.0000", (2, 21)),
            token!(TokenType::RightParen, ")", (2, 27)),
            token!(TokenType::Identifier("quit".into()), "quit", (2, 29)),
            token!(TokenType::LeftParen, "(", (2, 33)),
            token!(TokenType::RightParen, ")", (2, 34)),
            token!(TokenType::Semicolon, ";", (2, 35)),
        ]
    );
}
