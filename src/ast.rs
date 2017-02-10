use std::collections::BTreeMap;

pub type Identifier = String;
pub type Label = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum Module {
    Root { items: Vec<Item> },
    Inline { name: Identifier, items: Vec<Item> },
    Extern { name: Identifier },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Item {
    Use(Expression),
    Extern(Identifier),
    Module(Module),
    TypeDecl(Identifier),
    TypeImpl(TypeImpl),
    Function(Function),
    Trait(Trait),
    DocComment(String),
    ModuleDocComment(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub can_error: bool,
    pub is_member: bool,
    pub body: Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeImpl {
    pub name: Identifier,
    pub interface: Option<Identifier>,
    pub methods: Vec<Function>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trait {
    pub name: Identifier,
    pub methods: Vec<TraitFunction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitFunction {
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub can_error: bool,
    pub is_member: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Statement {
    Use(Expression),
    Expression(Expression),
    Declaration(Identifier, Vec<Identifier>, Option<Expression>),
    Assignment(Expression, Vec<Expression>, AssignOp, Expression),
    IfElse(IfElse),
    Loop(Loop),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    Return(Expression, Vec<Expression>),
    Throw(Expression),
    Break(Option<Label>),
    Continue(Option<Label>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfElse {
    pub condition: Box<Expression>,
    pub if_block: Box<Block>,
    pub else_block: Option<Box<Block>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Loop {
    pub label: Option<Label>,
    pub block: Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForLoop {
    pub label: Option<Label>,
    pub bindings: Vec<Identifier>,
    pub iterator: Expression,
    pub block: Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhileLoop {
    pub label: Option<Label>,
    pub condition: Expression,
    pub block: Block,
}

#[derive(Debug, Serialize, Deserialize)]
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
    Lambda(Box<Lambda>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lambda {
    pub parameters: Vec<Identifier>,
    pub can_error: bool,
    pub is_member: bool,
    pub body: Block,
}

pub fn lambda(mut params: Vec<Identifier>, err: Option<&str>, block: Block) -> Expression {
    let mut is_member = false;
    if let Some("self") = params.first().map(|s| &s[..]) {
        params.remove(0);
        is_member = true;
    }

    Expression::Lambda(Box::new(Lambda {
        parameters: params,
        can_error: err.is_some(),
        is_member: is_member,
        body: block,
    }))
}

pub fn expr_lambda(mut params: Vec<Identifier>, err: Option<&str>, expr: Expression) -> Expression {
    let mut is_member = false;
    if let Some("self") = params.first().map(|s| &s[..]) {
        params.remove(0);
        is_member = true;
    }

    Expression::Lambda(Box::new(Lambda {
        parameters: params,
        can_error: err.is_some(),
        is_member: is_member,
        body: Block { statements: vec![Statement::Return(expr, vec![])] },
    }))
}

// listed from lowest to highest precedence
#[derive(Debug, Serialize, Deserialize)]
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
    DivRem,
    Mod,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AssignOp {
    Assign,

    LogicalOr,
    LogicalAnd,

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
    Mod,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Object(ObjectLiteral),
    Array(Vec<Expression>),
}

pub type ObjectLiteral = BTreeMap<String, Expression>;
