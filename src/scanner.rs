use std::str::Chars;
use std::iter::{FromIterator, Peekable};
use std::mem::discriminant;
use itertools::{Itertools, PeekingNext};


#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum Literal {
    IDENTIFIER(String),
    STRING(String),
    NUMBER(i64)
}

// TODO: TokenTypes should be split out into Unary, Binary etc for type safety
#[allow(dead_code)]
#[derive(Clone)]
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // Literals.
    Literal(Literal),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF
}
impl PartialEq for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        let self_type = discriminant(self);
        if self_type == discriminant(other) {
            return match (self, other) {
                (TokenType::Literal(Literal::IDENTIFIER(self_i_string)), TokenType::Literal(Literal::IDENTIFIER(other_i_string))) =>
                    self_i_string == other_i_string,
                (TokenType::Literal(Literal::STRING(self_s_string)), TokenType::Literal(Literal::STRING(other_s_string))) =>
                    self_s_string == other_s_string,
                // TokenType::NUMBER isn't included here, as floats don't have a well defined notion of equality.
                _ => true
            }
        }
        false
    }
}
impl Eq for TokenType {}

#[derive(Clone)]
#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize
}

// TODO MC: Add a lifetime parameter for source and make `source` reference.
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    line: usize,
    pub errors: Vec<String>
}

impl Scanner {
    pub fn new(source: String) -> Scanner
    {
        Scanner {
            source: source,
            tokens: Vec::new(),
            line: 0,
            errors: Vec::new()
        }
    }
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        // TODO: clone hack
        let temp_copy = self.source.clone();
        let mut source_iter = temp_copy.chars().peekable();
        let mut remaining_chars = source_iter.clone().count();

        // TODO: error here
        while remaining_chars > 0 {
            if let Some(token) = self.scan_token(&mut source_iter) {
                self.tokens.push(token)
            }
            remaining_chars = source_iter.clone().count();
        }

