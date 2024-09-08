use serde_json::Value;

use crate::error::FhirpathError;
use crate::parser::entire_expression::EntireExpression;
use crate::parser::expression::{Expression, Term, TermExpression};
use crate::parser::identifier::{Identifier, LiteralIdentifier};
use crate::parser::invocation::{Invocation, InvocationTerm, MemberInvocation};
use crate::parser::traits::Parse;

pub type CompileResult<T> = std::result::Result<T, FhirpathError>;

pub struct ResourceNode<'a> {
    pub parent_node: Option<Box<&'a ResourceNode<'a>>>,
    pub data: Option<Value>,
}

pub struct CompiledPath {
    expression: Box<EntireExpression>,
}

pub trait Evaluate {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>>;
}

impl Evaluate for MemberInvocation {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if input.data.is_none() {
            return Ok(ResourceNode {
                parent_node: Some(Box::new(input)),
                data: None,
            });
        }

        let key_node = self
            .children
            .first()
            .ok_or(FhirpathError::CompileError {
                msg: "MemberInvocation has no child node".to_string(),
            })?
            .evaluate(input)?;

        let key = key_node.data;

        if key.is_none() {
            return Err(FhirpathError::CompileError {
                msg: "Could not determine property to invoke".to_string(),
            });
        }

        let key_value = key.unwrap();
        let input_data = input.data.as_ref().unwrap();

        let node_resource_type = input_data.get("resourceType");

        // MemberInvocation is resourceType, so return whole resource
        if node_resource_type.is_some_and(|resource_type| resource_type.eq(&key_value)) {
            return Ok(ResourceNode {
                parent_node: Some(Box::new(input)),
                data: Some(input_data.to_owned()),
            });
        }

        // Else look for a child property of the resource that matches the key
        let child_data = input_data.get(key_value.to_string());

        Ok(ResourceNode {
            parent_node: Some(Box::new(input)),
            data: child_data.cloned(),
        })
    }
}

impl Evaluate for Invocation {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Invocation::MemberInvocation(exp) => exp.evaluate(input),
        }
    }
}

impl Evaluate for LiteralIdentifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        Ok(ResourceNode {
            data: Some(Value::String(self.text.clone())),
            parent_node: Some(Box::new(input)),
        })
    }
}

impl Evaluate for Identifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Identifier::LiteralIdentifier(exp) => exp.evaluate(input),
        }
    }
}

impl Evaluate for InvocationTerm {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        self.children
            .first()
            .and_then(|x| x.evaluate(input))
            .ok_or(FhirpathError::CompileError {
                msg: "Missing InvocationTerm child element".to_string(),
            })
    }
}

impl Evaluate for Term {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Term::InvocationTerm(exp) => exp.evaluate(input),
        }
    }
}

impl Evaluate for TermExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        self.children
            .first()
            .and_then(|x| x.evaluate(input))
            .ok_or(FhirpathError::CompileError {
                msg: "Missing TermExpression child element".to_string(),
            })
    }
}

impl Evaluate for Expression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Expression::TermExpression(exp) => exp.evaluate(input),
            Expression::InvocationExpression(exp) => exp.evaluate(input),
        }
    }
}

impl Evaluate for EntireExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        self.children.first().and_then(|x| x.evaluate(input))
    }
}

pub struct EvaluateEntireExpression {}

impl EvaluateEntireExpression {
    fn evaluate(resource: Value) -> Value {}
}

pub enum Engine {
    EntireExpression(EvaluateEntireExpression),
}

impl CompiledPath {
    fn evaluate(resource: Value) -> Value {}
}

pub fn compile(path: &String) -> CompileResult<CompiledPath> {
    Ok(CompiledPath {
        expression: EntireExpression::parse(path)?,
    })
}
