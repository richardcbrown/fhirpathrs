use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    As,
    Is,
    Dot,
    Eq,
    Gt,
    Ge,
    Lt,
    Le,
    Ne,
    Unkown,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::And => write!(f, "and"),
            Operator::Or => write!(f, "or"),
            Operator::Xor => write!(f, "xor"),
            Operator::As => write!(f, "as"),
            Operator::Is => write!(f, "is"),
            Operator::Dot => write!(f, "."),
            Operator::Eq => write!(f, "="),
            Operator::Gt => write!(f, ">="),
            Operator::Ge => write!(f, ">"),
            Operator::Lt => write!(f, "<="),
            Operator::Le => write!(f, "<"),
            Operator::Ne => write!(f, "!="),
            Operator::Unkown => write!(f, "??"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    WhiteSpace,
    Colon,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Symbol(String),
    Text(String),
    DateTime(String),
    Number(String),
    Operator(Operator),
    Comparator(Operator),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub pos:usize,
    pub token_type: TokenType,
}

pub struct Tokeniser<'l>{
    input: &'l [u8],
    index: usize,
}

#[derive(Debug)]
pub struct TokeniserError {
    pub msg: String
}

impl<'l> Tokeniser<'l> {
    
    pub fn new(input: &'l str) -> Self {

        Self {
            input: input.as_bytes(),
            index: 0,
        }
    }

    pub fn tokenise(&mut self) -> Result<Vec<Token>, TokeniserError> {
        let mut tokens = Vec::new();

        loop {
            match self.next_char() {
                Some(ch) => {
                    let token = match ch {
                        b' ' | b'\n' | b'\t' | b'\r' => continue,
                        b'[' => Token{ pos: self.index, token_type: TokenType::OpenBracket },
                        b']' => Token{ pos: self.index, token_type: TokenType::CloseBracket },
                        b'(' => Token{ pos: self.index, token_type: TokenType::OpenParen },
                        b')' => Token{ pos: self.index, token_type: TokenType::CloseParen },
                        b',' => Token{ pos: self.index, token_type: TokenType::Colon },
                        b'+' | b'-' | b'*' | b'/' | b'.' => self.parse_operator(ch)?,
                        b'=' | b'!' | b'>' | b'<' | b'~' => self.parse_equality(ch)?,
                        b'@' => self.parse_datetime(ch)?,
                        b'\"' | b'\'' => self.parse_text(ch)?,
                        b'0'..=b'9' => self.parse_number(ch)?,
                        b'a'..=b'z' | b'A'..=b'Z' => self.parse_symbol(ch)?,
                        _ => return Err(TokeniserError { msg: format!("Unknown character {}", ch) })
                    };
                    tokens.push(token);
                }
                None => break,
            }
        }

        Ok(tokens)
    }

