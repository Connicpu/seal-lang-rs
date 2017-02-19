use ast;
use std::collections::HashMap;
use std::rc::Rc;
use vm::runtime as rt;
pub use vm::value::object::Object;
use vm::value::shared::{Shared, SharedRef};
use vm::value::simd::SimdValue;
pub use vm::value::sym::Symbol;

pub mod sym;
pub mod object;
pub mod simd;
pub mod shared;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValueKey {
    Integer(i64),
    String(Rc<String>),
    Symbol(Symbol),
    Shared(SharedRef),
}

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Integer(i64),
    Float(f64),
    String(Rc<String>),
    Symbol(Symbol),
    Object(Shared<Object>),
    Table(Shared<HashMap<ValueKey, Value>>),
    Array(Shared<Vec<Value>>),
    Simd(Box<SimdValue>),
    PlainFunction(Rc<ast::Function>),
    ExternFunction(Shared<rt::Function>), 

    // TODO: Closures
}

impl ValueKey {
    pub fn create(value: &Value) -> Result<ValueKey, &'static str> {
        use self::Value::*;
        match *value {
            Integer(i) => Ok(ValueKey::Integer(i)),
            String(ref s) => Ok(ValueKey::String(s.clone())),
            Symbol(s) => Ok(ValueKey::Symbol(s)),

            Object(ref o) => Ok(ValueKey::Shared(o.clone().into())),
            Table(ref t) => Ok(ValueKey::Shared(t.clone().into())),
            Array(ref a) => Ok(ValueKey::Shared(a.clone().into())),

            Nil => Err("nil cannot be used as a table key"),
            Float(_) => Err("floats cannot be used as a table key"),
            Simd(_) => Err("SIMD values cannot be used as a table key"),
            PlainFunction(_) => Err("Functions may not be used as table keys"),
        }
    }
}
