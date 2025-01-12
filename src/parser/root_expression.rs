use crate::lexer::tokens::Token;

use super::{
    entire_expression::EntireExpression,
    traits::{Parse, ParseDetails, ParseResult},
};

pub struct RootExpression {
    pub children: Vec<Box<EntireExpression>>,
}

impl Parse for RootExpression {
    fn parse(tokens: &Vec<Token>, cursor: usize) -> ParseResult<ParseDetails<Box<Self>>> {
        let mut children = Vec::<Box<EntireExpression>>::new();

        let entire_expression = EntireExpression::parse(tokens, cursor)?;

        children.push(entire_expression.value);

        Ok(ParseDetails {
            position: entire_expression.position,
            value: Box::new(Self { children }),
        })
    }
}
