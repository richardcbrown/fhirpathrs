use std::ops::Deref;

use serde_json::Value;

use crate::{
    error::FhirpathError,
    evaluate::{invocation_table::invocation_table, EvaluateResult, Evaluate, Text},
    parser::invocation::{FunctionInvocation, IdentifierAndParamList, ParamList},
};

use super::resource_node::ResourceNode;

impl Evaluate for FunctionInvocation {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> EvaluateResult<ResourceNode<'a>> {
        let function_name_child = self.children[0].deref();
        let param_list_child = if self.children.len() == 2 {
            Some(self.children[1].deref())
        } else {
            None
        };

        let identifier = match function_name_child {
            IdentifierAndParamList::Identifier(identifier) => Ok(identifier),
            _ => Err(FhirpathError::EvaluateError {
                msg: "First child of FunctionInvocation should be function name".to_string(),
            }),
        }?;

        let default_params =
            IdentifierAndParamList::ParamList(Box::new(ParamList { children: vec![] }));

        let param_list = param_list_child.unwrap_or(&default_params);

        let parameters = match param_list {
            IdentifierAndParamList::ParamList(param_list) => Ok(&param_list.children),
            _ => Err(FhirpathError::EvaluateError {
                msg: "Second child of FunctionInvocation should be function params".to_string(),
            }),
        }?;

        let function_name = identifier
            .evaluate(input)?
            .get_single()
            .and_then(|val| match val {
                Value::String(string) => Ok(string),
                _ => Err(FhirpathError::EvaluateError {
                    msg: "First child of FunctionInvocation should be function name".to_string(),
                }),
            })?;

        let invocation = invocation_table()
            .get(&function_name)
            .ok_or(FhirpathError::EvaluateError {
                msg: "No method in invocation table".to_string(),
            })?
            .clone();

        Ok(invocation(input, &parameters)?)
    }
}

impl Text for FunctionInvocation {
    fn text(&self) -> EvaluateResult<String> {
        let identifier = self.children.first().and_then(|ident| {
            match ident.deref() {
                IdentifierAndParamList::Identifier(ident) => Some(ident),
                _ => None,
            }
        }).ok_or(FhirpathError::EvaluateError {
            msg: "First child of FunctionInvocation should be function name".to_string(),
        })?.text()?;

        let paramlist = self.children.iter().nth(1).and_then(|ident| {
            match ident.deref() {
                IdentifierAndParamList::ParamList(param_list) => Some(param_list),
                _ => None,
            }
        }).and_then(|pl| {
            pl.children.iter().map(|child| child.text()).collect::<EvaluateResult<Vec<String>>>().ok()
        }).ok_or(FhirpathError::EvaluateError {
            msg: "Second child of FunctionInvocation should be parameter list".to_string(),
        })?;
        
        Ok(format!("{}({})",  identifier, paramlist.join(",")))
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluate::{compile, Text};

    #[test]
    fn test_function_invocation_text() {
        let compiled = compile(&"Patient.name.where(a = 'b')".to_string());

        let text = compiled.unwrap().expression.text().unwrap();
        assert_eq!(text, "Patient.name.where(a='b')");
    }
}