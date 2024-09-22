pub mod entire_expression;
pub mod expression;
pub mod identifier;
pub mod invocation;
pub mod literal;
pub mod root_expression;
pub mod traits;

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
        //RootExpression::parse(input)
        todo!()
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
