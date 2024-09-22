use regex::Regex;

use super::traits::{Matches, Parse};

static DATETIME_PRECISION_REGEX: &str = "year|month|week|day|hour|minute|second|millisecond";
static PLURAL_DATETIME_PRECISION_REGEX: &str =
    "years|months|weeks|days|hours|minutes|seconds|milliseconds";

pub struct LiteralTerm {
    pub children: Vec<Box<Literal>>,
}

pub enum Literal {
    NullLiteral(Box<NullLiteral>),
    BooleanLiteral(Box<BooleanLiteral>),
    StringLiteral(Box<StringLiteral>),
    NumberLiteral(Box<NumberLiteral>),
    DatetimeLiteral(Box<DatetimeLiteral>),
    TimeLiteral(Box<TimeLiteral>),
    QuantityLiteral(Box<QuantityLiteral>),
}

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

static STRING_REGEX: &str = "\'([^']*)\'";

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
            Regex::captures(&Regex::new(STRING_REGEX).unwrap(), input).unwrap()[0].to_string();

        Ok(Box::new(Self { text: capture_text }))
    }
}

static NUMBER_REGEX: &str = "[0-9]+(\\.[0-9]+)?";

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
