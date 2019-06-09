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

fn ast_printer<'a>(builder: &'a mut String, exp: &Exp) -> &'a String {
    // TODO MC: Commonize parantheses function without borrow checker being sad.
    match exp {
        Exp::BinaryExp(x) => {
            builder.push_str(format!("({} ", x.operator.lexeme).as_str());
            ast_printer(builder, x.left.borrow());
            builder.push(' ');
            ast_printer(builder, x.right.borrow());
            builder.push(')');
            builder
        },
        Exp::GroupingExp(x) => {
            builder.push_str("(group ");
            ast_printer(builder, x.exp.borrow());
            builder.push(')');
            builder
        },
        Exp::UnaryExp(x) => {
            builder.push_str(format!("({} ", x.operator.lexeme).as_str());
            ast_printer(builder, x.right.borrow());
            builder.push(')');
            builder
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

    #[test]
    fn ast_printer_simple()
    {
        let binary_exp = Exp::BinaryExp(
            BinaryExp {
                left: Box::new(Exp::LiteralExp(LiteralExp{value: Literal::NUMBER(1.2)})),
                operator: Token {
                    token_type: TokenType::EqualEqual,
                    lexeme: "==".to_string(),
                    line: 0
                },
                right: Box::new(Exp::LiteralExp(LiteralExp{value: Literal::NUMBER(1.5)}))});
        let mut output_string = String::new();
        let output = ast_printer(&mut output_string, &binary_exp);

        assert_eq!(*output, "(== 1.2 1.5)".to_string());
    }
}