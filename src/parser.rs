use scanner::Token;
use scanner::TokenType;
use scanner::Literal;
use ast::{Exp, BinaryExp, UnaryExp, LiteralExp, GroupingExp};
use std::ops::Index;

#[derive(Clone)]
pub struct Parser<'a> {
    data: &'a[Token],
    current_position: usize
}

impl TokenType {
    fn matches(&self, valid_tokens : &[TokenType]) -> bool {
        // Strictly speaking, a matcher function should be passed from the client in stead of doing `.contains(self)`
        valid_tokens.contains(self)
    }
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a[Token]) -> Parser<'a> {
        Parser {
            data,
            current_position: 0
        }
    }

    fn expression(&mut self) -> Result<Exp, String> {
        self.equality()
    }

    // TODO: Could implement this whole parser in terms of a huge match statement... Might be simpler...
    fn execute_level(&mut self, valid_tokens : &[TokenType], previous_exp : Exp,
        current_exp_generator : &Fn(&mut Parser, &Token, Exp) -> Result<Exp, String>) -> Result<Exp, String> {
        // TODO MC: Need to return based on whether or not the while loop is consumed, maybe return early?
        let mut expr = Ok(previous_exp);

        fn consume_valid_tokens(instance : &mut Parser, valid_tokens : &[TokenType]) -> bool {
            if instance.current_position != instance.data.len() &&
                instance.data.index(instance.current_position).token_type.matches(valid_tokens) {
                instance.current_position += 1;
                return true
            }
            false
        }
        while consume_valid_tokens(self, valid_tokens) {
            let operator = self.data.index(self.current_position - 1);
            expr = match expr {
                Ok(ex) => current_exp_generator(self, operator, ex),
                err => err
            }
        }

        expr
    }

    fn equality(&mut self) -> Result<Exp, String> {
        fn next_exp_generator(instance: &mut Parser, operator: &Token, curr_expr: Exp) -> Result<Exp, String> {
            instance.comparison().map(
              |right|  Exp::BinaryExp(
                  BinaryExp{
                      left: Box::new(curr_expr),
                      operator: operator.clone(),
                      right: Box::new(right) })
            )

        }
        match self.comparison() {
            Ok(prev_exp) => self.execute_level(
                &[TokenType::EqualEqual, TokenType::BangEqual],
                prev_exp,
                &next_exp_generator),
            err => err
        }

    }

    fn comparison(&mut self) -> Result<Exp, String> {
        fn next_exp_generator(instance: &mut Parser, operator: &Token, curr_expr: Exp) -> Result<Exp, String> {
            instance.addition().map(
                | right | Exp::BinaryExp(
                    BinaryExp{
                        left: Box::new(curr_expr),
                        operator: operator.clone(),
                        right: Box::new(right) })
            )
        }
        match self.addition() {
            Ok(prev_exp) => self.execute_level(
                &[TokenType::GREATER, TokenType::GreaterEqual, TokenType::LESS, TokenType::LessEqual],
                prev_exp,
                &next_exp_generator),
            err => err
        }
    }

    fn addition(&mut self) -> Result<Exp, String> {
        fn next_exp_generator(instance: &mut Parser, operator: &Token, curr_expr: Exp) -> Result<Exp, String> {
            instance.multiplication().map(
                |right| Exp::BinaryExp(
                    BinaryExp{
                        left: Box::new(curr_expr),
                        operator: operator.clone(),
                        right: Box::new(right) })
            )
        }
        match self.multiplication() {
            Ok(prev_exp) => self.execute_level(
                &[TokenType::MINUS, TokenType::PLUS],
                prev_exp,
                &next_exp_generator),
            err => err
        }
    }

    fn multiplication(&mut self) -> Result<Exp, String> {
        fn next_exp_generator(instance: &mut Parser, operator: &Token, curr_expr: Exp) -> Result<Exp, String> {
            instance.unary().map(
                | right | Exp::BinaryExp(
                    BinaryExp{
                        left: Box::new(curr_expr),
                        operator: operator.clone(),
                        right: Box::new(right) })
            )
        }
        match self.unary() {
            Ok(prev_exp) => self.execute_level(
                &[TokenType::SLASH, TokenType::STAR],
                prev_exp,
                &next_exp_generator),
            err => err
        }
    }

    fn unary(&mut self) -> Result<Exp, String> {
        let valid_tokens = &[TokenType::BANG, TokenType::MINUS];
        // TODO: If this whole scheme works, then make consume_valid_tokens a top level function and re-use
        fn consume_valid_tokens(instance : &mut Parser, valid_tokens : &[TokenType]) -> bool {
            let current = instance.data.index(instance.current_position);
            if current.token_type.matches(valid_tokens)
                && instance.current_position != instance.data.len() {
                instance.current_position += 1;
                return true
            }
            false
        }

        if consume_valid_tokens(self, valid_tokens) {
            let operator = self.data.index(self.current_position - 1);
            return self.unary().map(
                | right | Exp::UnaryExp(
                    UnaryExp{
                        right: Box::new(right),
                        operator: operator.clone()})
            )
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Exp, String> {
        fn consume_valid_tokens<'a>(instance : &mut Parser, valid_tokens : &'a [TokenType]) -> Option<&'a Token> {
            let current = instance.data.index(instance.current_position);
            if current.token_type.matches(valid_tokens)
                && instance.current_position != instance.data.len() {
                instance.current_position += 1;
                return Some(current)
            }
            None
        }
        if consume_valid_tokens(self, &[TokenType::FALSE]).is_some() {
            return Ok(Exp::LiteralExp(LiteralExp{value: Literal::STRING("false".to_string())}))
        }
        if consume_valid_tokens(self, &[TokenType::TRUE]).is_some() {
            return Ok(Exp::LiteralExp(LiteralExp{value: Literal::STRING("true".to_string())}))
        }
        if consume_valid_tokens(self, &[TokenType::NIL]).is_some() {
            return Ok(Exp::LiteralExp(LiteralExp{value: Literal::STRING("null".to_string())}))
        }
        // TODO: Fix up literals by taking the token out of the Option retuned by consume_valid_tokens
        if consume_valid_tokens(self, &[TokenType::Literal(Literal::STRING("".to_string()))]) {
            return Ok(Exp::LiteralExp(LiteralExp{value: Literal::STRING("null".to_string())}))
        }
        // Match literals
        let current = self.data.index(self.current_position);
        match current.token_type {
            TokenType::Literal(ref literal) => {
                if self.current_position != self.data.len() {
                    self.current_position += 1;
                }
                return Ok(Exp::LiteralExp(LiteralExp{value: literal.clone() }))},
            TokenType::LeftParen => {
                if consume_valid_tokens(self, &[TokenType::LeftParen]) {
                    let expr = self.expression();
                    if consume_valid_tokens(self, &[TokenType::RightParen]) {
                        return expr.map(|ex| Exp::GroupingExp(GroupingExp{exp: Box::new(ex)}))
                    }
                    return Err("Expect ')' after expression.".to_string())
                }
                return Err("No valid token".to_string()) // TODO: Unlear why this needs to be String not &str
            },
            TokenType::RightParen => panic!("Did not expect to reach this branch of match statement"),
            _ => Err("Didn't expect to get anything other than a literal or paren here".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::ast_printer;

    // TODO MC: Actually test unary, and identifier literal - probably fine for now
    #[test]
    fn parse_valid_example_expression()
    {
        let valid_tokens = vec![
            Token{token_type: TokenType::Literal(Literal::IDENTIFIER("foobar".to_string())), lexeme: "f".to_string(), line: 0},
            Token{token_type: TokenType::EQUAL, lexeme: "=".to_string(), line: 0},
            Token{token_type: TokenType::Literal(Literal::NUMBER(i64::from(2))), lexeme: "2".to_string(), line: 0},
        ];
        let expected_exp: Exp = Exp::BinaryExp(
            BinaryExp {
                left: Box::new(Exp::LiteralExp(LiteralExp{ value: Literal::STRING("foobar".to_string()) })
                ),
                operator: Token {
                    token_type: TokenType::EQUAL,
                    lexeme: "=".to_string(),
                    line: 0
                },
                right: Box::new(Exp::LiteralExp(LiteralExp{value: Literal::NUMBER(2)}))});
        let exp_result = Parser::new(valid_tokens.as_ref()).expression();
        match exp_result {
            // TODO: Verify that the expression is valid
            Ok(exp) => {
                assert_eq!(exp, expected_exp)
            },
            Err(err) => panic!(err)
        }
    }
}