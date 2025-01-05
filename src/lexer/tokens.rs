use std::sync::LazyLock;

use regex::Regex;

pub trait Matches {
    fn matches(input: &String, position: usize) -> Option<Token>;
}

pub struct Eq;

impl Matches for Eq {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Eq(Eq),
            &"=".to_string(),
            input,
            position,
            None,
            None,
        )
    }
}

pub struct True;
impl Matches for True {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::True(True),
            &"true".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct False;
impl Matches for False {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::False(False),
            &"false".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct Integer;
impl Matches for Integer {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_regex(
            TokenType::Integer(Integer),
            &Regex::new(r"$[\d+]").unwrap(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct Number;
impl Matches for Number {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_regex(
            TokenType::Number(Number),
            &Regex::new(r"$[\d+].[\d+]").unwrap(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct Add;
pub struct Sub;
pub struct Identifier;
pub struct StringLiteral;
pub struct Whitespace;
pub struct Lparen;
pub struct Rparen;
pub struct Lbracket;
pub struct Rbracket;
pub struct Gt;

impl Matches for Gt {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Gt(Gt),
            &">".to_string(),
            input,
            position,
            Some(Regex::new("$[=]").unwrap()),
            None,
        )
    }
}
pub struct Ge;

impl Matches for Ge {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Ge(Ge),
            &">=".to_string(),
            input,
            position,
            None,
            None,
        )
    }
}
pub struct Lt;
impl Matches for Lt {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Lt(Lt),
            &"<".to_string(),
            input,
            position,
            Some(Regex::new("$[=]").unwrap()),
            None,
        )
    }
}
pub struct Le;
impl Matches for Le {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Le(Le),
            &"<=".to_string(),
            input,
            position,
            None,
            None,
        )
    }
}
pub struct Ne;
impl Matches for Ne {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Ne(Ne),
            &"!=".to_string(),
            input,
            position,
            None,
            None,
        )
    }
}
pub struct Mul;
impl Matches for Mul {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Mul(Mul),
            &"*".to_string(),
            input,
            position,
            None,
            None,
        )
    }
}
pub struct Div;
impl Matches for Div {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Div(Div),
            &"div".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct Slash;
impl Matches for Slash {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Slash(Slash),
            &"/".to_string(),
            input,
            position,
            None,
            None,
        )
    }
}
pub struct And;
impl Matches for And {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::And(And),
            &"and".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct Or;
impl Matches for Or {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Or(Or),
            &"or".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct Xor;
impl Matches for Xor {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Xor(Xor),
            &"xor".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct As;
impl Matches for As {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::As(As),
            &"as".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct Is;
impl Matches for Is {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Is(Is),
            &"is".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct Dot;
impl Matches for Dot {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Dot(Dot),
            &".".to_string(),
            input,
            position,
            None,
            None,
        )
    }
}
pub struct Mod;
impl Matches for Mod {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Mod(Mod),
            &"mod".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct Tilde;
impl Matches for Tilde {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Tilde(Tilde),
            &"~".to_string(),
            input,
            position,
            None,
            None,
        )
    }
}
pub struct Ntilde;
impl Matches for Ntilde {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Ntilde(Ntilde),
            &"!~".to_string(),
            input,
            position,
            None,
            None,
        )
    }
}
pub struct Amp;
impl Matches for Amp {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Amp(Amp),
            &"&".to_string(),
            input,
            position,
            None,
            None,
        )
    }
}
pub struct Pipe;
impl Matches for Pipe {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Pipe(Pipe),
            &"|".to_string(),
            input,
            position,
            None,
            None,
        )
    }
}
pub struct In;
impl Matches for In {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::In(In),
            &"in".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct Contains;
impl Matches for Contains {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Contains(Contains),
            &"contains".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}
pub struct Implies;
impl Matches for Implies {
    fn matches(input: &String, position: usize) -> Option<Token> {
        match_value(
            TokenType::Implies(Implies),
            &"implies".to_string(),
            input,
            position,
            None,
            Some(Regex::new(r"$[\w]").unwrap()),
        )
    }
}

pub enum TokenType {
    Add(Add),
    Sub(Sub),
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    Whitespace(Whitespace),
    Lparen(Lparen),
    Rparen(Rparen),
    Lbracket(Lbracket),
    Rbracket(Rbracket),
    Gt(Gt),
    Ge(Ge),
    Lt(Lt),
    Le(Le),
    Ne(Ne),
    Mul(Mul),
    Div(Div),
    And(And),
    Or(Or),
    Xor(Xor),
    As(As),
    Is(Is),
    Dot(Dot),
    Eq(Eq),
    Mod(Mod),
    Slash(Slash),
    Tilde(Tilde),
    Ntilde(Ntilde),
    Contains(Contains),
    In(In),
    Implies(Implies),
    Amp(Amp),
    Pipe(Pipe),
    True(True),
    False(False),
    Number(Number),
    Integer(Integer),
}

impl Matches for Token {
    fn matches(input: &String, position: usize) -> Option<Token> {
        if let Some(token) = Integer::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Number::matches(input, position) {
            return Some(token);
        } else if let Some(token) = False::matches(input, position) {
            return Some(token);
        } else if let Some(token) = True::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Pipe::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Amp::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Implies::matches(input, position) {
            return Some(token);
        } else if let Some(token) = In::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Contains::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Ntilde::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Tilde::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Slash::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Mod::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Eq::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Dot::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Is::matches(input, position) {
            return Some(token);
        } else if let Some(token) = As::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Xor::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Or::matches(input, position) {
            return Some(token);
        } else if let Some(token) = And::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Div::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Mul::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Ne::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Le::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Lt::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Ge::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Gt::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Rbracket::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Lbracket::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Rparen::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Lparen::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Whitespace::matches(input, position) {
            return Some(token);
        } else if let Some(token) = StringLiteral::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Identifier::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Sub::matches(input, position) {
            return Some(token);
        } else if let Some(token) = Add::matches(input, position) {
            return Some(token);
        }

        None
    }
}

fn match_value(
    token_type: TokenType,
    value: &String,
    input: &String,
    position: usize,
    blocked_next_chars: Option<Regex>,
    allowed_next_chars: Option<Regex>,
) -> Option<Token> {
    let current_input: String = input.chars().skip(position).collect();

    if !current_input.starts_with(value) {
        return None;
    }

    let next_input: String = current_input.chars().skip(value.len()).collect();

    if let Some(regex) = blocked_next_chars {
        if Regex::is_match(&regex, &next_input) {
            return None;
        }
    }

    if let Some(regex) = allowed_next_chars {
        if !Regex::is_match(&regex, &next_input) {
            return None;
        }
    }

    Some(Token::new(token_type, position, value))
}

fn match_regex(
    token_type: TokenType,
    regex: &Regex,
    input: &String,
    position: usize,
    blocked_next_chars: Option<Regex>,
    allowed_next_chars: Option<Regex>,
) -> Option<Token> {
    let current_input: String = input.chars().skip(position).collect();

    let captures = Regex::captures(regex, &current_input)?;

    let value = captures[0].to_string();

    let next_input: String = current_input.chars().skip(value.len()).collect();

    if let Some(regex) = blocked_next_chars {
        if Regex::is_match(&regex, &next_input) {
            return None;
        }
    }

    if let Some(regex) = allowed_next_chars {
        if !Regex::is_match(&regex, &next_input) {
            return None;
        }
    }

    Some(Token::new(token_type, position, &value))
}

impl Matches for Add {
    fn matches(input: &String, position: usize) -> Option<Token> {
        let next_char = input.chars().nth(position)?;

        if next_char.eq(&'+') {
            return Some(Token {
                token_type: TokenType::Add(Add),
                start: position,
                end: position + 1,
                value: "+".to_string(),
            });
        }

        None
    }
}

impl Matches for Sub {
    fn matches(input: &String, position: usize) -> Option<Token> {
        let next_char = input.chars().nth(position)?;

        if next_char.eq(&'-') {
            return Some(Token::new(TokenType::Sub(Sub), position, &"-".to_string()));
        }

        None
    }
}

static IDENTIFIER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^([A-Za-z]|_)([A-Za-z0-9]|_)*").unwrap());

impl Matches for Identifier {
    fn matches(input: &String, position: usize) -> Option<Token> {
        let current: String = input.chars().skip(position).collect();

        let captures = Regex::captures(&IDENTIFIER_REGEX, &current)?;

        let capture_text = captures[0].to_string();

        Some(Token::new(
            TokenType::Identifier(Identifier),
            position,
            &capture_text,
        ))
    }
}

static STRING_LITERAL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\'([^']*)\'").unwrap());

