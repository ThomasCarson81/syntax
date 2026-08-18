use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Identifier,
    Number,

    Equals,
    Plus,
    Minus,
    Asterisk,
    Slash,

    LeftBracket,
    RightBracket,

    LeftSquiggly,
    RightSquiggly,

    LetTok,
    IfTok,
    ElseTok,
    ForTok,
    WhileTok,

    Semicolon,

    EndOfFile,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Number => write!(f, "Number"),

            TokenType::Equals => write!(f, "Equals"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Asterisk => write!(f, "Asterisk"),
            TokenType::Slash => write!(f, "Slash"),

            TokenType::LeftBracket => write!(f, "LeftBracket"),
            TokenType::RightBracket => write!(f, "RightBracket"),

            TokenType::LeftSquiggly => write!(f, "LeftSquiggly"),
            TokenType::RightSquiggly => write!(f, "RightSquiggly"),

            TokenType::LetTok => write!(f, "LetTok"),
            TokenType::IfTok => write!(f, "IfTok"),
            TokenType::ElseTok => write!(f, "ElseTok"),
            TokenType::ForTok => write!(f, "ForTok"),
            TokenType::WhileTok => write!(f, "WhileTok"),

            TokenType::Semicolon => write!(f, "Semicolon"),

            TokenType::EndOfFile => write!(f, "EndOfFile"),
        }
    }
}
impl TokenType {
    pub fn from_str(s: &str) -> Option<TokenType> {
        match s {
            "Identifier" => Some(TokenType::Identifier),
            "Number" => Some(TokenType::Number),

            "Equals" => Some(TokenType::Equals),
            "Plus" => Some(TokenType::Plus),
            "Minus" => Some(TokenType::Minus),
            "Asterisk" => Some(TokenType::Asterisk),
            "Slash" => Some(TokenType::Slash),

            "LeftBracket" => Some(TokenType::LeftBracket),
            "RightBracket" => Some(TokenType::RightBracket),

            "LeftSquiggly" => Some(TokenType::LeftSquiggly),
            "RightSquiggly" => Some(TokenType::RightSquiggly),

            "LetTok" => Some(TokenType::LetTok),
            "IfTok" => Some(TokenType::IfTok),
            "ElseTok" => Some(TokenType::ElseTok),
            "ForTok" => Some(TokenType::ForTok),
            "WhileTok" => Some(TokenType::WhileTok),

            "Semicolon" => Some(TokenType::Semicolon),

            "EndOfFile" => Some(TokenType::EndOfFile),

            _ => None,
        }
    }
}
