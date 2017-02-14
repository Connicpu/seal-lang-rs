use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(pub usize);

pub struct SymbolTable {
    strings: Vec<String>,
    lookup: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            strings: vec![],
            lookup: HashMap::new(),
        }
    }

    pub fn intern(&mut self, key: &str) -> Symbol {
        if let Some(&sym) = self.lookup.get(key) {
            return sym;
        }

        let sym = Symbol(self.strings.len());
        self.strings.push(key.to_string());
        self.lookup.insert(key.to_string(), sym);
        sym
    }

    pub fn get(&self, sym: Symbol) -> &str {
        &self.strings[sym.0]
    }
}
