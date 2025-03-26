mod combining;
mod comparison;
mod conversion;
mod equality;
mod existence;
mod fhir_type;
mod filtering;
mod invocation_table;
mod logic;
mod math;
mod strings;
mod subsetting;
mod target;
mod test;
mod types;
mod utils;

use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

use fhir_type::determine_fhir_type;
use invocation_table::invocation_table;
use rust_decimal::Decimal;
use serde_json::{json, Number, Value};
use types::date_time::DateTime;
use types::quantity::Quantity;
use types::time::Time;
use types::type_info::{TypeDetails, TypeInfo};
use utils::{get_string, try_convert_to_boolean};

use crate::error::FhirpathError;
use crate::models::ModelDetails;
use crate::parser::expression::{
    AdditiveExpression, AndExpression, EntireExpression, EqualityExpression, Expression,
    ExpressionAndInvocation, ExternalConstantTerm, IdentifierOrStringLiteral, IndexerExpression,
    InequalityExpression, InvocationExpression, MultiplicativeExpression, OrExpression,
    PolarityExpression, Term, TermExpression, TypeExpression, UnionExpression,
};
use crate::parser::identifier::{
    self, Identifier, LiteralContains, LiteralIdentifier, QualifiedIdentifier, TypeSpecifier,
};
use crate::parser::invocation::{
    FunctionInvocation, IdentifierAndParamList, Invocation, InvocationTerm, MemberInvocation,
    ParamList,
};
use crate::parser::literal::{
    BooleanLiteral, DatetimeLiteral, Literal, LiteralTerm, NumberLiteral, QuantityLiteral,
    StringLiteral, TimeLiteral,
};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub fhirpath);

pub type CompileResult<T> = std::result::Result<T, FhirpathError>;

pub struct FhirContext {
    pub model: Option<ModelDetails>,
    pub vars: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub struct PathDetails {
    pub path: String,
    pub fhir_type: Option<String>,
}

#[derive(Clone)]
pub struct ResourceNode<'a> {
    pub data_root: &'a Value,
    pub parent_node: Option<Box<&'a ResourceNode<'a>>>,
    pub data: Value,
    pub context: &'a FhirContext,
    pub path: Option<String>,
    pub fhir_types: Vec<Option<PathDetails>>,
}

impl<'a> ResourceNode<'a> {
    pub fn new(
        data_root: &'a Value,
        parent_node: Option<Box<&'a ResourceNode<'a>>>,
        data: Value,
        context: &'a FhirContext,
        path: Option<String>,
        fhir_types: Vec<Option<PathDetails>>,
    ) -> Self {
        let node_data = match data {
            Value::Array(array) => json!(array),
            Value::Bool(boolean) => json!([boolean]),
            Value::Number(num) => json!([num]),
            Value::Object(obj) => json!([obj]),
            Value::Null => json!([]),
            Value::String(string) => json!([string]),
        };

        Self {
            data_root,
            parent_node,
            data: node_data,
            context,
            path,
            fhir_types,
        }
    }

    pub fn from_node(node: &'a ResourceNode, data: Value) -> Self {
        Self::new(
            node.data_root,
            Some(Box::new(node)),
            data,
            node.context,
            node.path.clone(),
            node.fhir_types.clone(),
        )
    }

    pub fn is_empty(&self) -> CompileResult<bool> {
        match &self.data {
            Value::Array(array) => Ok(array.len() == 0),
            _ => Err(FhirpathError::CompileError {
                msg: "Data must be a Value::Array".to_string(),
            }),
        }
    }

    pub fn is_single(&self) -> CompileResult<bool> {
        match &self.data {
            Value::Array(array) => Ok(array.len() == 1),
            _ => Err(FhirpathError::CompileError {
                msg: "Data must be a Value::Array".to_string(),
            }),
        }
    }

