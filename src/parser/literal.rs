use regex::Regex;

use crate::{error::FhirpathError, lexer::tokens::Token};

use super::traits::{Parse, ParseDetails, ParseResult};

static DATETIME_PRECISION_REGEX: &str = "year|month|week|day|hour|minute|second|millisecond";
static PLURAL_DATETIME_PRECISION_REGEX: &str =
    "years|months|weeks|days|hours|minutes|seconds|milliseconds";

#[derive(Debug)]
pub struct LiteralTerm {
    pub children: Vec<Box<Literal>>,
}

impl Parse for LiteralTerm {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Literal>>::new();

        children.push(Literal::parse(input, cursor)?);

        Ok(Box::new(Self { children }))
    }
}

#[derive(Debug)]
pub enum Literal {
    NullLiteral(Box<NullLiteral>),
    BooleanLiteral(Box<BooleanLiteral>),
    StringLiteral(Box<StringLiteral>),
    NumberLiteral(Box<NumberLiteral>),
    DatetimeLiteral(Box<DatetimeLiteral>),
    TimeLiteral(Box<TimeLiteral>),
    QuantityLiteral(Box<QuantityLiteral>),
}

impl Parse for Literal {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<ParseDetails<Box<Self>>> {
        if let Ok(null_literal) = NullLiteral::parse(input, cursor) {
            Ok(Box::new(Literal::NullLiteral(null_literal)))
        } else if let Ok(boolean_literal) = BooleanLiteral::parse(input, cursor) {
            Ok(Box::new(Literal::BooleanLiteral(boolean_literal)))
        } else if let Ok(string_literal) = StringLiteral::parse(input, cursor) {
            Ok(Box::new(Literal::StringLiteral(string_literal)))
        } else if let Ok(number_literal) = NumberLiteral::parse(input, cursor) {
            Ok(Box::new(Literal::NumberLiteral(number_literal)))
        } else if let Ok(datetime_literal) = DatetimeLiteral::parse(input, cursor) {
            Ok(Box::new(Literal::DatetimeLiteral(datetime_literal)))
        } else if let Ok(time_literal) = TimeLiteral::parse(input, cursor) {
            Ok(Box::new(Literal::TimeLiteral(time_literal)))
        } else if let Ok(quantity_literal) = QuantityLiteral::parse(input, cursor) {
            Ok(Box::new(Literal::QuantityLiteral(quantity_literal)))
        } else {
            Err(FhirpathError::ParserError {
                msg: "Failed to parse Literal".to_string(),
            })
        }
    }
}

#[derive(Debug)]
pub struct NullLiteral {
    pub text: String,
}

impl Parse for NullLiteral {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        if !input.eq("null") {
            return Err(FhirpathError::ParserError {
                msg: "No match for TotalInvocation".to_string(),
            });
        }

        Ok(Box::new(Self {
            text: input.to_owned(),
        }))
    }
}

#[derive(Debug)]
pub struct BooleanLiteral {
    pub text: String,
}

impl Parse for BooleanLiteral {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        if !input.eq("true") && !input.eq("false") {
            return Err(FhirpathError::ParserError {
                msg: "No match for BooleanLiteral".to_string(),
            });
        }

        Ok(Box::new(Self {
            text: input.to_owned(),
        }))
    }
}

static STRING_REGEX: &str = "^\'([^']*)\'$";

#[derive(Debug)]
pub struct StringLiteral {
    pub text: String,
}

impl Parse for StringLiteral {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        let captures =
            Regex::captures(&Regex::new(STRING_REGEX).unwrap(), input).ok_or_else(|| {
                FhirpathError::ParserError {
                    msg: "No match for StringLiteral".to_string(),
                }
            })?;

        let capture_text = captures[1].to_string();

        Ok(Box::new(Self { text: capture_text }))
    }
}

static NUMBER_REGEX: &str = "^[0-9]+(\\.[0-9]+)?$";

#[derive(Debug)]
pub struct NumberLiteral {
    pub text: String,
}

impl Parse for NumberLiteral {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        let captures =
            Regex::captures(&Regex::new(NUMBER_REGEX).unwrap(), input).ok_or_else(|| {
                FhirpathError::ParserError {
                    msg: "No match for NumberLiteral".to_string(),
                }
            })?;

        let capture_text = captures[0].to_string();

        Ok(Box::new(Self { text: capture_text }))
    }
}

static TIME_FORMAT: &str =
    "[0-9][0-9](:[0-9][0-9](:[0-9][0-9](\\.[0-9]+)?)?)?(Z|(\\+|\\-)[0-9][0-9]:[0-9][0-9])?";

#[derive(Debug)]
pub struct DatetimeLiteral {
    pub text: String,
}

impl Parse for DatetimeLiteral {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        let captures = Regex::captures(
            &Regex::new(
                format!(
                    "@[0-9][0-9][0-9][0-9](\\-[0-9][0-9](\\-[0-9][0-9](T{})?)?)?Z?",
                    TIME_FORMAT
                )
                .as_str(),
            )
            .unwrap(),
            input,
        )
        .ok_or_else(|| FhirpathError::ParserError {
            msg: "No match for DatetimeLiteral".to_string(),
        })?;

        let capture_text = captures[0].to_string();

        Ok(Box::new(Self { text: capture_text }))
    }
}

#[derive(Debug)]
pub struct TimeLiteral {
    pub text: String,
}

impl Parse for TimeLiteral {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        let captures = Regex::captures(
            &Regex::new(format!("@T{}", TIME_FORMAT).as_str()).unwrap(),
            input,
        )
        .ok_or_else(|| FhirpathError::ParserError {
            msg: "No match for TimeLiteral".to_string(),
        })?;

        let capture_text = captures[0].to_string();

        Ok(Box::new(Self { text: capture_text }))
    }
}

#[derive(Debug)]
pub struct QuantityLiteral {
    pub text: String,
    pub unit: Option<String>,
}

impl Parse for QuantityLiteral {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<Box<Self>> {
        let quantity_regex = format!(
            "({})({}|{}|{})?",
            NUMBER_REGEX, DATETIME_PRECISION_REGEX, PLURAL_DATETIME_PRECISION_REGEX, STRING_REGEX
        );

        let captures = Regex::captures(&Regex::new(quantity_regex.as_str()).unwrap(), input)
            .ok_or_else(|| FhirpathError::ParserError {
                msg: "No match for QuantityLiteral".to_string(),
            })?;

        // @todo needs looking at
        let capture_text = captures[0].to_string();

        let capture_unit = captures[1].to_string();

        Ok(Box::new(Self {
            text: capture_text,
            unit: Some(capture_unit),
        }))
    }
}
