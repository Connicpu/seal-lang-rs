use std::collections::HashMap;
use vm::value::{Symbol, Value};

pub struct Runtime {
    pub root_module: ast::Module,
}

pub struct Scope {
    pub using: HashMap<Symbol, Value>,
    pub vars: HashMap<Symbol, Value>,
    pub consts: HashMap<Symbol, Value>,
}