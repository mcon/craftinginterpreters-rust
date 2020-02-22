use ast::{Exp, BinaryExp, GroupingExp, UnaryExp, LiteralExp};
use scanner::{Literal, Token, TokenType};
use std::iter::FromIterator;

#[derive(Eq, PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(i64),
    String(String),
}

enum MatchedValues {
    Nil,
    Boolean(bool, bool),
    Number(i64, i64),
    String(String, String),
}

fn match_items(l : Result<Value, String>, r : Result<Value, String>)
               -> Result<MatchedValues, String> {
    match (l, r) {
        (Ok(Value::Number(l_num)), Ok(Value::Number(r_num))) => Ok(MatchedValues::Number(l_num, r_num)),
        (Ok(Value::String(l_str)), Ok(Value::String(r_str))) => Ok(MatchedValues::String(l_str, r_str)),
        (Ok(Value::Boolean(l_bool)), Ok(Value::Boolean(r_bool))) => Ok(MatchedValues::Boolean(l_bool, r_bool)),
        (Ok(Value::Nil), Ok(Value::Nil)) => Ok(MatchedValues::Nil),
        (Ok(l_value), Ok(r_value)) =>
            Err(format!("Both sides of value must be the same type: {:?}, {:?}", l_value, r_value)),
        (Ok(l_other), Err(r_other)) => Err(r_other),
        (Err(l_other), Ok(_)) => Err(l_other),
        (Err(l_other), Err(_)) => Err(l_other),
    }
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => {false},
            Value::Boolean(bool_value) => {*bool_value},
            Value::Number(_) => {true},
            Value::String(_) => {true},
        }
    }

    fn is_equal(&self, other : &Value) -> bool {
        match_items(Ok(self.clone()), Ok(other.clone()))
            .map_or_else(|_| false, |x| match x {
                MatchedValues::Nil => {true},
                MatchedValues::Boolean(l, r) => {l == r},
                MatchedValues::Number(l, r) => {l == r},
                MatchedValues::String(l, r) => {l == r},
            } )
    }
}

pub fn interpret(exp : &Exp) -> Result<Value, String> {
    match exp {
        Exp::BinaryExp(bin_exp) => interpret_binary(bin_exp),
        Exp::GroupingExp(grouping_exp) => interpret_grouping(grouping_exp),
        Exp::UnaryExp(unary_exp) => interpret_unary(unary_exp),
        Exp::LiteralExp(literal_exp) => interpret_literal(literal_exp),
    }
}

fn interpret_binary(exp : &BinaryExp) -> Result<Value, String> {
    let right = interpret(exp.right.as_ref());
    let left = interpret(exp.left.as_ref());

    fn match_numbers(l : Result<Value, String>, r : Result<Value, String>)
        -> Result<(i64, i64), String> {
        match_items(l, r)
            .and_then(|x| match x {
                MatchedValues::Number(l, r) => Ok((l, r)),
                x => {Err("Non-number values not supported with operator -".to_string())},
            }
        )
    }

    // TODO: Work out whether or not using &str in stead of String is more appropriate / efficient
    match &exp.operator {
        Token { token_type: TokenType::MINUS, lexeme, line } => {
            match_numbers(left, right)
                .map(|(l, r)| Value::Number(l - r))
        },
        Token { token_type: TokenType::SLASH, lexeme, line } => {
            match_numbers(left, right)
                .map(|(l, r)| Value::Number(l / r))
        },
        Token { token_type: TokenType::STAR, lexeme, line } => {
            match_numbers(left, right)
                .map(|(l, r)| Value::Number(l * r))
        },
        Token { token_type: TokenType::GREATER, lexeme, line } => {
            match_numbers(left, right)
                .map(|(l, r)| Value::Boolean(l > r))
        },
        Token { token_type: TokenType::GreaterEqual, lexeme, line } => {
            match_numbers(left, right)
                .map(|(l, r)| Value::Boolean(l >= r))
        },
        Token { token_type: TokenType::LESS, lexeme, line } => {
            match_numbers(left, right)
                .map(|(l, r)| Value::Boolean(l < r))
        },
        Token { token_type: TokenType::LessEqual, lexeme, line } => {
            match_numbers(left, right)
                .map(|(l, r)| Value::Boolean(l <= r))
        },
        Token { token_type: TokenType::PLUS, lexeme, line } => {
            match_items(left, right)
                .and_then(|x| match x {
                    MatchedValues::Number(l, r) => Ok(Value::Number(l + r)),
                    MatchedValues::String(l, r) => {
                        Ok(Value::String(l + r.as_str()))},
                    x => {
                        Err("Only numbers and strings are supported for operator +".to_string())}
                })
        },
        Token { token_type: TokenType::BangEqual, lexeme, line } => {
            let items : Result<Vec<Value>, _> = [left, right].iter().cloned().collect();
            items.map(|x| Value::Boolean(!x[0].is_equal(&x[1])))
        },
        Token { token_type: TokenType::EqualEqual, lexeme, line } => {
            let items : Result<Vec<Value>, _> = [left, right].iter().cloned().collect();
            items.map(|x| Value::Boolean(!x[0].is_equal(&x[1])))
        },
        Token { token_type, lexeme, line } =>
            Err(format!("Unknown TokenType for binary expression: {:?}, line: {}", token_type, line)),
    }
}

fn interpret_grouping(exp : &GroupingExp) -> Result<Value, String> {
    interpret(exp.exp.as_ref())
}

fn interpret_unary(exp : &UnaryExp) -> Result<Value, String> {
    let right = interpret(exp.right.as_ref());
    match &exp.operator {
        Token { token_type: TokenType::BANG, lexeme: _, line: _ } => {
            right.map(|x| Value::Boolean(!x.is_truthy()))
        },
        Token { token_type: TokenType::MINUS, lexeme: _, line: _ } => {
            right.and_then(|x| match x {
                Value::Number(value) => {Ok(Value::Number(-value))},
                other => {Err(format!("Minus can't be used with this value: {:?}", other))},
            })
        },
        Token { token_type, lexeme: _, line } =>
            panic!("Unknown TokenType for unary: {:?}, line: {}", token_type, line),
    }
}

fn interpret_literal(exp : &LiteralExp) -> Result<Value, String> {
    match &exp.value {
        Literal::IDENTIFIER(_) => {panic!("Identifier not yet supported")},
        Literal::STRING(str_literal) => {Ok(Value::String(str_literal.clone()))},
        Literal::NUMBER(num_literal) => {Ok(Value::Number(*num_literal))},
    }
}