    pub fn get_single(&self) -> CompileResult<Value> {
        if !self.is_single()? {
            return Err(FhirpathError::CompileError {
                msg: "Expected single value for node".to_string(),
            });
        }

        match &self.data {
            Value::Array(array) => {
                let first = array.first().ok_or_else(|| FhirpathError::CompileError {
                    msg: "Expected single value for node".to_string(),
                })?;

                Ok(first.clone())
            }
            _ => Err(FhirpathError::CompileError {
                msg: "Data must be a Value::Array".to_string(),
            }),
        }
    }

    pub fn get_single_or_empty(&self) -> CompileResult<Option<Value>> {
        match &self.data {
            Value::Array(array) => Ok(array.first().cloned()),
            _ => Err(FhirpathError::CompileError {
                msg: "Data must be a Value::Array".to_string(),
            }),
        }
    }

    pub fn get_array(&self) -> CompileResult<&Vec<Value>> {
        match &self.data {
            Value::Array(array) => Ok(array),
            _ => Err(FhirpathError::CompileError {
                msg: "Data must be a Value::Array".to_string(),
            }),
        }
    }

    pub fn get_var(&self, var_name: &String) -> Option<Value> {
        self.context.vars.get(var_name).cloned()
    }

    pub fn get_type_info(&self) -> Vec<Option<TypeInfo>> {
        self.fhir_types
            .iter()
            .map(|pd| {
                pd.as_ref().and_then(|path_details| {
                    TypeInfo::try_from(&TypeDetails {
                        fhir_type: path_details.fhir_type.clone(),
                        model: &self.context.model,
                    })
                    .ok()
                })
            })
            .collect()
    }
}

pub struct CompiledPath {
    expression: Box<EntireExpression>,
}

pub trait Evaluate {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>>;
}

pub trait Text {
    fn text(&self) -> CompileResult<String>;
}

impl Evaluate for StringLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        Ok(ResourceNode::from_node(input, json!(self.text)))
    }
}

impl Text for StringLiteral {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}

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

impl Evaluate for BooleanLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let bool_val =
            try_convert_to_boolean(&Value::String(self.text.clone())).ok_or_else(|| {
                FhirpathError::CompileError {
                    msg: format!("Could not convert {} to Bool", self.text.clone()),
                }
            })?;

        Ok(ResourceNode::from_node(input, Value::Bool(bool_val)))
    }
}

impl Text for BooleanLiteral {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}

impl Evaluate for DatetimeLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        Ok(ResourceNode::from_node(
            input,
            json!(DateTime::try_from(&self.text)?),
        ))
    }
}

impl Text for DatetimeLiteral {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}

impl Evaluate for TimeLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        Ok(ResourceNode::from_node(
            input,
            json!(Time::try_from(&self.text)?),
        ))
    }
}

impl Text for TimeLiteral {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}

impl Evaluate for QuantityLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        Ok(ResourceNode::from_node(
            input,
            json!(Quantity::try_from(self)?),
        ))
    }
}

impl Text for QuantityLiteral {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}

impl Evaluate for Literal {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Literal::BooleanLiteral(exp) => exp.evaluate(input),
            Literal::DatetimeLiteral(exp) => exp.evaluate(input),
            Literal::NullLiteral(exp) => todo!(),
            Literal::NumberLiteral(exp) => exp.evaluate(input),
            Literal::QuantityLiteral(exp) => exp.evaluate(input),
            Literal::StringLiteral(exp) => exp.evaluate(input),
            Literal::TimeLiteral(exp) => exp.evaluate(input),
        }
    }
}

impl Text for Literal {
    fn text(&self) -> CompileResult<String> {
        match self {
            Literal::BooleanLiteral(exp) => exp.text(),
            Literal::DatetimeLiteral(exp) => exp.text(),
            Literal::NullLiteral(exp) => todo!(),
            Literal::NumberLiteral(exp) => exp.text(),
            Literal::QuantityLiteral(exp) => exp.text(),
            Literal::StringLiteral(exp) => exp.text(),
            Literal::TimeLiteral(exp) => exp.text(),
        }
    }
}

