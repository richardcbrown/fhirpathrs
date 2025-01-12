#[derive(Debug)]
pub struct LiteralTerm {
    pub children: Vec<Box<Literal>>,
}

#[derive(Debug)]
pub enum Literal {
    NullLiteral(Box<NullLiteral>),
    BooleanLiteral(Box<BooleanLiteral>),
    StringLiteral(Box<StringLiteral>),
    NumberLiteral(Box<NumberLiteral>),
    DatetimeLiteral(Box<DatetimeLiteral>),
    TimeLiteral(Box<TimeLiteral>),
    QuantityLiteral(Box<QuantityLiteral>),
}

#[derive(Debug)]
pub struct NullLiteral {
    pub text: String,
}

#[derive(Debug)]
pub struct BooleanLiteral {
    pub text: String,
}

#[derive(Debug)]
pub struct StringLiteral {
    pub text: String,
}

#[derive(Debug)]
pub struct NumberLiteral {
    pub text: String,
}

#[derive(Debug)]
pub struct DatetimeLiteral {
    pub text: String,
}

#[derive(Debug)]
pub struct TimeLiteral {
    pub text: String,
}

#[derive(Debug)]
pub struct QuantityLiteral {
    pub text: String,
    pub unit: Option<String>,
}
