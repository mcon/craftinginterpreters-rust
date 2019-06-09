use scanner::Token;
use scanner::Literal;
use core::borrow::{Borrow, BorrowMut};

pub enum Exp {
    BinaryExp(BinaryExp),
    GroupingExp(GroupingExp),
    UnaryExp(UnaryExp),
    LiteralExp(LiteralExp)
}

pub struct BinaryExp {
    left: Box<Exp>,
    operator: Token,
    right: Box<Exp>
}

pub struct GroupingExp {
    exp: Box<Exp>,
}

pub struct UnaryExp {
    right: Box<Exp>,
    operator: Token
}

pub struct LiteralExp {
    value: Literal
}

fn ast_printer<'a>(builder: &'a mut String, exp: &'a Exp) -> &'a String {
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
                right: Box::new(Exp::LiteralExp(LiteralExp{value: Literal::NUMBER(1.5)}))});
        let mut output_string = String::new();
        let output = ast_printer(&mut output_string, &binary_exp);

        assert_eq!(*output, "(== (group foobar) 1.5)".to_string());
    }
}