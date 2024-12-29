mod equality;
mod filtering;
mod invocation_table;
mod math;
mod strings;
mod subsetting;
mod utils;

use std::ops::Deref;

use invocation_table::invocation_table;
use serde_json::{json, Number, Value};

use crate::error::FhirpathError;
use crate::parser::entire_expression::EntireExpression;
use crate::parser::expression::{
    AdditiveExpression, EqualityExpression, Expression, ExpressionAndInvocation, IndexerExpression,
    InvocationExpression, PolarityExpression, Term, TermExpression,
};
use crate::parser::identifier::{Identifier, LiteralIdentifier};
use crate::parser::invocation::{
    FunctionInvocation, IdentifierAndParamList, Invocation, InvocationTerm, MemberInvocation,
    ParamList,
};
use crate::parser::literal::{Literal, LiteralTerm, NumberLiteral, StringLiteral};
use crate::parser::traits::Parse;

pub type CompileResult<T> = std::result::Result<T, FhirpathError>;

#[derive(Clone)]
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

impl Evaluate for StringLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        Ok(ResourceNode {
            parent_node: Some(Box::new(input)),
            data: Some(json!(self.text)),
        })
    }
}

impl Evaluate for NumberLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let value = self
            .text
            .parse::<i64>()
            .map_err(|_| FhirpathError::ParserError {
                msg: "NumberLiteral is not a Number".to_string(),
            })?;

        Ok(ResourceNode {
            parent_node: Some(Box::new(input)),
            data: Some(json!(value)),
        })
    }
}

impl Evaluate for Literal {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Literal::BooleanLiteral(exp) => todo!(),
            Literal::DatetimeLiteral(exp) => todo!(),
            Literal::NullLiteral(exp) => todo!(),
            Literal::NumberLiteral(exp) => exp.evaluate(input),
            Literal::QuantityLiteral(exp) => todo!(),
            Literal::StringLiteral(exp) => exp.evaluate(input),
            Literal::TimeLiteral(exp) => todo!(),
        }
    }
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

        let key_value = match key.unwrap() {
            Value::String(str) => str,
            _ => "".to_string(),
        };

        let input_data = input.data.as_ref().unwrap();

        let node_resource_type = input_data.get("resourceType");

        // MemberInvocation is resourceType, so return whole resource
        if node_resource_type.is_some_and(|resource_type| resource_type.eq(&key_value)) {
            return Ok(ResourceNode {
                parent_node: Some(Box::new(input)),
                data: Some(input_data.to_owned()),
            });
        }

        print!("data");
        println!("{}", input_data);
        println!("{:?}", &key_value.to_string());

        // Else look for a child property of the resource that matches the key
        let child_data = match input_data {
            Value::Object(obj) => obj.get(&key_value.to_string()).cloned(),
            Value::Array(array) => {
                let values: Vec<Value> = array
                    .to_owned()
                    .into_iter()
                    .filter_map(|item| item.get(&key_value.to_string()).cloned())
                    .collect();

                dbg!(&values);

                let mut flattened_values: Vec<Value> = vec![];

                values.iter().for_each(|val| match val {
                    Value::Array(array_val) => flattened_values.append(&mut array_val.clone()),
                    val => flattened_values.push(val.clone()),
                });

                Some(Value::Array(flattened_values))
            }
            _ => None,
        };

        println!("{:?}", child_data);

        Ok(ResourceNode {
            parent_node: Some(Box::new(input)),
            data: child_data,
        })
    }
}

impl Evaluate for ParamList {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        todo!()
    }
}

impl Evaluate for FunctionInvocation {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        // @TODO - Child of FunctionInvocation should be "Functn" in js fhirpath
        let function_name_child = self.children[0].deref();
        let param_list_child = if self.children.len() == 2 {
            Some(self.children[1].deref())
        } else {
            None
        };

        let identifier = match function_name_child {
            IdentifierAndParamList::Identifier(identifier) => Ok(identifier),
            _ => Err(FhirpathError::CompileError {
                msg: "First child of FunctionInvocation should be function name".to_string(),
            }),
        }?;

        let default_params =
            IdentifierAndParamList::ParamList(Box::new(ParamList { children: vec![] }));

        let param_list = param_list_child.unwrap_or(&default_params);

        let parameters = match param_list {
            IdentifierAndParamList::ParamList(param_list) => Ok(&param_list.children),
            _ => Err(FhirpathError::CompileError {
                msg: "Second child of FunctionInvocation should be function params".to_string(),
            }),
        }?;

