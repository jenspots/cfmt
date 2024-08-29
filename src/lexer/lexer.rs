use crate::lexer::direction::Direction::{Left, Right};
use crate::lexer::token::Token::{
    Ampersand, Arrow, Bang, BangEqual, Brace, Bracket, Caret, Comma, Dot, Equal, EqualEqual,
    Greater, GreaterEqual, Identifier, Keyword, Less, LessEqual, Minus, MinusMinus, Number,
    Parenthesis, Plus, PlusPlus, Semicolon, Slash, SlashSlash, SlashStar, Star, Str, Tilde,
};
use crate::lexer::token::TokenKeyword::For;
use crate::lexer::token::{Token, TokenKeyword};

/// A stateful lexer which can be executed once, returning a stream of tokens in the process.
#[derive(Debug)]
pub struct Lexer {
    /// The source code that will be parsed by the lexer.
    source: Vec<char>,
    /// The position of the next character that needs to be parsed.
    index: usize,
}

#[derive(Debug, Clone)]
pub enum LexerError {
    /// When the lexer is invoked, but there are no symbols to parse remaining.
    EndOfFileReached,

    /// An assumption about the character stream was made, which did not hold.
    CharacterMismatch,

    /// The format of a number is invalid. For example, a floating point number with two decimal
    /// points.
    InvalidNumber,

    /// The lexer found a character which it does not know how to handle, and rather quits.
    UnknownCharacter,
}

