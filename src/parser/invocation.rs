use super::{expression::Expression, identifier::Identifier};

#[derive(Debug)]
pub struct InvocationTerm {
    pub children: Vec<Box<Invocation>>,
}

#[derive(Debug)]
pub enum Invocation {
    MemberInvocation(Box<MemberInvocation>),
    FunctionInvocation(Box<FunctionInvocation>),
    ThisInvocation(Box<ThisInvocation>),
    IndexInvocation(Box<IndexInvocation>),
    TotalInvocation(Box<TotalInvocation>),
}

#[derive(Debug)]
pub struct MemberInvocation {
    pub children: Vec<Box<Identifier>>,
}

#[derive(Debug)]
pub struct ParamList {
    pub children: Vec<Box<Expression>>,
}

#[derive(Debug)]
pub enum IdentifierAndParamList {
    Identifier(Box<Identifier>),
    ParamList(Box<ParamList>),
}

#[derive(Debug)]
pub struct FunctionInvocation {
    pub children: Vec<Box<IdentifierAndParamList>>,
}

#[derive(Debug)]
pub struct ThisInvocation {
    pub text: String,
}

#[derive(Debug)]
pub struct IndexInvocation {
    pub text: String,
}

#[derive(Debug)]
pub struct TotalInvocation {
    pub text: String,
}
