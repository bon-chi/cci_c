use std::fs;
use std::io::{BufReader, Read, Chars};

enum TokenKind {
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Plus,
    Minus,
    Multi,
    Divi,
    Mod,
    Not,
    Colon,
    Semicolon,
    Assign,
    Sharp,
    Yen,
    Comma,
    SngQ,
    DblQ,
    Void,
    Int,
    If,
    Else,
    For,
    While,
    Do,
    Switch,
    Case,
    Equal,
    NotEq,
    Less,
    LessEq,
    Great,
    GreatEq,
    And,
    Or,
    END_KeyList,
    Idnet,
    IntNum,
    String,
    NulKind,
    Letter,
    Digit,
    EofTkn,
    Others,
}

struct Token {
    kind: TokenKind,
    string: String,
}

impl Token {
    fn next_char<T: Read>(&mut self, mut chars: Chars<T>) -> (Option<char>, Chars<T>) {
        let hoge = match chars.next() {
            None => 1,
            Some(c) if c == '/' => {
                match chars.next() {
                    Some('/') => {}
                    Some('*') => {}
                    Some(_) | None => {}
                }
            }
        };

        (Some('c'), chars)
    }
}
