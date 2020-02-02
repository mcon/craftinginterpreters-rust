use scanner::Token;
use scanner::Literal;
use core::borrow::{Borrow};

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub enum Exp {
    BinaryExp(BinaryExp),
    GroupingExp(GroupingExp),
    UnaryExp(UnaryExp),
    LiteralExp(LiteralExp)
}

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub struct BinaryExp {
    pub left: Box<Exp>,
    pub operator: Token,
    pub right: Box<Exp>
}

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub struct GroupingExp {
    pub exp: Box<Exp>,
}

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub struct UnaryExp {
    pub right: Box<Exp>,
    pub operator: Token
}

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub struct LiteralExp {
    pub value: Literal
}

// TODO: This should really be an implementation for something like AstBuilder of type String
pub fn ast_printer<'a>(builder: &'a mut String, exp: &'a Exp) -> &'a String {
    fn add_parens<'a>(builder: &'a mut String, name: String, exprs: Vec<&Exp>) -> &'a String {
        builder.push_str(format!("({}", name).as_str());
        for exp in exprs {
            let sub_string = &mut String::new();
            let sub_builder = ast_printer(sub_string, exp);
            builder.push_str(format!(" {}", sub_builder).as_str())
        }
        builder.push(')');
        builder
    }
    match exp {
        Exp::BinaryExp(x) => {
            add_parens(builder,x.operator.lexeme.clone(), vec![x.left.borrow(), x.right.borrow()])
        },
        Exp::GroupingExp(x) => {
            add_parens(builder,"group".to_string(), vec![x.exp.borrow()])
        },
        Exp::UnaryExp(x) => {
            add_parens(builder,x.operator.lexeme.clone(), vec![x.right.borrow()])
        },
        Exp::LiteralExp(x) => {
            builder.push_str((match x.value.clone() {
                Literal::IDENTIFIER(x) => {x},
                Literal::STRING(x) => {x},
                Literal::NUMBER(x) => {x.to_string()},
            }).as_str());
            builder
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scanner::TokenType;

    // TODO MC: Actually test unary, and identifier literal - probably fine for now
    #[test]
    fn ast_printer_simple()
    {
        let binary_exp: Exp = Exp::BinaryExp(
            BinaryExp {
                left: Box::new(Exp::GroupingExp(
                    GroupingExp{
                        exp: Box::new(Exp::LiteralExp(LiteralExp{ value: Literal::STRING("foobar".to_string()) }))
                    }
                )),
                operator: Token {
                    token_type: TokenType::EqualEqual,
                    lexeme: "==".to_string(),
                    line: 0
                },
                right: Box::new(Exp::LiteralExp(LiteralExp{value: Literal::NUMBER(2)}))});
        let mut output_string = String::new();
        let output = ast_printer(&mut output_string, &binary_exp);

        assert_eq!(*output, "(== (group foobar) 2)".to_string());
    }
}