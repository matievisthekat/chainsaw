use syntax::SyntaxNode;

#[derive(Debug)]
pub struct VariableDef(SyntaxNode);

impl VariableDef {
    pub fn name(&self) -> Option<?> {
        todo!()
    }

    pub fn value(&self) -> Option<?> {
        todo!()
    }
}
