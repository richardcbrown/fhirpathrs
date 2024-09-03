use serde_json::Value;

use crate::error::FhirpathError;
use crate::parser::entire_expression::EntireExpression;
use crate::parser::expression::{Expression, Term, TermExpression};
use crate::parser::identifier::{Identifier, LiteralIdentifier};
use crate::parser::invocation::{Invocation, InvocationTerm};
use crate::parser::traits::Parse;

pub type CompileResult<T> = std::result::Result<T, FhirpathError>;

pub struct ResourceNode {
    pub parent_node: Option<Box<ResourceNode>>,
    pub data: Value,
}

pub struct CompiledPath {
    expression: Box<EntireExpression>,
}

pub trait Evaluate {
    fn evaluate(&self, input: ResourceNode) -> CompileResult<ResourceNode>;
}

impl Evaluate for Invocation {
    fn evaluate(&self, input: ResourceNode) -> CompileResult<ResourceNode> {}
}

impl Evaluate for LiteralIdentifier {
    fn evaluate(&self, input: ResourceNode) -> CompileResult<ResourceNode> {
        Ok(ResourceNode {
            data: Value::String(self.text.clone()),
            parent_node: Some(Box::new(input)),
        })
    }
}

impl Evaluate for Identifier {
    fn evaluate(&self, input: ResourceNode) -> CompileResult<ResourceNode> {
        match self {
            Identifier::LiteralIdentifier(exp) => exp.evaluate(input),
        }
    }
}

impl Evaluate for InvocationTerm {
    fn evaluate(&self, input: ResourceNode) -> CompileResult<ResourceNode> {
        self.children
            .first()
            .and_then(|x| x.evaluate(input))
            .ok_or(FhirpathError::CompileError {
                msg: "Missing InvocationTerm child element".to_string(),
            })
    }
}

impl Evaluate for Term {
    fn evaluate(&self, input: ResourceNode) -> CompileResult<ResourceNode> {
        match self {
            Term::InvocationTerm(exp) => exp.evaluate(input),
        }
    }
}

impl Evaluate for TermExpression {
    fn evaluate(&self, input: ResourceNode) -> CompileResult<ResourceNode> {
        self.children
            .first()
            .and_then(|x| x.evaluate(input))
            .ok_or(FhirpathError::CompileError {
                msg: "Missing TermExpression child element".to_string(),
            })
    }
}

impl Evaluate for Expression {
    fn evaluate(&self, input: ResourceNode) -> CompileResult<ResourceNode> {
        match self {
            Expression::TermExpression(exp) => exp.evaluate(input),
            Expression::InvocationExpression(exp) => exp.evaluate(input),
        }
    }
}

impl Evaluate for EntireExpression {
    fn evaluate(&self, input: ResourceNode) -> CompileResult<ResourceNode> {
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
