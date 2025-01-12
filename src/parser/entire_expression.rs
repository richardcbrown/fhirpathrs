use crate::lexer::tokens::Token;

use super::{
    expression::Expression,
    traits::{Parse, ParseDetails, ParseResult},
};

#[derive(Debug)]
pub struct EntireExpression {
    pub children: Vec<Box<Expression>>,
}

impl Parse for EntireExpression {
    fn parse(input: &Vec<Token>, cursor: usize) -> ParseResult<ParseDetails<Box<Self>>> {
        let mut children = Vec::<Box<Expression>>::new();

        let expression = Expression::parse(input, cursor)?;

        children.push(expression.value);

        Ok(ParseDetails {
            value: Box::new(Self { children }),
            position: expression.position,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_path() {
        let result = EntireExpression::parse(&"Patient".to_string(), 0).unwrap();

        assert_eq!(result.children.len(), 1);
    }
}
