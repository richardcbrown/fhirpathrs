use crate::{
    evaluate::{CompileResult, Evaluate, Text},
    parser::expression::InvocationExpression,
};

use super::resource_node::ResourceNode;

impl Evaluate for InvocationExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        self.children.iter().fold(Ok(input.clone()), |acc, child| {
            acc.and_then(|val| {
                let result = child.evaluate(&val);

                match result {
                    Ok(res) => {
                        let mut node = ResourceNode::from_node(input, res.data);

                        node.path = res.path.clone();
                        node.fhir_types = res.fhir_types.clone();

                        Ok(node)
                    }
                    Err(err) => Err(err),
                }
            })
        })
    }
}

impl Text for InvocationExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join("."))
    }
}
