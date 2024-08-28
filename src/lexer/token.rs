use crate::lexer::direction::Direction;

/// Exhaustive list of all keywords.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenKeyword {
    If,
    Else,
    Return,
    Unsigned,
    For,
    Do,
    While,
    Goto,
    Switch,
    Case,
    Const,
    Volatile,
    External,
    Static,
    Auto,
    Struct,
    Union,
}

impl TokenKeyword {
    /// Attempt to match a string to a keyword.
    pub fn from(keyword: &str) -> Option<TokenKeyword> {
        // TODO: Check if this performant enough, or a trie/map is required.
        match keyword {
            "if" => Some(TokenKeyword::If),
            "else" => Some(TokenKeyword::Else),
            "return" => Some(TokenKeyword::Return),
            "unsigned" => Some(TokenKeyword::Unsigned),
            "for" => Some(TokenKeyword::For),
            "while" => Some(TokenKeyword::While),
            "do" => Some(TokenKeyword::Do),
            "goto" => Some(TokenKeyword::Goto),
            "switch" => Some(TokenKeyword::Switch),
            "case" => Some(TokenKeyword::Case),
            "const" => Some(TokenKeyword::Const),
            "volatile" => Some(TokenKeyword::Volatile),
            "external" => Some(TokenKeyword::External),
            "static" => Some(TokenKeyword::Static),
            "auto" => Some(TokenKeyword::Auto),
            "struct" => Some(TokenKeyword::Struct),
            "union" => Some(TokenKeyword::Union),
            &_ => None,
        }
    }
}

/// All token types used by cfmt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Star,
    Slash,
    SlashSlash(String),
    SlashStar(String),
    Bang,
    BangEqual,
    Tilde,
    Caret,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Brace(Direction),
    Parenthesis(Direction),
    Bracket(Direction),
    Semicolon,
    Ampersand,
    Comma,
    Dot,
    Arrow,
    Identifier(String),
    Number(String),
    Str(String),
    Keyword(TokenKeyword),
}
