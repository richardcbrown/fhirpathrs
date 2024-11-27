use regex::Regex;

use crate::error::FhirpathError;

use super::traits::{Matches, Parse};

static DATETIME_PRECISION_REGEX: &str = "year|month|week|day|hour|minute|second|millisecond";
static PLURAL_DATETIME_PRECISION_REGEX: &str =
    "years|months|weeks|days|hours|minutes|seconds|milliseconds";

#[derive(Debug)]
pub struct LiteralTerm {
    pub children: Vec<Box<Literal>>,
}

impl Matches for LiteralTerm {
    fn matches(input: &String) -> bool {
        Literal::matches(input)
    }
}

impl Parse for LiteralTerm {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Literal>>::new();

        children.push(Literal::parse(input)?);

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

impl Matches for Literal {
    fn matches(input: &String) -> bool {
        NullLiteral::matches(input)
            || BooleanLiteral::matches(input)
            || StringLiteral::matches(input)
            || NumberLiteral::matches(input)
            || DatetimeLiteral::matches(input)
            || TimeLiteral::matches(input)
            || QuantityLiteral::matches(input)
    }
}

impl Parse for Literal {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        if NullLiteral::matches(input) {
            Ok(Box::new(Literal::NullLiteral(NullLiteral::parse(input)?)))
        } else if BooleanLiteral::matches(input) {
            Ok(Box::new(Literal::BooleanLiteral(BooleanLiteral::parse(
                input,
            )?)))
        } else if StringLiteral::matches(input) {
            Ok(Box::new(Literal::StringLiteral(StringLiteral::parse(
                input,
            )?)))
        } else if NumberLiteral::matches(input) {
            Ok(Box::new(Literal::NumberLiteral(NumberLiteral::parse(
                input,
            )?)))
        } else if DatetimeLiteral::matches(input) {
            Ok(Box::new(Literal::DatetimeLiteral(DatetimeLiteral::parse(
                input,
            )?)))
        } else if TimeLiteral::matches(input) {
            Ok(Box::new(Literal::TimeLiteral(TimeLiteral::parse(input)?)))
        } else if QuantityLiteral::matches(input) {
            Ok(Box::new(Literal::QuantityLiteral(QuantityLiteral::parse(
                input,
            )?)))
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

impl Matches for NullLiteral {
    fn matches(input: &String) -> bool {
        input.eq("{}")
    }
}

impl Parse for NullLiteral {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        Ok(Box::new(Self {
            text: input.to_owned(),
        }))
    }
}

#[derive(Debug)]
pub struct BooleanLiteral {
    pub text: String,
}

impl Matches for BooleanLiteral {
    fn matches(input: &String) -> bool {
        input.eq("true") || input.eq("false")
    }
}

impl Parse for BooleanLiteral {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
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

impl Matches for StringLiteral {
    fn matches(input: &String) -> bool {
        Regex::is_match(&Regex::new(STRING_REGEX).unwrap(), input)
    }
}

impl Parse for StringLiteral {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let capture_text =
            Regex::captures(&Regex::new(STRING_REGEX).unwrap(), input).unwrap()[1].to_string();

        Ok(Box::new(Self { text: capture_text }))
    }
}

static NUMBER_REGEX: &str = "^[0-9]+(\\.[0-9]+)?$";

#[derive(Debug)]
pub struct NumberLiteral {
    pub text: String,
}

impl Matches for NumberLiteral {
    fn matches(input: &String) -> bool {
        Regex::is_match(&Regex::new(NUMBER_REGEX).unwrap(), input)
    }
}

impl Parse for NumberLiteral {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let capture_text =
            Regex::captures(&Regex::new(NUMBER_REGEX).unwrap(), input).unwrap()[0].to_string();

        Ok(Box::new(Self { text: capture_text }))
    }
}

static TIME_FORMAT: &str =
    "[0-9][0-9](:[0-9][0-9](:[0-9][0-9](\\.[0-9]+)?)?)?(Z|(\\+|\\-)[0-9][0-9]:[0-9][0-9])?";

#[derive(Debug)]
pub struct DatetimeLiteral {
    pub text: String,
}

impl Matches for DatetimeLiteral {
    fn matches(input: &String) -> bool {
        Regex::is_match(
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
    }
}

impl Parse for DatetimeLiteral {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let capture_text = Regex::captures(
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
        .unwrap()[0]
            .to_string();

        Ok(Box::new(Self { text: capture_text }))
    }
}

#[derive(Debug)]
pub struct TimeLiteral {
    pub text: String,
}

impl Matches for TimeLiteral {
    fn matches(input: &String) -> bool {
        Regex::is_match(
            &Regex::new(format!("@T{}", TIME_FORMAT).as_str()).unwrap(),
            input,
        )
    }
}

impl Parse for TimeLiteral {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let capture_text = Regex::captures(
            &Regex::new(format!("@T{}", TIME_FORMAT).as_str()).unwrap(),
            input,
        )
        .unwrap()[0]
            .to_string();

        Ok(Box::new(Self { text: capture_text }))
    }
}

#[derive(Debug)]
pub struct QuantityLiteral {
    pub text: String,
    pub unit: Option<String>,
}

impl Matches for QuantityLiteral {
    fn matches(input: &String) -> bool {
        let quantity_regex = format!(
            "({})({}|{}|{})?",
            NUMBER_REGEX, DATETIME_PRECISION_REGEX, PLURAL_DATETIME_PRECISION_REGEX, STRING_REGEX
        );

        Regex::is_match(&Regex::new(quantity_regex.as_str()).unwrap(), input)
    }
}

impl Parse for QuantityLiteral {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let quantity_regex = format!(
            "({})({}|{}|{})?",
            NUMBER_REGEX, DATETIME_PRECISION_REGEX, PLURAL_DATETIME_PRECISION_REGEX, STRING_REGEX
        );

        let captures =
            Regex::captures(&Regex::new(quantity_regex.as_str()).unwrap(), input).unwrap();

        // @todo needs looking at
        let capture_text = captures[0].to_string();

        let capture_unit = captures[1].to_string();

        Ok(Box::new(Self {
            text: capture_text,
            unit: Some(capture_unit),
        }))
    }
}
