use std::{fmt::Display, str::Chars};
use phf::{Map, phf_map};
use TokenKind::*;
use KeyWord::*;

const EOF_CHAR: char = '\0';

#[derive(Copy, Clone, Debug)]
pub enum KeyWord {
    Let,
    If,
    Enum,
    Return,
    Function,
    Struct
}

/// Perfect hash. It is faster to wrap this in an Enum
/// than to store the Outer enum (TokenKind) here
const KEY_WORDS: Map<&str, KeyWord> = phf_map! {
    "fn" => Function,
    "let" => Let,
    "if" => If,
    "enum" => Enum,
    "return" => Return,
    "struct" => Struct
};

#[derive(Debug, Copy, Clone)]
pub enum TokenKind {
    Integer(u64),
    Float(f64),
    Plus,
    Minus,
    Divide,
    Multiply,
    Assign,
    Semi,
    EOF,
    Ident,
    Invalid,
    RangeUntil,
    RangeEqual,
    Dereference,
    LParen,
    RParen,
    LCurly,
    RCurly,
    Keyword(KeyWord)
}

/// Converts Token's into a helper function, not particularly exciting
impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer(v) => write!(f, "Integer: {}", v),
            Float(v) => write!(f,"Float: {}", v),
            Plus => write!(f, "Plus"),
            Minus => write!(f, "Minus"),
            EOF => write!(f, "EOF"),
            Ident => write!(f,"Ident"),
            Invalid => write!(f, "Invalid"),
            Assign => write!(f, "Assign"),
            Semi => write!(f, "SemiColon"),
            Divide => write!(f, "Divide"),
            Multiply => write!(f, "Multiply"),
            RangeUntil => write!(f, "Range, exlusive last"),
            RangeEqual => write!(f, "Range Equal"),
            Dereference => write!(f, "Dereference"),
            LParen => write!(f, "LParen"),
            RParen => write!(f, "RParen"),
            LCurly => write!(f, "LCurly"),
            RCurly => write!(f, "RCurly"),
            Keyword(val) => {
                match val {
                    Let => write!(f, "Let"),
                    If => write!(f, "If"),
                    Enum => write!(f, "Enum"),
                    Return => write!(f, "Return"),
                    Function => write!(f, "Function"),
                    Struct => write!(f, "Struct"),
                }
            }
        }
    }
}

/// A token.
/// Since this lexer does not tokenise white space,
/// keeping track of the length of the token is necessary for
/// correctness purposes
#[derive(Debug)]
pub struct Token<'a> {
    pub chars: &'a str,
    pub token_kind: TokenKind,
}

impl<'a> Token<'a> {
    #[inline(always)]
    fn new(token_kind: TokenKind, chars: &'a str) -> Token<'a> {
        Token {
            token_kind,
            chars,
        }
    }
}

/// Is this character a digit
#[inline(always)]
fn is_digit(ch: char) -> bool {
    match ch {
        '0'..= '9' | '_' => true,
        _ => false,
    }
}

/// Is this an alphanumeric character, or a _
#[inline(always)]
fn is_continuing_alpha(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

/// A Lexer.
/// It retains a reference to it's input for debugging purposes.
pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
    current: char,
    first_ahead: char,
    second_ahead: char,
    position: i64,
}

impl<'a> Lexer<'a> {
    /// Read a char and advance the position of the Lexer
    /// Separated to it's own function to semantically differentiate
    /// between advancing a peeking iterator, and advancing the
    /// state of the lexer itself
    #[inline(always)]
    fn advance(&mut self) {
        self.current = self.first_ahead;
        self.first_ahead = self.second_ahead;
        self.second_ahead = self.chars.next().unwrap_or(EOF_CHAR);
        self.position += 1;
    }

    /// Advance while a provided function which evaluates the next character
    /// evaluates to true
    #[inline(always)]
    fn eat_while(&mut self, f: fn(c: char) -> bool) {
        while f(self.first_ahead) {
            self.advance();
        }
    }

    #[inline(always)]
    fn get_pos(&self) -> usize {
        // "current" translates to previous, we want to get one step back
        self.position as usize
    }

