use std::collections::BTreeMap;

pub type Identifier = String;
pub type Label = String;

#[derive(Debug)]
pub enum Module {
    Root { items: Vec<Item> },
    Inline { name: Identifier, items: Vec<Item> },
    Extern { name: Identifier },
}

#[derive(Debug)]
pub enum Item {
    Function(Function),
    Module(Module),
    TypeDecl(Identifier),
    TypeImpl(Identifier, Vec<Function>),
}

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub can_error: bool,
    pub is_member: bool,
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
    Assignment(Expression, AssignOp, Expression),
    IfElse(IfElse),
    Return(Expression),
    Throw(Expression),
}

#[derive(Debug)]
pub struct IfElse {
    pub condition: Box<Expression>,
    pub if_block: Box<Block>,
    pub else_block: Option<Box<Block>>,
}

#[derive(Debug)]
pub enum Expression {
    Nil,
    Literal(Literal),
    Identifier(Identifier),
    MemberAccess(Box<Expression>, Identifier),
    FunctionCall(Box<Expression>, Vec<Expression>),
    ObjectConstructor(Identifier, ObjectLiteral),
    BinaryOp(Box<Expression>, BinOp, Box<Expression>),
    Negate(Box<Expression>),
    Not(Box<Expression>),
    Try(Box<Expression>),
}

// listed from lowest to highest precedence
#[derive(Debug)]
pub enum BinOp {
    RangeExclusive,
    RangeInclusive,

    LogicalOr,

    LogicalAnd,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,

    BitOr,
    BitXor,
    BitAnd,

    LShiftLeft,
    LShiftRight,
    AShiftRight,

    Add,
    Sub,

    Mul,
    Div,
    Rem,
}

#[derive(Debug)]
pub enum AssignOp {
    Assign,

    BitOr,
    BitXor,
    BitAnd,

    LShiftLeft,
    LShiftRight,
    AShiftRight,

    Add,
    Sub,

    Mul,
    Div,
    Rem,
}

#[derive(Debug)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Object(ObjectLiteral),
    Array(Vec<Expression>),
}

pub type ObjectLiteral = BTreeMap<String, Expression>;
