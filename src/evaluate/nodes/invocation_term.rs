use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::invocation::InvocationTerm,
};

use super::resource_node::ResourceNode;

impl Evaluate for InvocationTerm {
    fn evaluate<'a, 'b>(&self, input: &'a ResourceNode<'a, 'b>) -> CompileResult<ResourceNode<'a, 'b>> {
        let child = self.children.first().ok_or(FhirpathError::CompileError {
            msg: "Missing InvocationTerm child element".to_string(),
        })?;

        child.evaluate(input)
    }
}

impl Text for InvocationTerm {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(""))
    }
}
