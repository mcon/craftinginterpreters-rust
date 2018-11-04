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

impl TokenType {
    fn source_from_char (source: char) -> Result<TokenType, String> {
        match source {
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            ',' => Ok(TokenType::COMMA),
            '.' => Ok(TokenType::DOT),
            '-' => Ok(TokenType::MINUS),
            '+' => Ok(TokenType::PLUS),
            ';' => Ok(TokenType::SEMICOLON),
            '*' => Ok(TokenType::STAR),
            _ => Err(format!("Token is not valid syntax: {}", source))
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