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

  pub(crate) fn get_binding_value(&self, name: &str) -> Result<Value, String> {
    self
      .bindings
      .get(name)
      .cloned()
      .ok_or_else(|| format!("binding with name ‘{}’ does not exist", name))
  }
}
