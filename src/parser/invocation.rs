use crate::error::FhirpathError;

use super::{
    identifier::Identifier,
    traits::{Matches, Parse},
};

pub struct InvocationTerm {
    pub children: Vec<Box<Invocation>>,
}

impl Matches for InvocationTerm {
    fn matches(input: &String) -> bool {
        return MemberInvocation::matches(input);
    }
}

impl Parse for InvocationTerm {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Invocation>>::new();

        let invocation = Invocation::parse(input)?;

        children.push(invocation);

        Ok(Box::new(Self { children }))
    }
}

pub enum Invocation {
    MemberInvocation(Box<MemberInvocation>),
}

impl Matches for Invocation {
    fn matches(input: &String) -> bool {
        return MemberInvocation::matches(input);
    }
}

impl Parse for Invocation {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        if MemberInvocation::matches(input) {
            return Ok(Box::new(Invocation::MemberInvocation(
                MemberInvocation::parse(input)?,
            )));
        }

        Err(FhirpathError::ParserError {
            msg: "Failed to match Invocation".to_string(),
        })
    }
}

pub struct MemberInvocation {
    pub children: Vec<Box<Identifier>>,
}

impl Matches for MemberInvocation {
    fn matches(input: &String) -> bool {
        return Identifier::matches(input);
    }
}

impl Parse for MemberInvocation {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Identifier>>::new();

        let identifier = Identifier::parse(input)?;

        children.push(identifier);

        Ok(Box::new(Self { children }))
    }
}