        let function_name = identifier
            .evaluate(input)?
            .data
            .and_then(|val| match val {
                Value::String(string) => Some(string),
                _ => None,
            })
            .ok_or(FhirpathError::CompileError {
                msg: "First child of FunctionInvocation should be function name".to_string(),
            })?;

        let invocation = invocation_table()
            .get(&function_name)
            .ok_or(FhirpathError::CompileError {
                msg: "No method in invocation table".to_string(),
            })?
            .clone();

        Ok(invocation(input, &parameters)?)
    }
}

impl Evaluate for Invocation {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Invocation::MemberInvocation(exp) => exp.evaluate(input),
            Invocation::FunctionInvocation(exp) => exp.evaluate(input),
            _ => todo!(),
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
            _ => todo!(),
        }
    }
}

impl Evaluate for InvocationTerm {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let child = self.children.first().ok_or(FhirpathError::CompileError {
            msg: "Missing InvocationTerm child element".to_string(),
        })?;

        child.evaluate(input)
    }
}

impl Evaluate for LiteralTerm {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 1 {
            return Err(FhirpathError::CompileError {
                msg: "LiteralTerm should have exactly one child".to_string(),
            });
        }

        let first = &self.children[0];

        Ok(ResourceNode {
            parent_node: Some(Box::new(input)),
            data: first.evaluate(input)?.data,
        })
    }
}

impl Evaluate for Term {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Term::InvocationTerm(exp) => exp.evaluate(input),
            Term::LiteralTerm(exp) => exp.evaluate(input),
            Term::ExternalConstantTerm(exp) => todo!(),
            Term::ParenthesizedTerm(exp) => todo!(),
        }
    }
}

impl Evaluate for TermExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let child = self.children.first().ok_or(FhirpathError::CompileError {
            msg: "Missing TermExpression child element".to_string(),
        })?;

        child.evaluate(input)
    }
}

impl Evaluate for ExpressionAndInvocation {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            ExpressionAndInvocation::Expression(expr) => expr.evaluate(input),
            ExpressionAndInvocation::Invocation(invocation) => invocation.evaluate(input),
        }
    }
}

impl Evaluate for InvocationExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        self.children.iter().fold(Ok(input.clone()), |acc, child| {
            acc.and_then(|val| {
                let result = child.evaluate(&val);

                match result {
                    Ok(res) => Ok(ResourceNode {
                        parent_node: Some(Box::new(input)),
                        data: res.data,
                    }),
                    Err(err) => Err(err),
                }
            })
        })
    }
}

fn invoke_operation<'a>(
    op: &String,
    input: &'a ResourceNode<'a>,
    expressions: &Vec<Box<Expression>>,
) -> CompileResult<ResourceNode<'a>> {
    invocation_table()
        .get(op)
        .ok_or(FhirpathError::CompileError {
            msg: format!("No such operator {}", op),
        })
        .and_then(|function| function(input, expressions))
}

impl Evaluate for EqualityExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "EqualityExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Evaluate for AdditiveExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "AdditiveExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Evaluate for IndexerExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "IndexerExpression must have exactly two children".to_string(),
            });
        }

        let array_node = self.children[0].evaluate(input)?;
        let index_node = self.children[1].evaluate(input)?;

        let index_value = index_node.data.ok_or(FhirpathError::ParserError {
            msg: "IndexerExpression index has no value".to_string(),
        })?;

        let index: i64 = match index_value {
            Value::Number(num) => num.as_i64().ok_or(FhirpathError::ParserError {
                msg: "IndexerExpression index is not a Number".to_string(),
            }),
            Value::String(num_string) => {
                num_string.parse().map_err(|_| FhirpathError::ParserError {
                    msg: "IndexerExpression index is not a Number".to_string(),
                })
            }
            _ => Err(FhirpathError::ParserError {
                msg: "IndexerExpression index is not a Number".to_string(),
            }),
        }?;

        let array_data = array_node.data.ok_or(FhirpathError::ParserError {
            msg: "IndexerExpression did not contain a Value".to_string(),
        })?;

        match array_data {
            Value::Array(array) => Ok(ResourceNode {
                parent_node: Some(Box::new(input)),
                data: Some(array[index as usize].clone()),
            }),
            _ => Err(FhirpathError::ParserError {
                msg: "Element is not an array".to_string(),
            }),
        }
    }
}