fn expand_choice_values<'a>(input: &'a ResourceNode<'a>, property: &String) -> Vec<String> {
    // if there is no model there's nothing to expand
    let model = &input.context.model;

    if model.is_none() {
        return vec![property.clone()];
    }

    // if there's no path there's nothing to expand
    let path = input.path.as_ref().and_then(|p| Some(p.clone()));

    if path.is_none() {
        return vec![property.clone()];
    }

    let choice_path = path.and_then(|p| Some(vec![p, property.clone()].join(".")));

    let choice_elements = model
        .as_ref()
        .and_then(|model_details| model_details.choice_type_paths.get(&choice_path?).clone())
        .and_then(|choice_items| {
            Some(
                choice_items
                    .into_iter()
                    .map(|ci| vec![property.to_string(), ci.to_string()].join(""))
                    .collect::<Vec<String>>(),
            )
        });

    dbg!(choice_elements.clone());

    match choice_elements {
        Some(ce) => ce.to_vec(),
        None => vec![property.clone()],
    }
}

impl Evaluate for MemberInvocation {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if input.is_empty()? {
            return Ok(ResourceNode::from_node(input, json!([])));
        }

        let key_node = self
            .children
            .first()
            .ok_or(FhirpathError::CompileError {
                msg: "MemberInvocation has no child node".to_string(),
            })?
            .evaluate(input)?;

        if !key_node.is_single()? {
            return Err(FhirpathError::CompileError {
                msg: "Could not determine property to invoke".to_string(),
            });
        }

        let key = key_node.get_single()?;

        let key_value = match key {
            Value::String(str) => str,
            _ => "".to_string(),
        };

        let input_data = input.get_array()?;

        let node_resource_type = input_data.first().and_then(|item| item.get("resourceType"));

        // MemberInvocation is resourceType, so return whole resource
        if node_resource_type.is_some_and(|resource_type| resource_type.eq(&key_value)) {
            let mut node = ResourceNode::from_node(input, json!(input_data));

            node.path = Some(key_value.clone());

            node.fhir_types = vec![Some(PathDetails {
                path: key_value.clone(),
                fhir_type: Some(key_value.clone()),
            })];

            return Ok(node);
        }

        dbg!("data");
        dbg!("{:?}", input_data);
        dbg!("{:?}", &key_value.to_string());

        // if the path leads to a choice (e.g. value[x]) expand the
        // choice properties
        let choice_values = expand_choice_values(input, &key_value);

        // Else look for a child property of the resource that matches the key
        let keys_values: Vec<(String, Vec<Value>)> =
            choice_values.iter().fold(vec![], |mut acc, prop| {
                let prop_vals: Vec<Value> = input_data
                    .to_owned()
                    .into_iter()
                    .filter_map(|item| item.get(&prop.to_string()).cloned())
                    .collect();

                if prop_vals.len() != 0 {
                    let flat_vals = prop_vals.into_iter().fold(vec![], |mut acc, item| {
                        match item {
                            Value::Array(mut arr) => acc.append(&mut arr),
                            _ => acc.push(item.clone()),
                        }
                        acc
                    });

                    acc.push((prop.to_string(), flat_vals));
                }

                acc
            });

        dbg!(&keys_values);

        let keys: Vec<String> = keys_values.iter().map(|kv| kv.0.clone()).collect();
        let values: Vec<Vec<Value>> = keys_values.iter().map(|kv| kv.1.clone()).collect();

        let flattened_values: Vec<Value> = values.iter().fold(vec![], |mut acc, val| {
            acc.append(&mut val.clone());
            acc
        });

        dbg!(keys.clone());
        dbg!(flattened_values.clone());

        let mut node = ResourceNode::from_node(input, json!(flattened_values));

        node.path = match &input.path {
            Some(path) => Some(determine_fhir_type(path, &key_value, input.context).path),
            None => None,
        };

