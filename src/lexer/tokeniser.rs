use crate::parser::traits::ParseResult;

use super::tokens::{Matches, Token};

pub fn tokenise(input: &String) -> ParseResult<Vec<Token>> {
    let mut position: usize = 0;

    let mut tokens: Vec<Token> = vec![];

    while position < input.len() {
        let token_result = Token::matches(input, position);

        if let Some(token) = token_result {
            position = token.end.clone();

            tokens.push(token);
        } else {
            return Err(crate::error::FhirpathError::LexerError {
                msg: "Failed to match input".to_string(),
            });
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::tokenise;

    #[test]
    fn basic_tokenise() {
        let result = tokenise(&"Patient.name.given".to_string()).unwrap();

        assert_eq!(result.len(), 5);
        assert_eq!(result[0].value, "Patient");
        assert_eq!(result[1].value, ".");
        assert_eq!(result[2].value, "name");
        assert_eq!(result[3].value, ".");
        assert_eq!(result[4].value, "given");
    }
}