impl Evaluate for PolarityExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let child = self.children.first();

        child
            .ok_or(FhirpathError::CompileError {
                msg: "PolarityExpression must have a single child expression".to_string(),
            })
            .and_then(|child_expr| child_expr.evaluate(input))
            .and_then(|result| {
                result.data.ok_or(FhirpathError::CompileError {
                    msg: "PolarityExpression had no result".to_string(),
                })
            })
            .and_then(|expr_result| match expr_result {
                Value::Number(json_num) => {
                    let mut num: i64 = json_num.as_i64().ok_or(FhirpathError::CompileError {
                        msg: "PolarityExpression result was not a number".to_string(),
                    })?;

                    if self.text == "-" {
                        num = -num;
                    }

                    Ok(Value::Number(Number::from(num)))
                }
                _ => Err(FhirpathError::CompileError {
                    msg: "PolarityExpression result was not a number".to_string(),
                }),
            })
            .and_then(|result| {
                Ok(ResourceNode {
                    parent_node: Some(Box::new(input)),
                    data: Some(result),
                })
            })
    }
}

impl Evaluate for Expression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Expression::TermExpression(exp) => exp.evaluate(input),
            Expression::InvocationExpression(exp) => exp.evaluate(input),
            Expression::IndexerExpression(exp) => exp.evaluate(input),
            Expression::PolarityExpression(exp) => exp.evaluate(input),
            Expression::MultiplicativeExpression(exp) => todo!(),
            Expression::AdditiveExpression(exp) => exp.evaluate(input),
            Expression::UnionExpression(exp) => todo!(),
            Expression::InequalityExpression(exp) => todo!(),
            Expression::TypeExpression(exp) => todo!(),
            Expression::EqualityExpression(exp) => exp.evaluate(input),
            Expression::MembershipExpression(exp) => todo!(),
            Expression::AndExpression(exp) => todo!(),
            Expression::OrExpression(exp) => todo!(),
            Expression::ImpliesExpression(exp) => todo!(),
        }
    }
}

impl Evaluate for EntireExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let child = self.children.first().ok_or(FhirpathError::CompileError {
            msg: "Missing EntireExpression child element".to_string(),
        })?;

        child.evaluate(input)
    }
}

impl CompiledPath {
    fn evaluate(&self, resource: Value) -> CompileResult<Option<Value>> {
        let node = ResourceNode {
            data: Some(resource),
            parent_node: None,
        };

        let evaluate_result = self.expression.evaluate(&node)?;

        Ok(evaluate_result.data)
    }
}

pub fn compile(path: &String) -> CompileResult<CompiledPath> {
    Ok(CompiledPath {
        expression: EntireExpression::parse(path)?,
    })
}