        let type_details: Vec<Option<PathDetails>> = keys
            .iter()
            .map(|key| match &input.path {
                Some(path) => Some(determine_fhir_type(path, &key, input.context)),
                None => None,
            })
            .collect();

        let mut fhir_types = vec![];

        for (pos, detail) in type_details.into_iter().enumerate() {
            fhir_types.append(&mut vec![detail; values[pos].len()]);
        }

        dbg!(fhir_types.clone());

        node.fhir_types = fhir_types;

        Ok(node)
    }
}

impl Text for MemberInvocation {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join("."))
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
            .get_single()
            .and_then(|val| match val {
                Value::String(string) => Ok(string),
                _ => Err(FhirpathError::CompileError {
                    msg: "First child of FunctionInvocation should be function name".to_string(),
                }),
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

impl Text for FunctionInvocation {
    fn text(&self) -> CompileResult<String> {
        todo!()
    }
}

impl Evaluate for Invocation {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Invocation::MemberInvocation(exp) => exp.evaluate(input),
            Invocation::FunctionInvocation(exp) => exp.evaluate(input),
            Invocation::IndexInvocation(exp) => todo!(),
            Invocation::ThisInvocation(exp) => todo!(),
            Invocation::TotalInvocation(exp) => todo!(),
        }
    }
}

impl Text for Invocation {
    fn text(&self) -> CompileResult<String> {
        match self {
            Invocation::MemberInvocation(exp) => exp.text(),
            Invocation::FunctionInvocation(exp) => exp.text(),
            Invocation::IndexInvocation(exp) => todo!(),
            Invocation::ThisInvocation(exp) => todo!(),
            Invocation::TotalInvocation(exp) => todo!(),
        }
    }
}

impl Evaluate for LiteralIdentifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        Ok(ResourceNode::from_node(input, json!(self.text.clone())))
    }
}

impl Text for LiteralIdentifier {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}

impl Evaluate for LiteralContains {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        Ok(ResourceNode::from_node(input, json!(self.text.clone())))
    }
}

impl Text for LiteralContains {
    fn text(&self) -> CompileResult<String> {
        Ok(self.text.clone())
    }
}

impl Evaluate for Identifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Identifier::LiteralIdentifier(exp) => exp.evaluate(input),
            Identifier::LiteralAs(exp) => todo!(),
            Identifier::LiteralContains(exp) => exp.evaluate(input),
            Identifier::LiteralDelimitedIdentifier(exp) => todo!(),
            Identifier::LiteralIn(exp) => todo!(),
            Identifier::LiteralIs(exp) => todo!(),
        }
    }
}

impl Text for Identifier {
    fn text(&self) -> CompileResult<String> {
        match self {
            Identifier::LiteralIdentifier(exp) => exp.text(),
            Identifier::LiteralAs(exp) => todo!(),
            Identifier::LiteralContains(exp) => exp.text(),
            Identifier::LiteralDelimitedIdentifier(exp) => todo!(),
            Identifier::LiteralIn(exp) => todo!(),
            Identifier::LiteralIs(exp) => todo!(),
        }
    }
}

impl Evaluate for QualifiedIdentifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let identifiers: Vec<String> = self
            .children
            .iter()
            .map(|child| {
                child
                    .evaluate(input)
                    .and_then(|node| match node.get_single()? {
                        Value::String(string_val) => Ok(string_val),
                        _ => Err(FhirpathError::CompileError {
                            msg: "Invalid Identifier".to_string(),
                        }),
                    })
            })
            .collect::<CompileResult<Vec<String>>>()?;

        Ok(ResourceNode::from_node(
            input,
            Value::String(identifiers.join(".")),
        ))
    }
}

impl Text for QualifiedIdentifier {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join("."))
    }
}

impl Evaluate for TypeSpecifier {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let specifier = match self {
            TypeSpecifier::QualifiedIdentifier(qi) => qi.evaluate(input),
        }?;

