mod test {
    use std::io::Cursor;

    use super::super::location::Location;
    use super::super::scanner::Scanner;
    use super::super::token::{Token, TokenKind};

    macro_rules! token {
        ($kind:expr,$lexeme:expr,$loc:expr) => {
            Token::new($kind, $lexeme, Location::from($loc))
        };
    }

    macro_rules! test {
        ($name:ident,$input:expr,$output:expr) => {
            #[test]
            fn $name() {
                let cursor = Cursor::new($input);

                let tokens = Scanner::from(cursor).scan_tokens().unwrap();

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
            token!(TokenKind::LeftParen, "(", (0, 0)),
            token!(TokenKind::RightParen, ")", (0, 1)),
            token!(TokenKind::LeftBrace, "{", (0, 2)),
            token!(TokenKind::RightBrace, "}", (0, 3)),
            token!(TokenKind::Comma, ",", (0, 4)),
            token!(TokenKind::Minus, "-", (0, 5)),
            token!(TokenKind::Dot, ".", (0, 6)),
            token!(TokenKind::Semicolon, ";", (0, 7)),
            token!(TokenKind::Plus, "+", (0, 8)),
            token!(TokenKind::Star, "*", (0, 9)),
        ]
    );

    test!(
        single_chars_repeated,
        "(){}**--,-.;+*",
        vec![
            token!(TokenKind::LeftParen, "(", (0, 0)),
            token!(TokenKind::RightParen, ")", (0, 1)),
            token!(TokenKind::LeftBrace, "{", (0, 2)),
            token!(TokenKind::RightBrace, "}", (0, 3)),
            token!(TokenKind::Star, "*", (0, 4)),
            token!(TokenKind::Star, "*", (0, 5)),
            token!(TokenKind::Minus, "-", (0, 6)),
            token!(TokenKind::Minus, "-", (0, 7)),
            token!(TokenKind::Comma, ",", (0, 8)),
            token!(TokenKind::Minus, "-", (0, 9)),
            token!(TokenKind::Dot, ".", (0, 10)),
            token!(TokenKind::Semicolon, ";", (0, 11)),
            token!(TokenKind::Plus, "+", (0, 12)),
            token!(TokenKind::Star, "*", (0, 13)),
        ]
    );

    test!(
        strings,
        "\"singleword\"",
        vec![token!(
            TokenKind::String("singleword".into()),
            "\"singleword\"",
            (0, 0)
        ),]
    );

    test!(
        strings_and_chars_and_lines,
        "var x = \"singleword\"\n{}",
        vec![
            token!(TokenKind::Var, "var", (0, 0)),
            token!(TokenKind::Identifier("x".into()), "x", (0, 4)),
            token!(TokenKind::Equal, "=", (0, 6)),
            token!(
                TokenKind::String("singleword".into()),
                "\"singleword\"",
                (0, 8)
            ),
            token!(TokenKind::LeftBrace, "{", (1, 0)),
            token!(TokenKind::RightBrace, "}", (1, 1)),
        ]
    );

    test!(
        strings_and_chars_and_lines_extended,
        "var x = \"singleword\";\nvar y = 2 + 3.;\nif (y >= 5.42 or y < 0.0000) quit();",
        vec![
            token!(TokenKind::Var, "var", (0, 0)),
            token!(TokenKind::Identifier("x".into()), "x", (0, 4)),
            token!(TokenKind::Equal, "=", (0, 6)),
            token!(
                TokenKind::String("singleword".into()),
                "\"singleword\"",
                (0, 8)
            ),
            token!(TokenKind::Semicolon, ";", (0, 20)),
            token!(TokenKind::Var, "var", (1, 0)),
            token!(TokenKind::Identifier("y".into()), "y", (1, 4)),
            token!(TokenKind::Equal, "=", (1, 6)),
            token!(TokenKind::Number(2f64), "2", (1, 8)),
            token!(TokenKind::Plus, "+", (1, 10)),
            token!(TokenKind::Number(3f64), "3.", (1, 12)),
            token!(TokenKind::Semicolon, ";", (1, 14)),
            token!(TokenKind::If, "if", (2, 0)),
            token!(TokenKind::LeftParen, "(", (2, 3)),
            token!(TokenKind::Identifier("y".into()), "y", (2, 4)),
            token!(TokenKind::GreaterEqual, ">=", (2, 6)),
            token!(TokenKind::Number(5.42f64), "5.42", (2, 9)),
            token!(TokenKind::Or, "or", (2, 14)),
            token!(TokenKind::Identifier("y".into()), "y", (2, 17)),
            token!(TokenKind::Less, "<", (2, 19)),
            token!(TokenKind::Number(0f64), "0.0000", (2, 21)),
            token!(TokenKind::RightParen, ")", (2, 27)),
            token!(TokenKind::Identifier("quit".into()), "quit", (2, 29)),
            token!(TokenKind::LeftParen, "(", (2, 33)),
            token!(TokenKind::RightParen, ")", (2, 34)),
            token!(TokenKind::Semicolon, ";", (2, 35)),
        ]
    );
}