    /// advance the iterator until the float has been fully
    /// parsed.
    ///
    /// todo: handle numbers like this 12e7, consider handling
    /// numbers like: .1, 1.
    fn read_float(&mut self) -> TokenKind {
        self.advance();
        self.advance();

        let mut is_valid = true;

        // if there's no digit here it's an invalid number
        if !self.current.is_ascii_digit() {
            // if this is alphanumeric, eat until it's not
            if self.current.is_alphanumeric() {
                self.eat_while(char::is_alphanumeric)
            }
            return Invalid;
        }
        self.eat_while(is_digit);
        // handle exponent if it exists
        if let 'e' | 'E' = self.first_ahead {
            self.advance();
            // if exponent exists, handle positive/negative exponent
            if let '-' | '+' = self.first_ahead {
                self.advance();
            }
            // for it to be a valid number another digit must
            // exist directly after
            is_valid = false;
        }

        if self.current.is_ascii_digit() {
            is_valid = true;
        }

        self.eat_while(is_digit);

        if is_valid {
            Float(0.0)
        } else {
            Invalid
        }
    }

    /// read the input string and determine if the number is an
    /// integer or a float at this stage.
    /// todo: handle binary, octal, and hex
    fn read_number(&mut self) -> Token<'a> {
        let start_pos = self.get_pos();
        let mut token_kind = Integer(0);

        self.eat_while(is_digit);
        if self.first_ahead == '.' && is_digit(self.second_ahead) {
            token_kind = self.read_float();
        }

        let len = self.get_pos() - start_pos;
        let range = start_pos..=(start_pos + len);

        let chars = &self.input[range.clone()];

        token_kind = match token_kind {
            Integer(_) => Integer(self.input[range].parse().unwrap()),
            Float(_) => Float(self.input[range].parse().unwrap()),
            Invalid => Invalid,
            _ => unreachable!(),
        };


        Token::new(token_kind, chars)
    }

    /// advance the iterator until the current char is a non
    /// whitespace character, and also not a comment
    // #[inline(always)]
    fn eat_whitespace_and_comments(&mut self) {
        loop {
            if self.current.is_whitespace() {
                self.eat_while(char::is_whitespace);
                self.advance();
            } else if self.current == '/' && self.first_ahead == '/' {
                self.eat_while(|c| c != '\n');
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Get the next token in the input.
    fn next_token(&mut self) -> Token<'a> {
        self.advance();
        self.eat_whitespace_and_comments();
        let start_pos = self.get_pos();

        match self.current {
            // read single character tokens
            '+' => Token::new(Plus, "+"),
            '-' => Token::new(Minus, "-"),
            '/' => Token::new(Divide, "/"),
            '*' => Token::new(Multiply, "*"),
            '=' => Token::new(Assign, "="),
            ';' => Token::new(Semi, ";"),
            '(' => Token::new(LParen, "("),
            ')' => Token::new(RParen, ")"),
            '{' => Token::new(LCurly, "{"),
            '}' => Token::new(RCurly, "}"),
            EOF_CHAR => Token::new(EOF, "\0"),
            '.' => {
                if self.first_ahead == '.' {
                    self.advance();
                    if self.first_ahead == '=' {
                        self.advance();
                        Token::new(RangeEqual, "..=")
                    } else {
                        Token::new(RangeUntil, "..")
                    }
                } else {
                    Token::new(Dereference, ".")
                }
            }

            // guaranteed multiple char tokens here
            '0'..= '9' => self.read_number(),
            'a'..= 'z' | 'A'..= 'Z' | '_' => {
                self.eat_while(is_continuing_alpha);
                let range = start_pos ..= self.get_pos();
                let chars = &self.input[range];
                match KEY_WORDS.get(chars) {
                    None => Token::new(Ident, chars),
                    Some(keyword) => Token::new(Keyword(*keyword), chars)
                }
            },
            _ => Token::new(Invalid, &self.input[start_pos ..= start_pos])
        }
    }

    fn new(input: &'a str) -> Self {
        let chars = input.chars();
        let mut lexer = Self {
            input,
            chars,
            current: EOF_CHAR,
            first_ahead: EOF_CHAR,
            second_ahead: EOF_CHAR,
            position: 0
        };

        lexer.advance();
        lexer.advance();

        // in order to prevent incorrect lexing this needs to be negative one
        lexer.position = -1;

        lexer
    }
}

///
pub fn tokenise<'a>(input: &'a str) -> impl Iterator<Item = Token<'a>> + 'a {
    let mut lexer = Lexer::new(input);

    std::iter::from_fn(move || {
        let next_token: Token<'a> = lexer.next_token();
        match next_token.token_kind {
            EOF => None,
            _=> Some(next_token),
        }
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    fn read_ints() {
        todo!()
    }
}