        return &self.tokens;
    }

    fn scan_token(&mut self, remaining_source: &mut Peekable<Chars>) -> Option<Token>
    {
        // TODO: Refactor this in terms of peek_next
        let next_char = remaining_source.peek().expect("Have asserted that char is there").clone();
        let token_match = match next_char {
            '(' => {
                remaining_source.next();
                Some(TokenType::LeftParen)
            },
            ')' => {
                remaining_source.next();
                Some(TokenType::RightParen)
            },
            '{' => {
                remaining_source.next();
                Some(TokenType::LeftBrace)
            },
            '}' => {
                remaining_source.next();
                Some(TokenType::RightBrace)
            },
            ',' => {
                remaining_source.next();
                Some(TokenType::COMMA)
            },
            '.' => {
                remaining_source.next();
                Some(TokenType::DOT)
            },
            '-' => {
                remaining_source.next();
                Some(TokenType::MINUS)
            },
            '+' => {
                remaining_source.next();
                Some(TokenType::PLUS)
            },
            ';' => {
                remaining_source.next();
                Some(TokenType::SEMICOLON)
            },
            '*' => {
                remaining_source.next();
                Some(TokenType::STAR)
            },
            '!' => {
                remaining_source.next();
                if remaining_source.next().expect("Have asserted that char is there") == '='
                { Some(TokenType::BangEqual) } else { Some(TokenType::BANG) }
            },
            '=' => {
                remaining_source.next();
                if remaining_source.next().expect("Have asserted that char is there") == '='
                { Some(TokenType::EqualEqual) } else { Some(TokenType::EQUAL) }
            },
            '<' => {
                remaining_source.next();
                if remaining_source.next().expect("Have asserted that char is there") == '='
                { Some(TokenType::LessEqual) } else { Some(TokenType::LESS) }
            },
            '>' => {
                remaining_source.next();
                if remaining_source.next().expect("Have asserted that char is there") == '='
                { Some(TokenType::GreaterEqual) } else { Some(TokenType::GREATER) }
            },
            '/' => {
                remaining_source.next();
                if remaining_source.next().expect("Have asserted that char is there") == '/'
                {
                    remaining_source.skip_while(|x| *x != '\n').next();
                    None
                } else { Some(TokenType::SLASH) }
            }
            ' ' => {
                remaining_source.next();
                None
            },
            '\r' => {
                remaining_source.next();
                None
            },
            '\t' => {
                remaining_source.next();
                None
            },
            '\n' => {
                remaining_source.next();
                self.line += 1;
                None
            }
            '"' => {
                remaining_source.next();
                self.scan_string(remaining_source)
            },
            character @ '0'..='9' => {
                self.scan_number(remaining_source)
            },
            _ => {
                self.scan_keyword_or_identifier(remaining_source)
            },
        };
        Some(Token {
            token_type: token_match?,
            lexeme: format!("{}", next_char), // TODO: get both identifiers
            line: self.line
        })
    }

    fn scan_string(&mut self, remaining_source: &mut Peekable<Chars>) -> Option<TokenType> {
        let string: String;
        {
            let string_iter =
                remaining_source.take_while(|x| x.ne(&'"'));
            string = String::from_iter(string_iter);
        }
        if remaining_source.peekable().peek().is_none() {
            self.errors.push(format!("Unterminated string starting on line {}", self.line));
            return None;
        }

        let newlines: String = string.matches('\n').collect();
        self.line += newlines.len();
        Some(TokenType::Literal(Literal::STRING(string)))
    }

    fn scan_number(&mut self, remaining_source: &mut Peekable<Chars>) -> Option<TokenType>
    {
        {
            let string_iter =
                remaining_source.peeking_take_while(|x| x.is_digit(10) || x.eq(&'.'));
            let string: String = string_iter.collect();
            if string.ends_with('.') {
                self.errors.push(format!("Number not permitted to end with '.' on line {}", self.line));
                return None;
            }
            match string.parse::<i64>() {
                Ok(result) => return Some(TokenType::Literal(Literal::NUMBER(result))),
                Err(_) => {
                    self.errors.push(format!("Unable to parse number on line {}", self.line));
                    return None
                }
            }
        }

        self.errors.push(format!("Unknown error parsing number on line {}", self.line));
        None
    }

    fn scan_keyword_or_identifier<I>(&mut self, remaining_source: &mut I) -> Option<TokenType>
        where I: PeekingNext<Item=char>
    {
        let string_iter =
            remaining_source.peeking_take_while(|x| x.is_alphanumeric());
        let string: String = string_iter.collect();

        match string.as_str() {
            "and" => Some(TokenType::AND),
            "class" => Some(TokenType::CLASS),
            "else" => Some(TokenType::ELSE),
            "false" => Some(TokenType::FALSE),
            "for" => Some(TokenType::FOR),
            "fun" => Some(TokenType::FUN),
            "if" => Some(TokenType::IF),
            "nil" => Some(TokenType::NIL),
            "or" => Some(TokenType::OR),
            "print" => Some(TokenType::PRINT),
            "return" => Some(TokenType::RETURN),
            "super" => Some(TokenType::SUPER),
            "this" => Some(TokenType::THIS),
            "true" => Some(TokenType::TRUE),
            "var" => Some(TokenType::VAR),
            "while" => Some(TokenType::WHILE),
            identifier => Some(TokenType::Literal(Literal::IDENTIFIER(identifier.to_string())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equals_statement()
    {
        let source = "foobar == 2";

        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        let expected = vec![
            Token{token_type: TokenType::Literal(Literal::IDENTIFIER("foobar".to_string())), lexeme: "f".to_string(), line: 0},
            Token{token_type: TokenType::EqualEqual, lexeme: "=".to_string(), line: 0},
            Token{token_type: TokenType::Literal(Literal::NUMBER(i64::from(2))), lexeme: "2".to_string(), line: 0},
        ];
        assert_eq!(&expected, tokens);
    }

    #[test]
    fn statement_with_brackets()
    {
        let source = "(foobar = 2)";

        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        let expected = vec![
            Token{token_type: TokenType::LeftParen, lexeme: "(".to_string(), line: 0},
            Token{token_type: TokenType::Literal(Literal::IDENTIFIER("foobar".to_string())), lexeme: "f".to_string(), line: 0},
            Token{token_type: TokenType::EQUAL, lexeme: "=".to_string(), line: 0},
            Token{token_type: TokenType::Literal(Literal::NUMBER(i64::from(2))), lexeme: "2".to_string(), line: 0},
            Token{token_type: TokenType::RightParen, lexeme: ")".to_string(), line: 0},
        ];
        assert_eq!(tokens, &expected);
    }
}