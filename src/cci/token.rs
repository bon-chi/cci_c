use std::str::Chars;
use std::iter::Peekable;
use super::char_type::CharType;
use super::char_type::CharTypable;

#[derive(Debug)]
#[derive(PartialEq)]
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
    EndKeyList,
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
    intValue: Option<i32>,
    text: Option<String>,
}

fn next_char<'a>(chars: &mut Peekable<Chars<'a>>) -> (Option<char>) {
    match chars.next() {
        None => None,
        Some(c) if c == '/' => {
            match chars.peek() {
                Some(&'/') => {
                    chars.next();
                    while let Some(c) = chars.next() {
                        if c == '\n' {
                            break;
                        }
                    }
                    Some('\n')
                }
                Some(&'*') => {
                    chars.next();
                    while let Some(c) = chars.next() {
                        if c == '*' {
                            if let Some(c) = chars.next() {
                                if c == '/' {
                                    return Some(' ');
                                }
                            }
                        }
                    }
                    None
                }
                Some(_) | None => Some('/'),
            }
        }
        Some(c) => Some(c),
    }
}

fn next_token<'a>(chars: &mut Peekable<Chars<'a>>) -> (Option<Token>) {
    while let Some(c) = next_char(chars) {
        if c == ' ' {
            continue;
        }
        match c.char_type() {
            CharType::Letter => {
                // while let Some(c) = next_char(chars) {
                // }
            }
            CharType::Digit => {
                let mut num = c.to_digit(10).unwrap() as i32;
                while let Some(c) = next_char(chars) {
                    match c.char_type() {
                        CharType::Digit => {
                            num = num * 10 + (c.to_digit(10).unwrap() as i32);
                        }
                        _ => {
                            break;
                        }
                    }
                }
                return Some(Token {
                    kind: TokenKind::IntNum,
                    intValue: Some(num),
                    text: None,
                });
            }
            CharType::SngQ => {}
            CharType::DblQ => {}
            _ => {}
        }
    }
    Some(Token {
        kind: TokenKind::Letter,
        intValue: None,
        text: Some("abc".to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::next_char;
    use super::next_token;
    use super::TokenKind;
    #[test]
    fn text_next_token() {
        let mut chars = "12 3 456 78".chars().peekable();
        let token = next_token(&mut chars).unwrap();
        assert_eq!(token.intValue, Some(12));
        assert_eq!(token.kind, TokenKind::IntNum);
        let token = next_token(&mut chars).unwrap();
        assert_eq!(token.intValue, Some(3));
        let token = next_token(&mut chars).unwrap();
        assert_eq!(token.intValue, Some(456));
        let token = next_token(&mut chars).unwrap();
        assert_eq!(token.intValue, Some(78));
    }
    fn test_next_char() {
        let mut chars = "// one line comment
        /*
         * multiline comment
         */
        \
                         {
            return 0;
        }"
            .chars()
            .peekable();
        assert_eq!(Some('\n'), next_char(&mut chars));
        while let Some(c) = next_char(&mut chars) {
            if c == ' ' {
                continue;
            }
            assert_eq!('\n', c);
            break;
        }
        while let Some(c) = next_char(&mut chars) {
            if c == ' ' {
                continue;
            }
            assert_eq!('{', c);
            break;
        }

        while let Some(c) = next_char(&mut chars) {
            if c == ' ' {
                continue;
            }
            assert_eq!('\n', c);
            break;
        }
        while let Some(c) = next_char(&mut chars) {
            if c == ' ' {
                continue;
            }
            assert_eq!('r', c);
            break;
        }
        assert_eq!(Some('e'), next_char(&mut chars));
        assert_eq!(Some('t'), next_char(&mut chars));
        assert_eq!(Some('u'), next_char(&mut chars));
        assert_eq!(Some('r'), next_char(&mut chars));
        assert_eq!(Some('n'), next_char(&mut chars));
        assert_eq!(Some(' '), next_char(&mut chars));
        assert_eq!(Some('0'), next_char(&mut chars));
        assert_eq!(Some(';'), next_char(&mut chars));
        assert_eq!(Some('\n'), next_char(&mut chars));
        while let Some(c) = next_char(&mut chars) {
            if c == ' ' {
                continue;
            }
            assert_eq!('}', c);
            break;
        }
    }
}