        Ok(ResourceNode::from_node(
            input,
            serde_json::to_value(TypeInfo::try_from(&specifier)?).map_err(|err| {
                FhirpathError::CompileError {
                    msg: format!("Failed to serialize TypeInfo: {}", err.to_string()),
                }
            })?,
        ))
    }
}

impl TryFrom<&Expression> for TypeSpecifier {
    type Error = FhirpathError;

    fn try_from(value: &Expression) -> Result<Self, Self::Error> {
        let text = value.text()?;

        let text_items: Vec<&str> = text.split(".").collect();

        // @todo only handles literal identifier
        let children = text_items
            .iter()
            .map(|ti| {
                Box::new(Identifier::LiteralIdentifier(Box::new(LiteralIdentifier {
                    text: ti.to_string(),
                })))
            })
            .collect();

        Ok(TypeSpecifier::QualifiedIdentifier(Box::new(
            QualifiedIdentifier { children },
        )))
    }
}

impl Text for TypeSpecifier {
    fn text(&self) -> CompileResult<String> {
        match self {
            TypeSpecifier::QualifiedIdentifier(qi) => qi.text(),
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

impl Evaluate for LiteralTerm {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 1 {
            return Err(FhirpathError::CompileError {
                msg: "LiteralTerm should have exactly one child".to_string(),
            });
        }

        let first = &self.children[0];

        Ok(ResourceNode::from_node(input, first.evaluate(input)?.data))
    }
}

impl Text for LiteralTerm {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(""))
    }
}

impl Evaluate for Term {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Term::InvocationTerm(exp) => exp.evaluate(input),
            Term::LiteralTerm(exp) => exp.evaluate(input),
            Term::ExternalConstantTerm(exp) => exp.evaluate(input),
            Term::ParenthesizedTerm(exp) => todo!(),
        }
    }
}

impl Text for Term {
    fn text(&self) -> CompileResult<String> {
        match self {
            Term::InvocationTerm(exp) => exp.text(),
            Term::LiteralTerm(exp) => exp.text(),
            Term::ExternalConstantTerm(exp) => exp.text(),
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

impl Text for TermExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(""))
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

impl Text for ExpressionAndInvocation {
    fn text(&self) -> CompileResult<String> {
        match self {
            ExpressionAndInvocation::Expression(expr) => expr.text(),
            ExpressionAndInvocation::Invocation(invocation) => invocation.text(),
        }
    }
}

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

impl Evaluate for IdentifierOrStringLiteral {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let result = match &self {
            IdentifierOrStringLiteral::Identifier(identifier) => identifier.evaluate(input),
            IdentifierOrStringLiteral::StringLiteral(literal) => literal.evaluate(input),
        }?;

        let var_name = get_string(&result.get_single()?)?;

        let variable = input
            .get_var(&var_name)
            .ok_or_else(|| FhirpathError::CompileError {
                msg: format!("Unknown variable {}", var_name),
            })?;

        Ok(ResourceNode::from_node(input, variable))
    }
}

impl Text for IdentifierOrStringLiteral {
    fn text(&self) -> CompileResult<String> {
        match &self {
            IdentifierOrStringLiteral::Identifier(identifier) => identifier.text(),
            IdentifierOrStringLiteral::StringLiteral(literal) => literal.text(),
        }
    }
}

impl Evaluate for ExternalConstantTerm {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        let expression = self
            .children
            .first()
            .ok_or_else(|| FhirpathError::CompileError {
                msg: "ExternalConstantTerm expects a single Expression".to_string(),
            })?;

        let variable = expression.evaluate(input)?;

        Ok(variable)
    }
}

impl Text for ExternalConstantTerm {
    fn text(&self) -> CompileResult<String> {
        Ok(format!(
            "%{}",
            self.children
                .iter()
                .map(|c| c.text())
                .collect::<CompileResult<Vec<String>>>()?
                .join("")
        ))
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

impl Text for EqualityExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(&self.op.clone()))
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

impl Text for AdditiveExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}