impl Lexer {
    /// Create a new lexer for a given source file.
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            index: 0,
        }
    }

    /// Check the next character in the input stream, without advancing the lexer.
    fn peek(&self) -> Result<char, LexerError> {
        if let Some(c) = self.source.get(self.index) {
            Ok(*c)
        } else {
            Err(LexerError::EndOfFileReached)
        }
    }

    /// Check if the source file has been completely finished.
    fn finished(&self) -> bool {
        self.source.len() == self.index
    }

    /// Remove all whitespace leading up to the next readable character.
    fn trim_leading_whitespace(&mut self) -> Result<(), LexerError> {
        while let Ok(c) = self.peek() {
            if c.is_whitespace() {
                self.eat(c)?;
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Attempt to remove a specific character from the input stream.
    fn eat(&mut self, c: char) -> Result<(), LexerError> {
        if self.peek()? == c {
            self.index += 1;
            Ok(())
        } else {
            Err(LexerError::CharacterMismatch)
        }
    }

    /// Eat all remaining characters on the current line. Handy for handling comments, nothing else
    /// really.
    fn eat_line(&mut self) -> Result<String, LexerError> {
        self.eat_until('\n')
    }

    /// Attempt to eat a string literal.
    fn eat_string_literal(&mut self) -> Result<String, LexerError> {
        self.eat('"')?;
        self.eat_until('"')
    }

    /// Attempt to eat all characters until a specific character is found. Also eat that character.
    /// Note that if a character is escaped using `\` in the source code, it will be skipped.
    fn eat_until(&mut self, goal: char) -> Result<String, LexerError> {
        let mut result = String::new();
        let mut espaced = false;

        while let Ok(c) = self.peek() {
            self.eat(c)?;

            if !espaced && c == goal {
                break;
            }

            espaced = c == '\\';
            result.push(c);
        }

        Ok(result)
    }

    /// Attempt to eat a number literal.
    fn eat_number_literal(&mut self) -> Result<String, LexerError> {
        let mut result = String::new();
        let mut period_passed = false;

        while let Ok(c) = self.peek() {
            if c == '.' {
                if period_passed {
                    return Err(LexerError::InvalidNumber);
                }
                self.eat(c)?;
                period_passed = true;
                result.push(c);
            } else if c.is_numeric() {
                self.eat(c)?;
                result.push(c);
            } else {
                break;
            }
        }

        Ok(result)
    }

    /// Eat all characters which might be part of an identifier or a keyword.
    fn eat_alphanumeric(&mut self) -> Result<String, LexerError> {
        let mut result = String::new();

        while let Ok(c) = self.peek() {
            if c != '_' && !c.is_alphanumeric() {
                break;
            }

            self.eat(c)?;
            result.push(c);
        }

        Ok(result)
    }

    /// Find the next token in input stream.
    fn next_token(&mut self) -> Result<Token, LexerError> {
        self.trim_leading_whitespace()?;

        match self.peek()? {
            '+' => {
                self.eat('+')?;

                if let Ok(()) = self.eat('+') {
                    Ok(PlusPlus)
                } else {
                    Ok(Plus)
                }
            }
            '-' => {
                self.eat('-')?;

                if let Ok(()) = self.eat('-') {
                    Ok(MinusMinus)
                } else if let Ok(()) = self.eat('>') {
                    Ok(Arrow)
                } else {
                    Ok(Minus)
                }
            }
            '*' => {
                self.eat('*')?;

                Ok(Star)
            }
            '/' => {
                self.eat('/')?;

                if let Ok(()) = self.eat('/') {
                    let comment = self.eat_line()?;
                    Ok(SlashSlash(comment))
                } else if let Ok(()) = self.eat('*') {
                    Ok(SlashStar(String::new())) // TODO: Implementation
                } else {
                    Ok(Slash)
                }
            }
            '!' => {
                self.eat('!')?;

                if let Ok(()) = self.eat('=') {
                    Ok(BangEqual)
                } else {
                    Ok(Bang)
                }
            }
            '~' => {
                self.eat('~')?;
                Ok(Tilde)
            }
            '^' => {
                self.eat('^')?;
                Ok(Caret)
            }
            '=' => {
                self.eat('=')?;

                if let Ok(()) = self.eat('=') {
                    Ok(EqualEqual)
                } else {
                    Ok(Equal)
                }
            }
            '>' => {
                self.eat('>')?;

                if let Ok(()) = self.eat('=') {
                    Ok(GreaterEqual)
                } else {
                    Ok(Greater)
                }
            }
            '<' => {
                self.eat('<')?;

                if let Ok(()) = self.eat('=') {
                    Ok(LessEqual)
                } else {
                    Ok(Less)
                }
            }
            '(' => {
                self.eat('(')?;
                Ok(Parenthesis(Left))
            }
            ')' => {
                self.eat(')')?;
                Ok(Parenthesis(Right))
            }
            '{' => {
                self.eat('{')?;
                Ok(Brace(Left))
            }
            '}' => {
                self.eat('}')?;
                Ok(Brace(Right))
            }
            '[' => {
                self.eat('[')?;
                Ok(Bracket(Left))
            }
            ']' => {
                self.eat(']')?;
                Ok(Bracket(Right))
            }
            ';' => {
                self.eat(';')?;
                Ok(Semicolon)
            }
            '&' => {
                self.eat('&')?;
                Ok(Ampersand)
            }
            ',' => {
                self.eat(',')?;
                Ok(Comma)
            }
            '.' => {
                self.eat('.')?;
                Ok(Dot)
            }
            '"' => Ok(Str(self.eat_string_literal()?)),
            '0'..='9' => Ok(Number(self.eat_number_literal()?)),
            'a'..='z' | 'A'..='Z' | '_' => {
                let result = self.eat_alphanumeric()?;
                if let Some(keyword) = TokenKeyword::from(&result) {
                    Ok(Keyword(keyword))
                } else {
                    Ok(Identifier(result))
                }
            }
            _ => Err(LexerError::UnknownCharacter),
        }
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(token) => Some(Ok(token)),
            Err(LexerError::EndOfFileReached) => {
                if self.finished() {
                    None
                } else {
                    Some(Err(LexerError::EndOfFileReached))
                }
            }
            err => Some(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::Token::Number;
    use crate::lexer::token::TokenKeyword::Auto;

    #[test]
    fn empty_string() {
        let input = "".to_string();
        let expected = vec![];

        let lexer = Lexer::new(input);
        let result = lexer.collect::<Result<Vec<Token>, LexerError>>().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn single_integer() {
        let input = "24".to_string();
        let expected = vec![Number("24".to_string())];

        let lexer = Lexer::new(input);
        let result = lexer.collect::<Result<Vec<Token>, LexerError>>().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn single_float() {
        let input = "4.63".to_string();
        let expected = vec![Number("4.63".to_string())];

        let lexer = Lexer::new(input);
        let result = lexer.collect::<Result<Vec<Token>, LexerError>>().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn single_string() {
        let input = "\"Hello, World!\"".to_string();
        let expected = vec![Str("Hello, World!".to_string())];

        let lexer = Lexer::new(input);
        let result = lexer.collect::<Result<Vec<Token>, LexerError>>().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn complex_access() {
        let input = "a.b->c.d->e".to_string();
        let expected = vec![
            Identifier("a".to_string()),
            Dot,
            Identifier("b".to_string()),
            Arrow,
            Identifier("c".to_string()),
            Dot,
            Identifier("d".to_string()),
            Arrow,
            Identifier("e".to_string()),
        ];

        let lexer = Lexer::new(input);
        let result = lexer.collect::<Result<Vec<Token>, LexerError>>().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn for_loop() {
        let input = "for (int i = 0; i < n; ++i)".to_string();
        let expected = vec![
            Keyword(For),
            Parenthesis(Left),
            Identifier("int".to_string()),
            Identifier("i".to_string()),
            Equal,
            Number("0".to_string()),
            Semicolon,
            Identifier("i".to_string()),
            Less,
            Identifier("n".to_string()),
            Semicolon,
            PlusPlus,
            Identifier("i".to_string()),
            Parenthesis(Right),
        ];

        let lexer = Lexer::new(input);
        let result = lexer.collect::<Result<Vec<Token>, LexerError>>().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn maximum_munch() {
        let input = "for foreign auto automatic".to_string();
        let expected = vec![
            Keyword(For),
            Str("foreign".to_string()),
            Keyword(Auto),
            Str("auto".to_string()),
        ];
    }

    #[test]
    fn hello_world() {
        let input = "int main(int argc, char** argv) { printf(\"Hello, World!\"); }";
        let expected = vec![
            Identifier("int".to_string()),
            Identifier("main".to_string()),
            Parenthesis(Left),
            Identifier("int".to_string()),
            Identifier("argc".to_string()),
            Comma,
            Identifier("char".to_string()),
            Star,
            Star,
            Identifier("argv".to_string()),
            Parenthesis(Right),
            Brace(Left),
            Identifier("printf".to_string()),
            Parenthesis(Left),
            Str("Hello, World!".to_string()),
            Parenthesis(Right),
            Semicolon,
            Brace(Right),
        ];

        let lexer = Lexer::new(input.to_string());
        let result = lexer.collect::<Result<Vec<Token>, LexerError>>().unwrap();
        assert_eq!(expected, result);
    }
}
