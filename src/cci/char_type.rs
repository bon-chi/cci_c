#[derive(Debug)]
#[derive(PartialEq)]
pub enum CharType {
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
    Less,
    Great,
    Letter,
    Digit,
    Others,
}

pub trait CharTypable {
    fn char_type(&self) -> CharType;
}

impl CharTypable for char {
    fn char_type(&self) -> CharType {
        match self {
            &'(' => CharType::Lparen,
            &')' => CharType::Rparen,
            &'{' => CharType::Lbrace,
            &'}' => CharType::Rbrace,
            &'[' => CharType::Lbracket,
            &']' => CharType::Rbracket,
            &'+' => CharType::Plus,
            &'-' => CharType::Minus,
            &'*' => CharType::Multi,
            &'/' => CharType::Divi,
            &'%' => CharType::Mod,
            &'!' => CharType::Not,
            &':' => CharType::Colon,
            &';' => CharType::Semicolon,
            &'=' => CharType::Assign,
            &'#' => CharType::Sharp,
            &'\\' => CharType::Yen,
            &',' => CharType::Comma,
            &'\'' => CharType::SngQ,
            &'"' => CharType::DblQ,
            &'<' => CharType::Less,
            &'>' => CharType::Great,
            &'a'...'z' => CharType::Letter,
            &'A'...'Z' => CharType::Letter,
            &'0'...'9' => CharType::Digit,
            _ => CharType::Others,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CharTypable;
    use super::CharType;
    #[test]
    fn test_char_type() {
        assert_eq!('('.char_type(), CharType::Lparen);
        assert_eq!('\''.char_type(), CharType::SngQ);
        assert_eq!('\\'.char_type(), CharType::Yen);
        assert_eq!('a'.char_type(), CharType::Letter);
        assert_eq!('z'.char_type(), CharType::Letter);
        assert_eq!('A'.char_type(), CharType::Letter);
        assert_eq!('Z'.char_type(), CharType::Letter);
        assert_eq!('0'.char_type(), CharType::Digit);
        assert_eq!('9'.char_type(), CharType::Digit);
    }
}
