use std::collections::HashMap;
use serde_json::Value;
use crate::{error::FhirpathError, parser::expression::Expression};
use super::{CompileResult, Evaluate, ResourceNode};

fn where_function<'a>(input: &'a ResourceNode<'a>, expressions: Vec<Expression>) -> CompileResult<ResourceNode<'a>> {
  let first_expression = expressions.first();

  match first_expression {
    Some(expr) => {
      Ok(ResourceNode {
        parent_node: Some(Box::new(input)),
        data: input.data.as_ref().and_then(|val| {
          match val {
              Value::Array(array) => {
                let results: Vec<Value> = array.iter().filter_map(|item| {
                    let node = ResourceNode {
                      data: Some(item.to_owned()),
                      parent_node: None
                    };
                    
                    expr.evaluate(&node).ok().and_then(|result| {
                      match result.data {
                        Some(value) => {
                          match value {
                            Value::Bool(bool) => {
                              if bool {
                                return Some(item.to_owned());
                              }

                              None
                            }
                            _ => None
                          }
                        }
                        None => None
                      }
                    })
                }).collect();

                Some(Value::Array(results))
              },
              _ => None
          }
        })
      })
    }
    None => Err(FhirpathError::CompileError { msg: "where_function requires single expression argument".to_string() })
  }
}

pub fn invocation_table<'a>() -> HashMap<String, fn (input: &'a ResourceNode<'a>, expressions: Vec<Expression>) -> CompileResult<ResourceNode<'a>>> {
  let mut map = HashMap::<String, fn (input: &'a ResourceNode<'a>, expressions: Vec<Expression>) -> CompileResult<ResourceNode<'a>>>::new();

  map.insert("where".to_string(), where_function);

  map
}