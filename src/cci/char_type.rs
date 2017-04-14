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

trait CharTypable {
    fn char_type(&self) -> CharType;
}

impl CharTypable for char {
    fn char_type(&self) -> CharType {
        match self {
            &'{' => CharType::Lparen,
            &'}' => CharType::Rparen,
            _ => CharType::Others,
        }
    }
}
