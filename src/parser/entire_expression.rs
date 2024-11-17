use super::{
    expression::Expression,
    traits::{Parse, ParseResult},
};

#[derive(Debug)]
pub struct EntireExpression {
    pub children: Vec<Box<Expression>>,
}

impl Parse for EntireExpression {
    fn parse(input: &String) -> ParseResult<Box<Self>> {
        let mut children = Vec::<Box<Expression>>::new();

        let expression = Expression::parse(input)?;

        children.push(expression);

        Ok(Box::new(Self { children }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_path() {
        let result = EntireExpression::parse(&"Patient".to_string()).unwrap();

        assert_eq!(result.children.len(), 1);
    }
}
