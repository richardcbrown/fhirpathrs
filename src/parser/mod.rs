mod entire_expression;
mod error;
mod expression;
mod identifier;
mod invocation;
mod root_expression;
mod traits;

use crate::tokeniser::Token;
use root_expression::RootExpression;

// pub enum MemberInvocationChild {
//     MemberInvocation(MemberInvocationNode),
// }

// pub struct MemberInvocationNode {
//     name: String,
//     nodes: Option<Box<MemberInvocationChild>>,
// }

pub struct Parser {
    //tokens: Vec<Token>,
}

impl Parser {
    pub fn parse(input: &String) -> Self {
        RootExpression::parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let _result = Parser::parse(&"Patient.name.where(use = 'official')".to_string());
    }
}
