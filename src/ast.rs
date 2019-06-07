use scanner::Token;
use scanner::Literal;

pub enum Exp {
    BinaryExp,
    GroupingExp,
    UnaryExp,
    LiteralExp
}

pub struct BinaryExp {
    left: Exp,
    operator: Token,
    right: Exp
}

pub struct GroupingExp {
    operator: Token
}

pub struct UnaryExp {
    exp: Exp,
    operator: Token
}

pub struct LiteralExp {
    value: Literal
}