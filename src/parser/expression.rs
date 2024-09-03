use super::{
    invocation::{Invocation, InvocationTerm},
    traits::{Matches, Parse},
};
use crate::error::FhirpathError;

/**
 * expression
 *   : term                                               #termExpression
 *   | expression '.' invocation                          #invocationExpression
 *   | expression '[' expression ']'                      #indexerExpression
 *   | ('+' | '-') expression                             #polarityExpression
 *   | expression ('*' | '/' | 'div' | 'mod') expression  #multiplicativeExpression
 *   | expression ('+' | '-' | '&') expression            #additiveExpression
 *   | expression '|' expression                          #unionExpression
 *   | expression ('<=' | '<' | '>' | '>=') expression    #inequalityExpression
 *   | expression ('is' | 'as') typeSpecifier             #typeExpression
 *   | expression ('=' | '~' | '!=' | '!~') expression    #equalityExpression
 *   | expression ('in' | 'contains') expression          #membershipExpression
 *   | expression 'and' expression                        #andExpression
 *   | expression ('or' | 'xor') expression               #orExpression
 *   | expression 'implies' expression                    #impliesExpression
 *   //| (IDENTIFIER)? '=>' expression                    #lambdaExpression
 *   ;
 */
pub enum Expression {
    TermExpression(Box<TermExpression>),
    InvocationExpression(Box<InvocationExpression>),
}

impl Matches for Expression {
    fn matches(input: &String) -> bool {
        TermExpression::matches(input) || InvocationExpression::matches(input)
    }
}

impl Parse for Expression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        if TermExpression::matches(input) {
            return Ok(Box::new(Expression::TermExpression(TermExpression::parse(
                input,
            )?)));
        } else if InvocationExpression::matches(input) {
            return Ok(Box::new(Expression::InvocationExpression(
                InvocationExpression::parse(input)?,
            )));
        }

        Err(FhirpathError::ParserError {
            msg: "No match".to_string(),
        })
    }
}

pub enum Term {
    InvocationTerm(Box<InvocationTerm>),
}

impl Matches for Term {
    fn matches(input: &String) -> bool {
        return InvocationTerm::matches(input);
    }
}

impl Parse for Term {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        if InvocationTerm::matches(input) {
            return Ok(Box::new(Term::InvocationTerm(InvocationTerm::parse(
                input,
            )?)));
        }

        Err(FhirpathError::ParserError {
            msg: "Failed to match Term".to_string(),
        })
    }
}

pub struct TermExpression {
    pub children: Vec<Box<Term>>,
}

impl Matches for TermExpression {
    fn matches(input: &String) -> bool {
        return InvocationTerm::matches(input);
    }
}

impl Parse for TermExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Term>>::new();

        let term = Term::parse(input)?;

        children.push(term);

        Ok(Box::new(Self { children }))
    }
}

pub struct InvocationExpression {}

impl Matches for InvocationExpression {
    fn matches(input: &String) -> bool {
        let components: Vec<&str> = input.split('.').collect();

        if components.len() != 2 {
            return false;
        }

        Expression::matches(&components[0].to_string())
            && Invocation::matches(&components[1].to_string())
    }
}

impl Parse for InvocationExpression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        todo!()
    }
}