impl Evaluate for MultiplicativeExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "MultiplicativeExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Text for MultiplicativeExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}

impl Evaluate for UnionExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "UnionExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Text for UnionExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}

impl Evaluate for AndExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "AndExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Text for AndExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}

impl Evaluate for OrExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "OrExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Text for OrExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}

impl Evaluate for InequalityExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "InequalityExpression must have exactly two children".to_string(),
            });
        }

        invoke_operation(&self.op, input, &self.children)
    }
}

impl Text for InequalityExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(&self.op.clone()))
    }
}

impl Evaluate for TypeExpression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        if self.children.len() != 2 {
            return Err(FhirpathError::CompileError {
                msg: "TypeExpression must have exactly two children".to_string(),
            });
        }

        todo!()
    }
}

impl Text for TypeExpression {
    fn text(&self) -> CompileResult<String> {
        todo!()
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

        let index_value = index_node.get_single()?;

        let index: i64 = match index_value {
            Value::Number(num) => {
                let fpn: f64 = num.as_f64().ok_or(FhirpathError::ParserError {
                    msg: "IndexerExpression index is not a Number".to_string(),
                })?;

                // @todo check conversion here
                Ok(fpn as i64)
            }
            Value::String(num_string) => {
                num_string.parse().map_err(|_| FhirpathError::ParserError {
                    msg: "IndexerExpression index is not a Number".to_string(),
                })
            }
            _ => Err(FhirpathError::ParserError {
                msg: "IndexerExpression index is not a Number".to_string(),
            }),
        }?;

        let array_data = array_node.data;

        match array_data {
            Value::Array(array) => Ok(ResourceNode::from_node(
                input,
                array[index as usize].clone(),
            )),
            _ => Err(FhirpathError::ParserError {
                msg: "Element is not an array".to_string(),
            }),
        }
    }
}

impl Text for IndexerExpression {
    fn text(&self) -> CompileResult<String> {
        todo!()
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
            .and_then(|result| Ok(result.get_single()?))
            .and_then(|expr_result| match expr_result {
                Value::Number(json_num) => {
                    let mut num: i64 = json_num.as_i64().ok_or(FhirpathError::CompileError {
                        msg: "PolarityExpression result was not a number".to_string(),
                    })?;

                    if self.op == "-" {
                        num = -num;
                    }

                    Ok(Value::Number(Number::from(num)))
                }
                _ => Err(FhirpathError::CompileError {
                    msg: "PolarityExpression result was not a number".to_string(),
                }),
            })
            .and_then(|result| Ok(ResourceNode::from_node(input, result)))
    }
}

impl Text for PolarityExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(format!(
            "{}{}",
            self.op.clone(),
            self.children
                .iter()
                .map(|c| c.text())
                .collect::<CompileResult<Vec<String>>>()?
                .join("")
        ))
    }
}

impl Evaluate for Expression {
    fn evaluate<'a>(&self, input: &'a ResourceNode<'a>) -> CompileResult<ResourceNode<'a>> {
        match self {
            Expression::TermExpression(exp) => exp.evaluate(input),
            Expression::InvocationExpression(exp) => exp.evaluate(input),
            Expression::IndexerExpression(exp) => exp.evaluate(input),
            Expression::PolarityExpression(exp) => exp.evaluate(input),
            Expression::MultiplicativeExpression(exp) => exp.evaluate(input),
            Expression::AdditiveExpression(exp) => exp.evaluate(input),
            Expression::UnionExpression(exp) => exp.evaluate(input),
            Expression::InequalityExpression(exp) => exp.evaluate(input),
            Expression::TypeExpression(exp) => exp.evaluate(input),
            Expression::EqualityExpression(exp) => exp.evaluate(input),
            Expression::MembershipExpression(exp) => todo!(),
            Expression::AndExpression(exp) => exp.evaluate(input),
            Expression::OrExpression(exp) => exp.evaluate(input),
            Expression::ImpliesExpression(exp) => todo!(),
        }
    }
}

