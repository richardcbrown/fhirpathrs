use super::{
    expression::Expression,
    traits::{Parse, ParseResult},
};

pub struct EntireExpression {
    children: Vec<Box<Expression>>,
}

impl Parse for EntireExpression {
    fn parse(input: &String) -> ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Expression>>::new();

        let expression = Expression::parse(input)?;

        children.push(expression);

        Ok(Box::new(Self { children }))
    }
}
