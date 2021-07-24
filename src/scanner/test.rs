mod test {
    use crate::location::Location;
    use crate::scanner::token::TokenErrorKind;
    use crate::scanner::{Scanner, Token, TokenKind};

    macro_rules! token {
        ($kind:expr,$lexeme:expr,$start:expr,$end:expr) => {
            Token::new($kind, $lexeme, Location::from($start), Location::from($end))
        };
    }

    macro_rules! test {
        ($name:ident,$input:expr,$output:expr) => {
            #[test]
            fn $name() {
                let scanner = Scanner::new($input);

                for (idx, t) in scanner.into_iter().enumerate() {
                    assert!(
                        idx < $output.len(),
                        "Output has unexpected token {}",
                        t.kind()
                    );
                    let ot = &$output[idx];
                    assert_eq!(*ot, t);
                }
            }
        };
    }

    test!(
        single_chars,
        "(){},-.;+*",
        vec![
            token!(TokenKind::LeftParen, "(", (0, 0, 0), (1, 0, 1)),
            token!(TokenKind::RightParen, ")", (1, 0, 1), (2, 0, 2)),
            token!(TokenKind::LeftBrace, "{", (2, 0, 2), (3, 0, 3)),
            token!(TokenKind::RightBrace, "}", (3, 0, 3), (4, 0, 4)),
            token!(TokenKind::Comma, ",", (4, 0, 4), (5, 0, 5)),
            token!(TokenKind::Minus, "-", (5, 0, 5), (6, 0, 6)),
            token!(TokenKind::Dot, ".", (6, 0, 6), (7, 0, 7)),
            token!(TokenKind::Semicolon, ";", (7, 0, 7), (8, 0, 8)),
            token!(TokenKind::Plus, "+", (8, 0, 8), (9, 0, 9)),
            token!(TokenKind::Star, "*", (9, 0, 9), (10, 0, 10)),
            token!(TokenKind::Eof, "", (10, 0, 10), (10, 0, 10)),
        ]
    );

    test!(
        single_chars_repeated,
        "(){}**--,-.;+*",
        vec![
            token!(TokenKind::LeftParen, "(", (0, 0, 0), (1, 0, 1)),
            token!(TokenKind::RightParen, ")", (1, 0, 1), (2, 0, 2)),
            token!(TokenKind::LeftBrace, "{", (2, 0, 2), (3, 0, 3)),
            token!(TokenKind::RightBrace, "}", (3, 0, 3), (4, 0, 4)),
            token!(TokenKind::Star, "*", (4, 0, 4), (5, 0, 5)),
            token!(TokenKind::Star, "*", (5, 0, 5), (6, 0, 6)),
            token!(TokenKind::Minus, "-", (6, 0, 6), (7, 0, 7)),
            token!(TokenKind::Minus, "-", (7, 0, 7), (8, 0, 8)),
            token!(TokenKind::Comma, ",", (8, 0, 8), (9, 0, 9)),
            token!(TokenKind::Minus, "-", (9, 0, 9), (10, 0, 10)),
            token!(TokenKind::Dot, ".", (10, 0, 10), (11, 0, 11)),
            token!(TokenKind::Semicolon, ";", (11, 0, 11), (12, 0, 12)),
            token!(TokenKind::Plus, "+", (12, 0, 12), (13, 0, 13)),
            token!(TokenKind::Star, "*", (13, 0, 13), (14, 0, 14)),
            token!(TokenKind::Eof, "", (14, 0, 14), (14, 0, 14)),
        ]
    );

    test!(
        strings,
        "\"singleword\"",
        vec![
            token!(TokenKind::String, "\"singleword\"", (0, 0, 0), (12, 0, 12)),
            token!(TokenKind::Eof, "", (12, 0, 12), (12, 0, 12)),
        ]
    );

    test!(
        unterminated_string,
        "\"singleword\n\n",
        vec![
            token!(
                TokenKind::Error(TokenErrorKind::UnterminatedString),
                "\"singleword\n\n",
                (0, 0, 0),
                (11, 0, 11)
            ),
            token!(TokenKind::Eof, "", (11, 0, 11), (11, 0, 11)),
        ]
    );

    test!(
        strings_and_chars_and_lines,
        "var x = \"singleword\"\n{}",
        vec![
            token!(TokenKind::Var, "var", (0, 0, 0), (3, 0, 3)),
            token!(TokenKind::Identifier, "x", (4, 0, 4), (5, 0, 5)),
            token!(TokenKind::Equal, "=", (6, 0, 6), (7, 0, 7)),
            token!(TokenKind::String, "\"singleword\"", (8, 0, 8), (20, 0, 20)),
            token!(TokenKind::LeftBrace, "{", (21, 1, 0), (22, 1, 1)),
            token!(TokenKind::RightBrace, "}", (22, 1, 1), (23, 1, 2)),
            token!(TokenKind::Eof, "", (23, 1, 2), (23, 1, 2)),
        ]
    );

    test!(
        strings_and_chars_and_lines_extended,
        "var x = \"singleword\";\nvar y = 2 + 32;\nif (y >= 5.42 or y < 0.0000) quit();",
        vec![
            token!(TokenKind::Var, "var", (0, 0, 0), (3, 0, 3)),
            token!(TokenKind::Identifier, "x", (4, 0, 4), (5, 0, 5)),
            token!(TokenKind::Equal, "=", (6, 0, 6), (7, 0, 7)),
            token!(TokenKind::String, "\"singleword\"", (8, 0, 8), (20, 0, 20)),
            token!(TokenKind::Semicolon, ";", (20, 0, 20), (21, 0, 21)),
            token!(TokenKind::Var, "var", (22, 1, 0), (25, 1, 3)),
            token!(TokenKind::Identifier, "y", (26, 1, 4), (27, 1, 5)),
            token!(TokenKind::Equal, "=", (28, 1, 6), (29, 1, 7)),
            token!(TokenKind::Number, "2", (30, 1, 8), (31, 1, 9)),
            token!(TokenKind::Plus, "+", (32, 1, 10), (33, 1, 11)),
            token!(TokenKind::Number, "32", (34, 1, 12), (36, 1, 14)),
            token!(TokenKind::Semicolon, ";", (36, 1, 14), (37, 1, 15)),
            token!(TokenKind::If, "if", (38, 2, 0), (40, 2, 2)),
            token!(TokenKind::LeftParen, "(", (41, 2, 3), (42, 2, 4)),
            token!(TokenKind::Identifier, "y", (42, 2, 4), (43, 2, 5)),
            token!(TokenKind::GreaterEqual, ">=", (44, 2, 6), (46, 2, 8)),
            token!(TokenKind::Number, "5.42", (47, 2, 9), (51, 2, 13)),
            token!(TokenKind::Or, "or", (52, 2, 14), (54, 2, 16)),
            token!(TokenKind::Identifier, "y", (55, 2, 17), (56, 2, 18)),
            token!(TokenKind::Less, "<", (57, 2, 19), (58, 2, 20)),
            token!(TokenKind::Number, "0.0000", (59, 2, 21), (65, 2, 27)),
            token!(TokenKind::RightParen, ")", (65, 2, 27), (66, 2, 28)),
            token!(TokenKind::Identifier, "quit", (67, 2, 29), (71, 2, 33)),
            token!(TokenKind::LeftParen, "(", (71, 2, 33), (72, 2, 34)),
            token!(TokenKind::RightParen, ")", (72, 2, 34), (73, 2, 35)),
            token!(TokenKind::Semicolon, ";", (73, 2, 35), (74, 2, 36)),
            token!(TokenKind::Eof, "", (74, 2, 36), (74, 2, 36)),
        ]
    );
}
