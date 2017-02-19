use ast;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;
use vm::value::Value;
use vm::value::sym::Symbol;

#[derive(Clone, Debug)]
pub struct Object {
    pub metatype: Option<Rc<MetaType>>,
    pub fields: ObjectFields,
}

#[derive(Clone, Debug)]
pub struct MetaType {
    pub name: Symbol,
    pub inherent_type: TypeImpl,
    pub trait_impls: HashMap<Symbol, TypeImpl>,
}

#[derive(Clone, Debug)]
pub struct TraitDef {
    pub name: Symbol,
    pub constants: HashSet<Symbol>,
    pub static_methods: HashMap<Symbol, Rc<ast::TraitFunction>>,
    pub member_methods: HashMap<Symbol, Rc<ast::TraitFunction>>,
    pub default_impl: Option<TypeImpl>,
}

#[derive(Clone, Debug)]
pub struct TypeImpl {
    pub name: Symbol,
    pub interface: Option<Symbol>,
    pub constants: HashMap<Symbol, Value>,
    pub static_methods: HashMap<Symbol, Rc<ast::Function>>,
    pub member_methods: HashMap<Symbol, Rc<ast::Function>>,
}

#[derive(Clone, Debug)]
pub enum ObjectFields {
    Tree(BTreeMap<Symbol, Value>),
    Hash(HashMap<Symbol, Value>),
}

impl ObjectFields {
    pub fn new(hash: bool) -> Self {
        if hash {
            ObjectFields::Hash(HashMap::new())
        } else {
            ObjectFields::Tree(BTreeMap::new())
        }
    }

    pub fn get(&self, key: Symbol) -> Value {
        match *self {
            ObjectFields::Tree(ref tree) => tree.get(&key).cloned().unwrap_or(Value::Nil),
            ObjectFields::Hash(ref hash) => hash.get(&key).cloned().unwrap_or(Value::Nil),
        }
    }

    pub fn set(&mut self, key: Symbol, value: Value) {
        match *self {
            ObjectFields::Tree(ref mut tree) => {
                tree.insert(key, value);
            }
            ObjectFields::Hash(ref mut hash) => {
                hash.insert(key, value);
            }
        }
    }
}