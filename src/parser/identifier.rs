#[derive(Debug)]
pub enum Identifier {
    LiteralIdentifier(Box<LiteralIdentifier>),
    LiteralDelimitedIdentifier(Box<LiteralDelimitedIdentifier>),
    LiteralAs(Box<LiteralAs>),
    LiteralIs(Box<LiteralIs>),
    LiteralContains(Box<LiteralContains>),
    LiteralIn(Box<LiteralIn>),
}

#[derive(Debug)]
pub struct LiteralIdentifier {
    pub text: String,
}

#[derive(Debug)]
pub struct LiteralDelimitedIdentifier {
    pub text: String,
}

#[derive(Debug)]
pub struct LiteralAs {
    pub text: String,
}

#[derive(Debug)]
pub struct LiteralIs {
    pub text: String,
}

#[derive(Debug)]
pub struct LiteralContains {
    pub text: String,
}

#[derive(Debug)]
pub struct LiteralIn {
    pub text: String,
}

#[derive(Debug)]
pub enum TypeSpecifier {
    QualifiedIdentifier(Box<QualifiedIdentifier>),
}

#[derive(Debug)]
pub struct QualifiedIdentifier {
    pub children: Vec<Box<Identifier>>,
}
