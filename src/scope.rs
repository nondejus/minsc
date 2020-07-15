use std::collections::HashMap;

use crate::ast::Ident;
use crate::error::{Error, Result};
use crate::miniscript::attach_builtins;
use crate::runtime::Value;

#[derive(Default, Debug)]
pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    local: HashMap<Ident, Value>,
}

impl<'a> Scope<'a> {
    pub fn root() -> Self {
        let mut scope = Self::default();
        attach_builtins(&mut scope);
        scope
    }

    pub fn get<T: AsRef<str>>(&self, key: T) -> Option<&Value> {
        self.local
            .get(key.as_ref())
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(key)))
    }

    pub fn set<T: Into<Ident>>(&mut self, key: T, value: Value) -> Result<()> {
        let key = key.into();
        if self.local.contains_key(&key) {
            // cannot be set if already exists in this scope, but could shadow over a definition from a parent scope
            Err(Error::AssignedVariableExists(key))
        } else {
            self.local.insert(key, value);
            Ok(())
        }
    }

    pub fn child(&'a self) -> Self {
        Scope {
            parent: Some(&self),
            local: HashMap::new(),
        }
    }
}