impl Text for Expression {
    fn text(&self) -> CompileResult<String> {
        match self {
            Expression::TermExpression(exp) => exp.text(),
            Expression::InvocationExpression(exp) => exp.text(),
            Expression::IndexerExpression(exp) => exp.text(),
            Expression::PolarityExpression(exp) => exp.text(),
            Expression::MultiplicativeExpression(exp) => exp.text(),
            Expression::AdditiveExpression(exp) => exp.text(),
            Expression::UnionExpression(exp) => exp.text(),
            Expression::InequalityExpression(exp) => exp.text(),
            Expression::TypeExpression(exp) => exp.text(),
            Expression::EqualityExpression(exp) => exp.text(),
            Expression::MembershipExpression(exp) => todo!(),
            Expression::AndExpression(exp) => exp.text(),
            Expression::OrExpression(exp) => exp.text(),
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

impl Text for EntireExpression {
    fn text(&self) -> CompileResult<String> {
        Ok(self
            .children
            .iter()
            .map(|c| c.text())
            .collect::<CompileResult<Vec<String>>>()?
            .join(""))
    }
}

#[derive(Clone)]
pub struct EvaluateOptions {
    model: Option<ModelDetails>,
    vars: Option<HashMap<String, Value>>,
}

impl CompiledPath {
    fn evaluate(&self, resource: Value, options: Option<EvaluateOptions>) -> CompileResult<Value> {
        let opts = options.unwrap_or(EvaluateOptions {
            model: None,
            vars: None,
        });

        let mut vars = HashMap::<String, Value>::new();

        vars.insert(
            "ucum".to_string(),
            Value::String("http://unitsofmeasure.org".to_string()),
        );

        vars.insert("resource".to_string(), resource.clone());
        vars.insert("rootResource".to_string(), resource.clone());

        if let Some(custom_vars) = opts.vars {
            vars.extend(custom_vars);
        }

        let context = FhirContext {
            model: opts.model,
            vars,
        };

        let node = ResourceNode::new(&resource, None, resource.clone(), &context, None, vec![]);

        let evaluate_result = self.expression.evaluate(&node)?;

        dbg!(evaluate_result.path);

        Ok(evaluate_result.data)
    }
}

pub fn compile(path: &String) -> CompileResult<CompiledPath> {
    Ok(CompiledPath {
        expression: Box::new(fhirpath::EntireExpressionParser::new().parse(path).unwrap()),
    })
}

#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use crate::models::{get_model_details, ModelType};

    use super::*;

    #[test]
    fn evaluates_basic_path() {
        let compiled = compile(&"Patient".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient"
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "resourceType": "Patient"
            }])
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

        let evaluate_result = compiled
            .evaluate(
                patient,
                Some(EvaluateOptions {
                    model: Some(get_model_details(ModelType::Stu3).unwrap()),
                    vars: None,
                }),
            )
            .unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        println!("{:?}", evaluate_result);

        assert_json_eq!(evaluate_result, json!(["test", "test2"]));
    }

