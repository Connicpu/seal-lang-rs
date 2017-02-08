use std::collections::BTreeMap;

pub type Identifier = String;
pub type Label = String;

#[derive(Debug)]
pub struct Module {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Function(Function),
}

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub can_error: bool,
    pub body: Block,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Declaration(Identifier, Option<Expression>),
    Assignment(Identifier, Expression),
    IfElse(IfElse),
}

#[derive(Debug)]
pub struct IfElse {
    pub condition: Box<Expression>,
    pub if_block: Box<Block>,
    pub else_block: Option<Box<Block>>,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    MemberAccess(Box<Expression>, Identifier),
    FunctionCall(Box<Expression>, Vec<Expression>),
}

#[derive(Debug)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Object(BTreeMap<String, Expression>),
    Array(Vec<Expression>),
}
