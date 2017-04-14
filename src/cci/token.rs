use std::str::Chars;
use std::iter::Peekable;

// enum CharType {
//     Lparen,
//     Rparen,
//     Lbrace,
//     Rbrace,
//     Lbracket,
//     Rbracket,
//     Plus,
//     Minus,
//     Multi,
//     Divi,
//     Mod,
//     Not,
//     Colon,
//     Semicolon,
//     Assign,
//     Sharp,
//     Yen,
//     Comma,
//     SngQ,
//     DblQ,
//     Void,
//     Int,
//     If,
//     Else,
//     For,
//     While,
//     Do,
//     Switch,
//     Case,
//     Equal,
//     NotEq,
//     Less,
//     LessEq,
//     Great,
//     GreatEq,
//     And,
//     Or,
//     EndKeyList,
//     Idnet,
//     IntNum,
//     String,
//     NulKind,
//     Letter,
//     Digit,
//     EofTkn,
//     Others,
// }

struct Token {
    // kind: CharType,
    value: String,
}

impl Token {
    fn next_char<'a>(&mut self, chars: &mut Peekable<Chars<'a>>) -> (Option<char>) {
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
}

#[cfg(test)]
mod tests {
    use super::Token;
    #[test]
    fn test_next_char() {
        let mut token = Token {
            // kind: CharType::Lparen,
            value: "main".to_string(),
        };
        let mut chars = "// one line comment
        /*
         * multiline comment
         */
        {
            return 0;
        }"
            .chars()
            .peekable();
        assert_eq!(Some('\n'), token.next_char(&mut chars));
        while let Some(c) = token.next_char(&mut chars) {
            if c == ' ' {
                continue;
            }
            assert_eq!('\n', c);
            break;
        }
        while let Some(c) = token.next_char(&mut chars) {
            if c == ' ' {
                continue;
            }
            assert_eq!('{', c);
            break;
        }

        while let Some(c) = token.next_char(&mut chars) {
            if c == ' ' {
                continue;
            }
            assert_eq!('\n', c);
            break;
        }
        while let Some(c) = token.next_char(&mut chars) {
            if c == ' ' {
                continue;
            }
            assert_eq!('r', c);
            break;
        }
        assert_eq!(Some('e'), token.next_char(&mut chars));
        assert_eq!(Some('t'), token.next_char(&mut chars));
        assert_eq!(Some('u'), token.next_char(&mut chars));
        assert_eq!(Some('r'), token.next_char(&mut chars));
        assert_eq!(Some('n'), token.next_char(&mut chars));
        assert_eq!(Some(' '), token.next_char(&mut chars));
        assert_eq!(Some('0'), token.next_char(&mut chars));
        assert_eq!(Some(';'), token.next_char(&mut chars));
        assert_eq!(Some('\n'), token.next_char(&mut chars));
        while let Some(c) = token.next_char(&mut chars) {
            if c == ' ' {
                continue;
            }
            assert_eq!('}', c);
            break;
        }
    }
}
