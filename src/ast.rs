use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Code(pub Vec<Toplevel>);

pub type Arg = (Ident, Option<Dir>, Option<i32>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Toplevel {
    Module(Ident, Vec<Arg>, Vec<Decl>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ident(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Dir {
    Input,
    Output,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Edge {
    Pos,
    Neg,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EdgeRef(pub Ident, pub Edge);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Decl {
    InnerArg(Vec<Arg>),
    Reg(Ident, Vec<Expr>),
    Wire(Ident, Vec<Expr>, Option<Expr>),
    Let(Ident, Ident, Vec<(Ident, Expr)>),
    Const(Ident, Expr),
    Always(EdgeRef, SeqBlock),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SeqBlock {
    Block(Vec<Seq>),
    Single(Box<Seq>),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum BlockType {
    Blocking,
    NonBlocking,
    Static,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Seq {
    If(Expr, SeqBlock, Option<SeqBlock>),
    Set(BlockType, Ident, Expr),
    SetIndex(BlockType, Ident, Expr, Expr),
    SetRange(BlockType, Ident, Expr, Expr, Expr),
    Match(Expr, Vec<(Vec<Expr>, SeqBlock)>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lte,
    Gte,
    And,
    Or,
    Lt,
    Gt,
    Ne,
    BinOr,
    BinAnd,
    LShift,
    RShift,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnaryOp {
    Not,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expr {
    Ref(Ident),
    Slice(Ident, Box<Expr>, Option<Box<Expr>>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Concat(Vec<Expr>),
    Repeat(Box<Expr>, Box<Expr>),
    Arith(Op, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Num(i32),
}

impl Expr {
    pub fn to_i32(&self) -> i32 {
        match *self {
            Expr::Num(value) => value,
            _ => {
                panic!("Called to_i32 on non-Num.")
            }
        }
    }
}
