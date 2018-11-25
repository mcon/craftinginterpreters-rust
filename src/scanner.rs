use scanner::TokenParseResult::TokenFound;
use scanner::TokenParseResult::Error;
use scanner::TokenParseResult::NoToken;
use std::str::Chars;
use std::iter::FromIterator;

#[allow(dead_code)]
#[derive(Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

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
    IDENTIFIER,
    STRING (String),
    NUMBER (f64),

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}
#[warn(dead_code)]

enum TokenParseResult {
    TokenFound((TokenType, usize)),
    NoToken(usize),
    Error(String)
}


#[derive(Clone)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize
}

// TODO MC: Add a lifetime parameter for source and make `source` reference.
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    line: usize,
    errors: Vec<String>
}

impl Scanner {
    pub fn new (source: String) -> Scanner
    {
        Scanner{
            source: source,
            tokens: Vec::new(),
            line: 0,
            errors: Vec::new()
        }
    }
    pub fn scan_tokens(&mut self) {
        // TODO: clone hack
        let mut temp_copy = self.source.clone();
        let mut source_iter = temp_copy.chars();
        let mut remaining_chars = source_iter.clone().count();

        // TODO: error here
        while remaining_chars > 0 {
            if let Some(token) = self.scan_token(&mut source_iter) {
                self.tokens.push(token)
            }
            remaining_chars = source_iter.clone().count();
        }
    }

    fn scan_token(&mut self, remaining_source: &mut Chars) -> Option<Token> {
        let next_char = remaining_source.next().expect("Have asserted that char is there");
        let token_match = match next_char {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::COMMA),
            '.' => Some(TokenType::DOT),
            '-' => Some(TokenType::MINUS),
            '+' => Some(TokenType::PLUS),
            ';' => Some(TokenType::SEMICOLON),
            '*' => Some(TokenType::STAR),
            '!' => if remaining_source.next().expect("Have asserted that char is there") == '='
                {Some(TokenType::BangEqual)} else {Some(TokenType::BANG)},
            '=' => if remaining_source.next().expect("Have asserted that char is there") == '='
                {Some(TokenType::EqualEqual)} else {Some(TokenType::EQUAL)},
            '<' => if remaining_source.next().expect("Have asserted that char is there") == '='
                {Some(TokenType::LessEqual)} else {Some(TokenType::LESS)},
            '>' => if remaining_source.next().expect("Have asserted that char is there") == '='
                {Some(TokenType::GreaterEqual)} else {Some(TokenType::GREATER)},
            '/' => if remaining_source.next().expect("Have asserted that char is there") == '/'
                {
                    remaining_source.skip_while(|x| *x != '\n').next();
                    None
                } else {Some(TokenType::SLASH)}
            ' ' => None,
            '\r' => None,
            '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            '"' => self.scan_string(remaining_source),
            '0' ... '9' => self.scan_number(remaining_source),
            _ => {
                self.errors.push(format!("Unexpected identifier: {} on line {}", next_char, self.line));
                None
            }
        };
        Some(Token {
            token_type: token_match?,
            lexeme: format!("{}", next_char), // TODO: get both identifiers
            line: self.line
        })
    }

    fn scan_string(&mut self, remaining_source: &mut Chars) -> Option<TokenType> {
        let string : String;
        {
            let mut string_iter =
                remaining_source.take_while(|x| x.ne(&'"'));
            string = String::from_iter(string_iter);
        }
        if remaining_source.peekable().peek().is_none() {
            self.errors.push(format!("Unterminated string starting on line {}", self.line));
            return None;
        }

        let newlines: String =  string.matches('\n').collect();
        self.line += newlines.len();
        Some(TokenType::STRING(string))
    }

    fn scan_number(&mut self, remaining_source: &mut Chars) -> Option<TokenType> {
        {
            let mut string_iter =
                remaining_source.take_while(|x| x.is_digit(10) || x.eq(&'.'));
            let string : String = string_iter.collect();
            if string.ends_with('.') {
                self.errors.push(format!("Number not permitted to end with '.' {}", self.line));
                return None;
            }
            match string.parse::<f64>() {
                Ok(result) => return Some(TokenType::NUMBER(result)),
                Err(_) => {
                    self.errors.push(format!("Unable to parse number on line {}", self.line));
                    return None
                }
            }
        }
        if remaining_source.peekable().peek().is_none() {
            self.errors.push(format!("Unterminated number starting on line {}", self.line));
            return None;
        }

        self.errors.push(format!("Unknown error parsing number on line {}", self.line));
        None
    }
}