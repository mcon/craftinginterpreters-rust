use scanner::TokenParseResult::TokenFound;
use scanner::TokenParseResult::Error;
use scanner::TokenParseResult::NoToken;

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
    IDENTIFIER, STRING, NUMBER,

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

impl TokenType {
    // Returns the number of characters to advance over
    fn source_from_char (source: &String) -> TokenParseResult {
        if source.len() < 2 {Error(String::from("Not enough characters left to deduce token."));}
        let mut chars = source.chars();
        match chars.next().expect("Have asserted that char is there") {
            '(' => TokenFound((TokenType::LeftParen, 1)),
            ')' => TokenFound((TokenType::RightParen, 1)),
            '{' => TokenFound((TokenType::LeftBrace, 1)),
            '}' => TokenFound((TokenType::RightBrace, 1)),
            ',' => TokenFound((TokenType::COMMA, 1)),
            '.' => TokenFound((TokenType::DOT, 1)),
            '-' => TokenFound((TokenType::MINUS, 1)),
            '+' => TokenFound((TokenType::PLUS, 1)),
            ';' => TokenFound((TokenType::SEMICOLON, 1)),
            '*' => TokenFound((TokenType::STAR, 1)),
            '!' => if chars.next().expect("Have asserted that char is there") == '='
                {TokenFound((TokenType::BangEqual, 2))} else {TokenFound((TokenType::BANG, 1))},
            '=' => if chars.next().expect("Have asserted that char is there") == '='
                {TokenFound((TokenType::EqualEqual, 2))} else {TokenFound((TokenType::EQUAL, 1))},
            '<' => if chars.next().expect("Have asserted that char is there") == '='
                {TokenFound((TokenType::LessEqual, 2))} else {TokenFound((TokenType::LESS, 1))},
            '>' => if chars.next().expect("Have asserted that char is there") == '='
                {TokenFound((TokenType::GreaterEqual, 2))} else {TokenFound((TokenType::GREATER, 1))},
            '/' => if chars.next().expect("Have asserted that char is there") == '/'
                {
                    let num_comment_chars = chars.position(|x| x == '\n').expect("Expect a newline character");
                    NoToken(num_comment_chars + 2)
                } else {TokenFound((TokenType::SLASH, 1))}
            ' ' => NoToken(1),
            '\r' => NoToken(1),
            '\t' => NoToken(1),
            _ => Error(format!("Token is not valid syntax: {}", source))
        }
    }
}

#[derive(Clone)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize
}

struct Scanner {
    source: String,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new (source: String) -> Scanner
    {
        Scanner{
            source: source,
            tokens: Vec::new()
        }
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut line = 0;

        if !self.tokens.is_empty() {
            // TODO: Slight hack for now: am returning copy of tokens in stead of reference.
            return self.tokens.to_vec();
        }

        for (idx, character) in self.source.chars().enumerate() {
            match character {
                '\n' => line+=1,
                _ => {
                    let token = Scanner::scan_token(format!("{}", character).as_str(), idx);
                    self.tokens.push(token);

                }
            }
        }
        self.tokens.push(Token{token_type: TokenType::EOF, lexeme: String::new(), line: line});

        return self.tokens.to_vec();
        // TODO: Should return errors from this function as a vector to be handled by parent code - print for now.
    }

    pub fn scan_token(remaining_source: &str, line: usize) -> Token {
        let character = remaining_source.chars().next().expect("Sholudn't be at end");
        Token {
            token_type: TokenType::source_from_char(character).expect(format!("Token: '{}' on line {} should be valid", character, line).as_str()),
            lexeme: format!("{}", character),
            line: line
        }
    }
}