    #[test]
    fn evaluate_where_path_simple() {
        let compiled = compile(&"Patient.name.where(use = 'usual')".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [{
                "use": "usual",
                "given": ["test"]
            }]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"]
            }])
        );
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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(
            evaluate_result,
            json!([{
                "use": "usual",
                "given": ["test"]
            }])
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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["test"]));
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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["test1"]));
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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None);

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

        let evaluate_result = compiled.evaluate(patient, None);

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1, 2]));
    }

    #[test]
    fn evaluate_exclude_path() {
        let compiled = compile(&"Patient.a.exclude(Patient.b)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2,3],
            "b": [1,2]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([3]));
    }

    #[test]
    fn evaluate_union_path() {
        let compiled = compile(&"Patient.a.union(Patient.b)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1, 2, 3]));
    }

    #[test]
    fn evaluate_union_symbol_path() {
        let compiled = compile(&"Patient.a | Patient.b".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1, 2, 3]));
    }

    #[test]
    fn evaluate_combine_path() {
        let compiled = compile(&"Patient.a.combine(Patient.b)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1, 2, 1, 2, 3]));
    }

    #[test]
    fn evaluate_iif_path() {
        let compiled = compile(&"iif(Patient.c, Patient.a, Patient.b)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3],
            "c": true
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([1, 2]));
    }

    #[test]
    fn evaluate_iif_path_default_else() {
        let compiled = compile(&"iif(Patient.c, Patient.a)".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_toboolean_true_path() {
        let compiled = compile(&"Patient.a[0].toBoolean()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_toboolean_false_path() {
        let compiled = compile(&"Patient.a[1].toBoolean()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,0],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_toboolean_empty_path() {
        let compiled = compile(&"Patient.b[2].toBoolean()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,0],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([]));
    }

    #[test]
    fn evaluate_convertstoboolean_true_path() {
        let compiled = compile(&"Patient.a[0].convertsToBoolean()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,2],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_convertstoboolean_false_path() {
        let compiled = compile(&"Patient.b[2].convertsToBoolean()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [1,0],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_empty_true_path() {
        let compiled = compile(&"Patient.a.empty()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_empty_false_path() {
        let compiled = compile(&"Patient.b.empty()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_exists_true_path() {
        let compiled = compile(&"Patient.b.exists()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_exists_false_path() {
        let compiled = compile(&"Patient.a.exists()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "a": [],
            "b": [1,2,3],
            "c": false
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_exists_criteria_true_path() {
        let compiled = compile(&"Patient.name.exists(family.endsWith('bc'))".to_string()).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_exists_criteria_false_path() {
        let compiled = compile(&"Patient.name.exists(family.endsWith('bb'))".to_string()).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_all_criteria_true_path() {
        let compiled = compile(&"Patient.name.all(family.endsWith('bc'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "usual",
                    "given": ["test"],
                    "family": "tebc"
                },
                {
                    "use": "official",
                    "given": ["test1"],
                    "family": "abc"
                }
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_all_criteria_empty_path() {
        let compiled = compile(&"Patient.name.all(family.endsWith('bc'))".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "name": []
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_all_criteria_false_path() {
        let compiled = compile(&"Patient.name.all(family.endsWith('bc'))".to_string()).unwrap();

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

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_alltrue_true_path() {
        let compiled = compile(&"Patient.b.allTrue()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "b": [1, "1", 1.0, "1.0", "y", "yes", true]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([true]));
    }

    #[test]
    fn evaluate_alltrue_false_path() {
        let compiled = compile(&"Patient.b.allTrue()".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "b": [1, "1", 1.0, "1.0", "y", "yes", true, false]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!([false]));
    }

    #[test]
    fn evaluate_variable_basic_path() {
        let compiled = compile(&"Patient.b.where(value = %ucum).value".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "b": [
                { "value": "1" },
                { "value": "2" },
                { "value": "http://unitsofmeasure.org"}
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["http://unitsofmeasure.org"]));
    }

    #[test]
    fn evaluate_variable_resource_path() {
        let compiled = compile(&"%resource.b.where(value = %ucum).value".to_string()).unwrap();

        print!("{:?}", compiled.expression);

        let patient = json!({
            "resourceType": "Patient",
            "b": [
                { "value": "1" },
                { "value": "2" },
                { "value": "http://unitsofmeasure.org"}
            ]
        });

        let evaluate_result = compiled.evaluate(patient, None).unwrap();

        assert_json_eq!(evaluate_result, json!(["http://unitsofmeasure.org"]));
    }
}
