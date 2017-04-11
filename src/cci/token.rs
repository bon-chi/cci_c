use std::fs;
use std::str::Chars;
// use std::io::{BufReader, Read, Chars};
use std::io::{BufReader, Read};
use std::iter::Peekable;

enum CharType {
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
    kind: CharType,
    string: String,
}

impl Token {
    fn next_char<'a>(&mut self,
                     mut chars: Peekable<Chars<'a>>)
                     -> (Option<char>, Peekable<Chars<'a>>) {
        match chars.next() {
            None => (None, chars),
            Some(c) if c == '/' => {
                match chars.peek() {
                    Some(&'/') => {
                        chars.next();
                        while let Some(c) = chars.next() {
                            if c == '\n' {
                                break;
                            }
                        }
                        (Some('\n'), chars)
                    }
                    Some(&'*') => {
                        chars.next();
                        while let Some(c) = chars.next() {
                            if c == '*' {
                                if let Some(c) = chars.next() {
                                    if c == '/' {
                                        return (Some(' '), chars);
                                    }
                                }
                            }
                        }
                        (None, chars)
                    }
                    Some(_) | None => (Some('/'), chars),
                }
            }
            Some(c) => (Some(c), chars),
        }
    }
}
