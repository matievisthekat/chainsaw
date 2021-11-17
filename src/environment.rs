use crate::values::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Environment {
  bindings: HashMap<String, Value>,
}

impl Environment {
  pub(crate) fn store_binding(&mut self, name: String, val: Value) {
    self.bindings.insert(name, val);
  }
}
