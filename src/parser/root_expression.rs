use std::io::Cursor;

use super::{
    entire_expression::EntireExpression,
    traits::{Parse, ParseResult},
};

pub struct RootExpression {
    pub children: Vec<Box<EntireExpression>>,
}

impl Parse for RootExpression {
    fn parse(input: &String, cursor: usize) -> ParseResult<Box<Self>> {
        let mut children = Vec::<Box<EntireExpression>>::new();

        let entire_expression = EntireExpression::parse(input, cursor)?;

        children.push(entire_expression);

        Ok(Box::new(Self { children }))
    }
}
