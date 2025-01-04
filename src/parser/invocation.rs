use fancy_regex::Regex;

use crate::error::FhirpathError;

use super::{
    expression::Expression,
    identifier::Identifier,
    traits::{Parse, ParseResult},
};

#[derive(Debug)]
pub struct InvocationTerm {
    pub children: Vec<Box<Invocation>>,
}

// impl Matches for InvocationTerm {
//     fn matches(input: &String, cursor: usize) -> bool {
//         return MemberInvocation::matches(input, cursor);
//     }
// }

impl Parse for InvocationTerm {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Invocation>>::new();

        let invocation = Invocation::parse(input, cursor)?;

        children.push(invocation);

        Ok(Box::new(Self { children }))
    }
}

#[derive(Debug)]
pub enum Invocation {
    MemberInvocation(Box<MemberInvocation>),
    FunctionInvocation(Box<FunctionInvocation>),
    ThisInvocation(Box<ThisInvocation>),
    IndexInvocation(Box<IndexInvocation>),
    TotalInvocation(Box<TotalInvocation>),
}

// impl Matches for Invocation {
//     fn matches(input: &String, cursor: usize) -> bool {
//         MemberInvocation::matches(input, cursor)
//             || FunctionInvocation::matches(input, cursor)
//             || ThisInvocation::matches(input, cursor)
//             || IndexInvocation::matches(input, cursor)
//             || TotalInvocation::matches(input, cursor)
//     }
// }

impl Parse for Invocation {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        if let Ok(member_invocation) = MemberInvocation::parse(input, cursor) {
            return Ok(Box::new(Invocation::MemberInvocation(member_invocation)));
        } else if let Ok(function_invocation) = FunctionInvocation::parse(input, cursor) {
            return Ok(Box::new(Invocation::FunctionInvocation(
                function_invocation,
            )));
        } else if let Ok(this_invocation) = ThisInvocation::parse(input, cursor) {
            return Ok(Box::new(Invocation::ThisInvocation(this_invocation)));
        } else if let Ok(index_invocation) = IndexInvocation::parse(input, cursor) {
            return Ok(Box::new(Invocation::IndexInvocation(index_invocation)));
        } else if let Ok(total_invocation) = TotalInvocation::parse(input, cursor) {
            return Ok(Box::new(Invocation::TotalInvocation(total_invocation)));
        }

        Err(FhirpathError::ParserError {
            msg: "Failed to match Invocation".to_string(),
        })
    }
}

#[derive(Debug)]
pub struct MemberInvocation {
    pub children: Vec<Box<Identifier>>,
}

// impl Matches for MemberInvocation {
//     fn matches(input: &String, cursor: usize) -> bool {
//         return Identifier::matches(input, cursor);
//     }
// }

impl Parse for MemberInvocation {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Identifier>>::new();

        let identifier = Identifier::parse(input, cursor)?;

        children.push(identifier);

        Ok(Box::new(Self { children }))
    }
}

static WHITESPACE_REGEX: &str = r"\s+(?=((\\[\\']|[^\\'])*'(\\[\\']|[^\\'])*')*(\\[\\']|[^\\'])*$)";

pub fn filter_ignored_data(s: &str) -> ParseResult<String> {
    let regex = Regex::new(WHITESPACE_REGEX).map_err(|e| {
        dbg!(e);

        FhirpathError::ParserError {
            msg: "Failed to create whitespace regex".to_string(),
        }
    })?;

    Ok(regex.replace_all(s, "").to_string())
}

#[derive(Debug)]
pub struct ParamList {
    pub children: Vec<Box<Expression>>,
}

// impl Matches for ParamList {
//     fn matches(input: &String, _cursor: usize) -> bool {
//         let expr: ParseResult<Vec<String>> =
//             input.split(',').map(|s| filter_ignored_data(s)).collect();

//         expr.and_then(|expressions| {
//             Ok(expressions.iter().all(|exp| {
//                 dbg!(exp);

//                 Expression::matches(&exp.to_string(), 0)
//             }))
//         })
//         .unwrap_or(false)
//     }
// }

impl Parse for ParamList {
    fn parse(input: &String, _cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        let expressions: Vec<String> = input
            .split(',')
            .map(|s| filter_ignored_data(s))
            .collect::<ParseResult<Vec<String>>>()?;

        let mut children = Vec::<Box<Expression>>::new();

        for exp in expressions.iter() {
            children.push(Expression::parse(&exp.to_string(), 0)?)
        }

        Ok(Box::new(Self { children }))
    }
}

#[derive(Debug)]
pub enum IdentifierAndParamList {
    Identifier(Box<Identifier>),
    ParamList(Box<ParamList>),
}

static FUNCTION_INVOCATION_REGEX: &str = r"^([^()]*)\((.*)\)$";

#[derive(Debug)]
pub struct FunctionInvocation {
    pub children: Vec<Box<IdentifierAndParamList>>,
}

// impl Matches for FunctionInvocation {
//     fn matches(input: &String, _cursor: usize) -> bool {
//         let captures =
//             Regex::captures(&Regex::new(FUNCTION_INVOCATION_REGEX).unwrap(), input).unwrap();

//         match captures {
//             Some(capture) => {
//                 let identifier_match = Identifier::matches(&capture[1].to_string(), 0);
//                 let param_list_match_or_empty =
//                     capture[2].is_empty() || ParamList::matches(&capture[2].to_string(), 0);

//                 return identifier_match && param_list_match_or_empty;
//             }
//             None => false,
//         }
//     }
// }

impl Parse for FunctionInvocation {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        let mut children = Vec::<Box<IdentifierAndParamList>>::new();

        let captures = Regex::captures(&Regex::new(FUNCTION_INVOCATION_REGEX).unwrap(), input)
            .map_err(|_| FhirpathError::ParserError {
                msg: "No match for FunctionInvocation".to_string(),
            })?
            .ok_or_else(|| FhirpathError::ParserError {
                msg: "No match for FunctionInvocation".to_string(),
            })?;

        children.push(Box::new(IdentifierAndParamList::Identifier(
            Identifier::parse(&captures[1].to_string(), 0)?,
        )));

        if !captures[2].is_empty() {
            children.push(Box::new(IdentifierAndParamList::ParamList(
                ParamList::parse(&captures[2].to_string(), 0)?,
            )));
        }

        Ok(Box::new(Self { children }))
    }
}

#[derive(Debug)]
pub struct ThisInvocation {
    pub text: String,
}

// impl Matches for ThisInvocation {
//     fn matches(input: &String, cursor: usize) -> bool {
//         input.eq("$this")
//     }
// }

impl Parse for ThisInvocation {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        if !input.eq("$this") {
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
pub struct IndexInvocation {
    pub text: String,
}

// impl Matches for IndexInvocation {
//     fn matches(input: &String, cursor: usize) -> bool {
//         input.eq("$index")
//     }
// }

impl Parse for IndexInvocation {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        if !input.eq("$index") {
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
pub struct TotalInvocation {
    pub text: String,
}

// impl Matches for TotalInvocation {
//     fn matches(input: &String, cursor: usize) -> bool {
//         input.eq("$total")
//     }
// }

impl Parse for TotalInvocation {
    fn parse(input: &String, cursor: usize) -> super::traits::ParseResult<Box<Self>> {
        if !input.eq("$total") {
            return Err(FhirpathError::ParserError {
                msg: "No match for TotalInvocation".to_string(),
            });
        }

        Ok(Box::new(Self {
            text: input.to_owned(),
        }))
    }
}