    fn next_char(&mut self) -> Option<u8> {
        if self.index < self.input.len() {
            let ch = self.input[self.index];
            self.index += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn eat_char(&mut self) {
        self.index += 1;
    }

    fn peek(&mut self) -> Option<u8> {
        if self.index < self.input.len() {
            Some(self.input[self.index])
        } else {
            None
        }
    }

    // fn parse_escape(&self) {

    // }

    // TODO: The time format has not been processed yet
    fn parse_datetime(&mut self, _ch: u8) -> Result<Token, TokeniserError> {
        Ok(Token{ pos: self.index, token_type: TokenType::WhiteSpace })
    }

    fn parse_text(&mut self, _ch: u8) -> Result<Token, TokeniserError> {
        let mut scratch = vec![];

        loop {
            match self.peek() {
                Some(ch) => {
                    match ch {
                        b'\'' | b'\"' => {
                            self.eat_char();
                            break;
                        },
                        _ => {
                            scratch.push(ch);
                            self.eat_char();
                        },
                    }
                }
                None => break,
            }
        }

        match String::from_utf8(scratch) {
            Ok(text) => Ok(Token{ pos: self.index, token_type: TokenType::Text(text) }),
            Err(_) => Err(TokeniserError { msg: "Failed to parse text".to_string() }),
        }
    }

    fn parse_operator(&mut self, ch: u8) -> Result<Token, TokeniserError> {
        match ch {
            b'+' => Ok(Token{ pos: self.index, token_type: TokenType::Operator(Operator::Add) }),
            b'-' => Ok(Token{ pos: self.index, token_type: TokenType::Operator(Operator::Sub) }),
            b'*' => Ok(Token{ pos: self.index, token_type: TokenType::Operator(Operator::Mul) }),
            b'/' => Ok(Token{ pos: self.index, token_type: TokenType::Operator(Operator::Div) }),
            b'.' => Ok(Token{ pos: self.index, token_type: TokenType::Operator(Operator::Dot) }),
            _ => unreachable!()
        }
    }

    fn parse_equality(&mut self, ch: u8) -> Result<Token, TokeniserError> {
        match ch {
            b'=' | b'~' => Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Operator::Eq) }),
            b'!' => {
                match self.peek() {
                    Some(next) => {
                        match next {
                            b'=' | b'~' => {
                                self.eat_char();
                                Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Operator::Ne) })
                            },
                            _ => Err(TokeniserError { msg: format!("Unknown character {}", next) }),
                        }
                    }
                    None => Err(TokeniserError { msg: format!("Invalid equality") }),
                }
            },
            b'>' => {
                match self.peek() {
                    Some(next) => {
                        match next {
                            b'=' => {
                                self.eat_char();
                                Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Operator::Gt) })
                            },
                            _ => Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Operator::Ge) }),
                        }
                    }
                    None => Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Operator::Ge) }),
                }
            }
            b'<' => {
                match self.peek() {
                    Some(next) => {
                        match next {
                            b'=' => {
                                self.eat_char();
                                Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Operator::Lt) })
                            },
                            _ => Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Operator::Le) }),
                        }
                    }
                    None => Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Operator::Le) }),
                }
            }
            _ => unreachable!()
        }
    }

    fn parse_number(&mut self, ch: u8) -> Result<Token, TokeniserError> {
        let mut scratch = vec![ch];

        loop {
            match self.peek() {
                Some(ch) => {
                    match ch {
                        b'0'..=b'9' | b'.' => {
                            scratch.push(ch);
                            self.eat_char();
                        },
                        _ => break,
                    }
                }
                None => break,
            }
        }

        match String::from_utf8(scratch) {
            Ok(number) => Ok(Token{ pos: self.index, token_type: TokenType::Number(number) }),
            Err(_) => Err(TokeniserError { msg: "Failed to parse number".to_string() }),
        }
    }

    fn parse_symbol(&mut self, ch: u8) -> Result<Token, TokeniserError> {
        let mut scratch = vec![ch];
        loop {
            match self.peek() {
                Some(ch) => {
                    match ch {
                        b'a'..=b'z' | b'A'..=b'Z' => {
                            scratch.push(ch);
                            self.eat_char();
                        },
                        _ => break,
                    }
                }
                None => break,
            }
        }
        
        match String::from_utf8(scratch) {
            Ok(symbol) => {
                match symbol.as_str() {
                    "and" => Ok(Token{ pos: self.index, token_type: TokenType::Operator(Operator::And) }),
                    "or" => Ok(Token{ pos: self.index, token_type: TokenType::Operator(Operator::Or) }),
                    "xor" => Ok(Token{ pos: self.index, token_type: TokenType::Operator(Operator::Xor) }),
                    "as" => Ok(Token{ pos: self.index, token_type: TokenType::Operator(Operator::As) }),
                    "is" => Ok(Token{ pos: self.index, token_type: TokenType::Operator(Operator::Is) }),
                    _ => Ok(Token{ pos: self.index, token_type: TokenType::Symbol(symbol) })
                }
            },
            Err(_) => Err(TokeniserError { msg: "Failed to parse symbol".to_string() }),
        }
    }
}

#[test]
pub fn test_tokeniser() {
    let mut tokeniser = Tokeniser::new("Patient.name[0].given[2] >= 'abc.fgh>10'");

    let list = tokeniser.tokenise().unwrap();

    assert_eq!(list[0].token_type, TokenType::Symbol("Patient".to_string()));
    assert_eq!(list[1].token_type, TokenType::Operator(Operator::Dot));
    assert_eq!(list[2].token_type, TokenType::Symbol("name".to_string()));
    assert_eq!(list[3].token_type, TokenType::OpenBracket);
    assert_eq!(list[4].token_type, TokenType::Number("0".to_string()));
    assert_eq!(list[5].token_type, TokenType::CloseBracket);
    assert_eq!(list[6].token_type, TokenType::Operator(Operator::Dot));
    assert_eq!(list[7].token_type, TokenType::Symbol("given".to_string()));
    assert_eq!(list[8].token_type, TokenType::OpenBracket);
    assert_eq!(list[9].token_type, TokenType::Number("2".to_string()));
    assert_eq!(list[10].token_type, TokenType::CloseBracket);
    assert_eq!(list[11].token_type, TokenType::Comparator(Operator::Gt));
    assert_eq!(list[12].token_type, TokenType::Text("abc.fgh>10".into()));
}