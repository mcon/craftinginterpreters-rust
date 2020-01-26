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
    fn expression(&mut self) -> Exp {
        self.equality()
    }

    // TODO: Could implement this whole parser in terms of a huge match statement... Might be simpler...
    fn execute_level(&mut self, valid_tokens : &[TokenType], previous_exp : Exp,
        current_exp_generator : &Fn(&mut Parser, &Token, Exp) -> Exp) -> Exp {
        let mut expr = previous_exp;

        fn consume_valid_tokens(instance : &mut Parser, valid_tokens : &[TokenType]) -> bool {
            if instance.current_position != instance.data.len() &&
                instance.data.index(instance.current_position).token_type.matches(valid_tokens) {
                instance.current_position += 1;
                return true
            }
            false
        }
        while (consume_valid_tokens(self, valid_tokens)) {
            let operator = self.data.index(self.current_position - 1);
            expr = current_exp_generator(self, operator, expr);
        }

        expr
    }

    fn equality(&mut self) -> Exp {
        fn next_exp_generator(instance: &mut Parser, operator: &Token, curr_expr: Exp) -> Exp {
            let right : Exp = instance.comparison();
            Exp::BinaryExp(
                BinaryExp{
                    left: Box::new(curr_expr),
                    operator: operator.clone(),
                    right: Box::new(right) })
        }
        let prev_exp = self.comparison();
        self.execute_level(
            &[TokenType::EqualEqual, TokenType::BangEqual],
            prev_exp,
            &next_exp_generator)
    }

    fn comparison(&mut self) -> Exp {
        fn next_exp_generator(instance: &mut Parser, operator: &Token, curr_expr: Exp) -> Exp {
            let right : Exp = instance.addition();
            Exp::BinaryExp(
                BinaryExp{
                    left: Box::new(curr_expr),
                    operator: operator.clone(),
                    right: Box::new(right) })
        }
        let prev_exp = self.addition();
        self.execute_level(
            &[TokenType::GREATER, TokenType::GreaterEqual, TokenType::LESS, TokenType::LessEqual],
            prev_exp,
            &next_exp_generator)
    }

    fn addition(&mut self) -> Exp {
        fn next_exp_generator(instance: &mut Parser, operator: &Token, curr_expr: Exp) -> Exp {
            let right : Exp = instance.multiplication();
            Exp::BinaryExp(
                BinaryExp{
                    left: Box::new(curr_expr),
                    operator: operator.clone(),
                    right: Box::new(right) })
        }
        let prev_exp = self.multiplication();
        self.execute_level(
            &[TokenType::MINUS, TokenType::PLUS],
            prev_exp,
            &next_exp_generator)
    }

    fn multiplication(&mut self) -> Exp {
        fn next_exp_generator(instance: &mut Parser, operator: &Token, curr_expr: Exp) -> Exp {
            let right : Exp = instance.unary();
            Exp::BinaryExp(
                BinaryExp{
                    left: Box::new(curr_expr),
                    operator: operator.clone(),
                    right: Box::new(right) })
        }
        let prev_exp = self.unary();
        self.execute_level(
            &[TokenType::SLASH, TokenType::STAR],
            prev_exp,
            &next_exp_generator)
    }

    fn unary(&mut self) -> Exp {
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

        if (consume_valid_tokens(self, valid_tokens)) {
            let operator = self.data.index(self.current_position - 1);
            let right = self.unary();
            return Exp::UnaryExp(
                    UnaryExp{
                        right: Box::new(right),
                        operator: operator.clone()})
        }

        self.primary()
    }

    fn primary(&mut self) -> Exp {
        fn consume_valid_tokens(instance : &mut Parser, valid_tokens : &[TokenType]) -> bool {
            let current = instance.data.index(instance.current_position);
            if current.token_type.matches(valid_tokens)
                && instance.current_position != instance.data.len() {
                instance.current_position += 1;
                return true
            }
            false
        }
        if consume_valid_tokens(self, &[TokenType::FALSE]) {
            return Exp::LiteralExp(LiteralExp{value: Literal::STRING("false".to_string())})
        }
        if consume_valid_tokens(self, &[TokenType::TRUE]) {
            return Exp::LiteralExp(LiteralExp{value: Literal::STRING("true".to_string())})
        }
        if consume_valid_tokens(self, &[TokenType::NIL]) {
            return Exp::LiteralExp(LiteralExp{value: Literal::STRING("null".to_string())})
        }
        // TODO: Literals not working at the moment - actually, don't even use consume_valid_tokens - do the pattern match for literal here
        if consume_valid_tokens(self, &[TokenType::Literal(Literal::STRING("".to_string()))]) {
            return Exp::LiteralExp(LiteralExp{value: Literal::STRING("null".to_string())})
        }
        // Match literals
        let current = self.data.index(self.current_position);
        match current.token_type {
            TokenType::Literal(ref literal) => {
                if self.current_position != self.data.len() {
                    self.current_position += 1;
                }
                return Exp::LiteralExp(LiteralExp{value: literal.clone() })},
            TokenType::LeftParen | TokenType::RightParen => {},
            _ => panic!("Didn't expect to get anything other than a literal or paren here")
        }

        if consume_valid_tokens(self, &[TokenType::LeftParen]) {
            let expr = self.expression();
            if consume_valid_tokens(self, &[TokenType::RightParen]) {
                return Exp::GroupingExp(GroupingExp{exp: Box::new(expr)})
            }
            panic!("Expect ')' after expression.")
        }
        panic!("No valid token")

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO MC: Actually test unary, and identifier literal - probably fine for now
    #[test]
    fn parse_valid_example_expression()
    {
        let valid_tokens = vec![
            Token{token_type: TokenType::Literal(Literal::IDENTIFIER("foobar".to_string())), lexeme: "f".to_string(), line: 0},
            Token{token_type: TokenType::EqualEqual, lexeme: "=".to_string(), line: 0},
            Token{token_type: TokenType::Literal(Literal::NUMBER(f64::from(2))), lexeme: "2".to_string(), line: 0},
        ];
        let exp = Parser::new(valid_tokens.as_ref()).expression();
    }
}