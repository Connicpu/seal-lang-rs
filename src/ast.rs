use std::collections::BTreeMap;

pub type Identifier = String;
pub type Label = String;

#[derive(Serialize, Deserialize)]
pub enum Module {
    Root { items: Vec<Item> },
    Inline { name: Identifier, items: Vec<Item> },
    Extern { name: Identifier },
}

#[derive(Serialize, Deserialize)]
pub enum Item {
    Function(Function),
    Module(Module),
    TypeDecl(Identifier),
    TypeImpl(Identifier, Vec<Function>),
}

#[derive(Serialize, Deserialize)]
pub struct Function {
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub can_error: bool,
    pub is_member: bool,
    pub body: Block,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Serialize, Deserialize)]
pub enum Statement {
    Use(Expression),
    Expression(Expression),
    Declaration(Identifier, Vec<Identifier>, Option<Expression>),
    Assignment(Expression, Vec<Expression>, AssignOp, Expression),
    IfElse(IfElse),
    Return(Expression, Vec<Expression>),
    Throw(Expression),
}

#[derive(Serialize, Deserialize)]
pub struct IfElse {
    pub condition: Box<Expression>,
    pub if_block: Box<Block>,
    pub else_block: Option<Box<Block>>,
}

#[derive(Serialize, Deserialize)]
pub enum Expression {
    Nil,
    Literal(Literal),
    Identifier(Identifier),
    MemberAccess(Box<Expression>, Identifier),
    IndexAccess(Box<Expression>, Box<Expression>),
    FunctionCall(Box<Expression>, Vec<Expression>),
    ObjectConstructor(Identifier, ObjectLiteral),
    BinaryOp(Box<Expression>, BinOp, Box<Expression>),
    Negate(Box<Expression>),
    Not(Box<Expression>),
    Try(Box<Expression>),
}

// listed from lowest to highest precedence
#[derive(Serialize, Deserialize)]
pub enum BinOp {
    Implements,

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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Object(ObjectLiteral),
    Array(Vec<Expression>),
}

pub type ObjectLiteral = BTreeMap<String, Expression>;