#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn evaluates_basic_path() {
        let compiled = compile(&"Patient".to_string()).unwrap();

        let patient = json!({
            "resourceType": "Patient"
        });

        let evaluate_result = compiled.evaluate(patient).unwrap().unwrap();

        assert_json_eq!(
            evaluate_result,
            json!({
                "resourceType": "Patient"
            })
        );
    }

    #[test]
    fn evaluate_name_path() {
        let compiled = compile(&"Patient.name".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test"]
            }]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"]
            }])
        );
    }

    #[test]
    fn evaluate_name_given_path() {
        let compiled = compile(&"Patient.name.given".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test", "test2"]
            }]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        println!("{:?}", evaluate_result);

        assert_json_eq!(evaluate_result, json!(["test", "test2"]));
    }

    #[test]
    fn evaluate_where_path() {
        let compiled = compile(&"Patient.name.where(use = 'usual').given".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test"]
            }]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!(["test"]));
    }

    #[test]
    fn evaluate_index_path() {
        let compiled = compile(&"Patient.name[0]".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test"]
            }]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!({
                "use": "usual",
                "given": ["test"]
            })
        );
    }

    #[test]
    fn evaluate_index_invocation_path() {
        let compiled = compile(&"Patient.name[0].family".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            }]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!("test"));
    }

    #[test]
    fn evaluate_complex_index_path() {
        let compiled = compile(&"Patient.name.where(use = 'usual').given[1]".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test", "test1"]
            }]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!("test1"));
    }

    #[test]
    fn evaluate_inequality_path() {
        let compiled = compile(&"Patient.name.where(use != 'usual').given".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"]
                },
                {
                    "use": "official",
                    "given": ["test1"]
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!(["test1"]));
    }

    #[test]
    fn evaluate_indexof_path() {
        let compiled =
            compile(&"Patient.name.where(family.indexOf('test') = -1)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_substring_path() {
        let compiled =
            compile(&"Patient.name.where(family.substring(1) = 'est')".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            }])
        );
    }

    #[test]
    fn evaluate_startswith_path() {
        let compiled =
            compile(&"Patient.name.where(family.startsWith('tes'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            }])
        );
    }

    #[test]
    fn evaluate_endswith_path() {
        let compiled = compile(&"Patient.name.where(family.endsWith('bc'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_contains_path() {
        let compiled = compile(&"Patient.name.where(family.contains('b'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_upper_path() {
        let compiled = compile(&"Patient.name[0].family.upper()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!(["TEST"]));
    }

    #[test]
    fn evaluate_lower_path() {
        let compiled = compile(&"Patient.name[0].family.lower()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "TEST"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!(["test"]));
    }

    #[test]
    fn evaluate_replace_path() {
        let compiled = compile(&"Patient.name.family.replace('es', '')".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!(["tt", "abc"]));
    }

    #[test]
    fn evaluate_matches_path() {
        let compiled = compile(&"Patient.name[0].family.matches('tes')".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!(true));
    }

    #[test]
    fn evaluate_replace_matches_path() {
        let compiled =
            compile(&"Patient.name.family.replaceMatches('es', '')".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!(["tt", "abc"]));
    }

    #[test]
    fn evaluate_length_path() {
        let compiled = compile(&"Patient.name.family.length()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!([4, 3]));
    }

    #[test]
    fn evaluate_to_chars_path() {
        let compiled = compile(&"Patient.name.family.toChars()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([['t', 'e', 's', 't'], ['a', 'b', 'c']])
        );
    }

    #[test]
    fn evaluate_select_path() {
        let compiled =
            compile(&"Patient.name.select(given[0] + ' ' + family)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!(["test test", "test1 abc"]));
    }

    #[test]
    fn evaluate_single_path() {
        let compiled = compile(&"Patient.name.single()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            }])
        );
    }

    #[test]
    fn evaluate_single_path_no_values() {
        let compiled = compile(&"Patient.name.single()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient);

        assert_eq!(
            evaluate_result,
            Err(FhirpathError::CompileError {
                msg: "Expected array with single element but had 0".to_string()
            })
        )
    }

    #[test]
    fn evaluate_single_path_multiple_values() {
        let compiled = compile(&"Patient.name.single()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient);

        assert_eq!(
            evaluate_result,
            Err(FhirpathError::CompileError {
                msg: "Expected array with single element but had 2".to_string()
            })
        )
    }

    #[test]
    fn evaluate_first_path() {
        let compiled = compile(&"Patient.name.first()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            }])
        );
    }

    #[test]
    fn evaluate_first_empty_path() {
        let compiled = compile(&"Patient.name.first()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_last_path() {
        let compiled = compile(&"Patient.name.last()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_last_empty_path() {
        let compiled = compile(&"Patient.name.last()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_tail_path() {
        let compiled = compile(&"Patient.name.tail()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                },
                {
                    "use": "official",
                    "given": ["test2"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            },
            {
                "use": "official",
                "given": ["test2"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_tail_empty_path() {
        let compiled = compile(&"Patient.name.last()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_skip_path() {
        let compiled = compile(&"Patient.name.skip(1)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                },
                {
                    "use": "official",
                    "given": ["test2"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            },
            {
                "use": "official",
                "given": ["test2"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_skip_empty_path() {
        let compiled = compile(&"Patient.name.skip(1)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_take_path() {
        let compiled = compile(&"Patient.name.take(2)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "test"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                },
                {
                    "use": "official",
                    "given": ["test2"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"],
                "family": "test"
            },
            {
                "use": "official",
                "given": ["test1"],
                "family": "abc"
            }])
        );
    }

    #[test]
    fn evaluate_take_empty_path() {
        let compiled = compile(&"Patient.name.take(1)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_intersect_path() {
        let compiled = compile(&"Patient.a.intersect(Patient.b)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2,3],
            "b": [1,2]
        });

        let evaluate_result = compiled.evaluate(patient).unwrap();

        assert_json_eq!(evaluate_result, json!([1, 2]));
    }
}