impl Matches for StringLiteral {
    fn matches(input: &String, position: usize) -> Option<Token> {
        let current: String = input.chars().skip(position).collect();

        let captures = Regex::captures(&STRING_LITERAL_REGEX, &current)?;

        let capture_text = captures[0].to_string();

        Some(Token::new(
            TokenType::StringLiteral(StringLiteral),
            position,
            &capture_text,
        ))
    }
}

impl Matches for Whitespace {
    fn matches(input: &String, position: usize) -> Option<Token> {
        let first = input.chars().nth(position)?;

        if first.eq(&' ') {
            return Some(Token::new(
                TokenType::Whitespace(Whitespace),
                position,
                &" ".to_string(),
            ));
        }

        None
    }
}

impl Matches for Lparen {
    fn matches(input: &String, position: usize) -> Option<Token> {
        return match_value(
            TokenType::Lparen(Lparen),
            &"(".to_string(),
            input,
            position,
            None,
            None,
        );
    }
}

impl Matches for Rparen {
    fn matches(input: &String, position: usize) -> Option<Token> {
        return match_value(
            TokenType::Rparen(Rparen),
            &")".to_string(),
            input,
            position,
            None,
            None,
        );
    }
}

impl Matches for Lbracket {
    fn matches(input: &String, position: usize) -> Option<Token> {
        return match_value(
            TokenType::Lbracket(Lbracket),
            &"[".to_string(),
            input,
            position,
            None,
            None,
        );
    }
}

