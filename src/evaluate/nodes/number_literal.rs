use std::str::FromStr;

use rust_decimal::Decimal;

use crate::{
    error::FhirpathError,
    evaluate::{CompileResult, Evaluate, Text},
    parser::literal::NumberLiteral,
};

use super::resource_node::ResourceNode;

impl Evaluate for NumberLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let value =
            Decimal::from_str(self.text.as_str()).map_err(|_| FhirpathError::ParserError {
                msg: "NumberLiteral is not a Number".to_string(),
            })?;

        let decimal_value =
            serde_json::to_value(value).map_err(|err| FhirpathError::ParserError {
                msg: format!("Failed to serialize Decimal: {}", err.to_string()),
            })?;

        Ok(ResourceNode::from_node(input, decimal_value))
    }
}

impl Text for NumberLiteral {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}
