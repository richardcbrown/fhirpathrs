use super::{
    error::FhirpathError,
    invocation::Invocation,
    traits::{Matches, Parse},
};

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
    TermExpression(TermExpression),
    InvocationExpression(InvocationExpression),
}

impl Matches for Expression {
    fn matches(input: &String) -> bool {
        TermExpression::matches(input) || InvocationExpression::matches(input)
    }
}

impl Parse for Expression {
    fn parse(input: &String) -> super::traits::ParseResult<Box<Self>> {
        if TermExpression::matches(input) {
            return Ok(TermExpression::parse(input));
        } else if InvocationExpression::matches(input) {
            return Ok(InvocationExpression::parse(input));
        }

        Err(FhirpathError::ParserError {
            msg: "No match".to_string(),
        })
    }
}

pub struct TermExpression {}

impl Matches for TermExpression {
    fn matches(input: &String) -> bool {
        todo!()
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