impl Matches for Rbracket {
    fn matches(input: &String, position: usize) -> Option<Token> {
        return match_value(
            TokenType::Rbracket(Rbracket),
            &"]".to_string(),
            input,
            position,
            None,
            None,
        );
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

impl Token {
    fn new(token_type: TokenType, position: usize, value: &String) -> Self {
        Self {
            token_type,
            start: position,
            end: position + value.len(),
            value: value.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::tokens::{Lbracket, Lparen, Rbracket, Rparen, StringLiteral, Whitespace};

    use super::{Add, Identifier, Matches};

    #[test]
    fn matches_add() {
        let result = Add::matches(&"+".to_string(), 0).unwrap();

        assert_eq!(result.value, "+".to_string());
        assert_eq!(result.start, 0);
        assert_eq!(result.end, 1);
    }

    #[test]
    fn matches_identifier() {
        let result = Identifier::matches(&"Patient".to_string(), 0).unwrap();

        assert_eq!(result.value, "Patient".to_string());
        assert_eq!(result.start, 0);
        assert_eq!(result.end, "Patient".len());
    }

    #[test]
    fn matches_string_literal() {
        let result = StringLiteral::matches(&"'Patient'".to_string(), 0).unwrap();

        assert_eq!(result.value, "'Patient'".to_string());
        assert_eq!(result.start, 0);
        assert_eq!(result.end, "'Patient'".len());
    }

    #[test]
    fn matches_whitespace() {
        let result = Whitespace::matches(&" ".to_string(), 0).unwrap();

        assert_eq!(result.value, " ".to_string());
        assert_eq!(result.start, 0);
        assert_eq!(result.end, " ".len());
    }

    #[test]
    fn matches_lparen() {
        let result = Lparen::matches(&"(".to_string(), 0).unwrap();

        assert_eq!(result.value, "(".to_string());
        assert_eq!(result.start, 0);
        assert_eq!(result.end, "(".len());
    }

    #[test]
    fn matches_rparen() {
        let result = Rparen::matches(&")".to_string(), 0).unwrap();

        assert_eq!(result.value, ")".to_string());
        assert_eq!(result.start, 0);
        assert_eq!(result.end, ")".len());
    }

    #[test]
    fn matches_lbracket() {
        let result = Lbracket::matches(&"[".to_string(), 0).unwrap();

        assert_eq!(result.value, "[".to_string());
        assert_eq!(result.start, 0);
        assert_eq!(result.end, "[".len());
    }

    #[test]
    fn matches_rbracket() {
        let result = Rbracket::matches(&"]".to_string(), 0).unwrap();

        assert_eq!(result.value, "]".to_string());
        assert_eq!(result.start, 0);
        assert_eq!(result.end, "]".len());
    }
}
