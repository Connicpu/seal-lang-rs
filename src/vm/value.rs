use ast;
use simd::f32x4;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Integer(i64),
    Float(f64),
    SimdFloat(Shared<f32x4>),
    String(Shared<String>),
    ConstString(Rc<String>),
    PlainFunction(Rc<ast::Function>),
}

#[derive(Clone, Debug)]
pub struct Object {
    metatype: Option<Rc<MetaType>>,
    fields: ObjectFields,
}

#[derive(Clone, Debug)]
pub struct Shared<T> {
    inner: Rc<RefCell<T>>,
}

#[derive(Clone, Debug)]
pub struct MetaType {
    name: Rc<String>,
    static_methods: Vec<Rc<ast::Function>>,
    member_methods: Vec<Rc<ast::Function>>,
}

#[derive(Clone, Debug)]
pub enum ObjectFields {
    Tree(BTreeMap<String, Value>),
    Hash(HashMap<String, Value>),
}

impl ObjectFields {
    pub fn new(hash: bool) -> Self {
        if hash {
            ObjectFields::Hash(HashMap::new())
        } else {
            ObjectFields::Tree(BTreeMap::new())
        }
    }

    pub fn get(&self, key: &str) -> Value {
        match *self {
            ObjectFields::Tree(ref tree) => tree.get(key).cloned().unwrap_or(Value::Nil),
            ObjectFields::Hash(ref hash) => hash.get(key).cloned().unwrap_or(Value::Nil),
        }
    }

    pub fn set(&mut self, key: Cow<str>, value: Value) {
        match *self {
            ObjectFields::Tree(ref mut tree) => {
                if let Some(slot) = tree.get_mut(key.as_ref(): &str) {
                    *slot = value;
                    return;
                }

                tree.insert(key.into_owned(), value);
            }
            ObjectFields::Hash(ref mut hash) => {
                if let Some(slot) = hash.get_mut(key.as_ref(): &str) {
                    *slot = value;
                    return;
                }

                hash.insert(key.into_owned(), value);
            }
        }
    }
}
