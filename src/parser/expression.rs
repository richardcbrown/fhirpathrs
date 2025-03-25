use super::{
    identifier::{Identifier, TypeSpecifier},
    invocation::{Invocation, InvocationTerm},
    literal::{LiteralTerm, StringLiteral},
};

/**
 * expression
 *   : term                                               #termExpression
 *   | expression '.' invocation                          #invocationExpression
 *   | expression '[' expression ']'                      #indexerExpression
 *   | ('+' | '-') expression                             #polarityExpression
 *   | expression ('*' | '/' | 'div' | 'mod') expression  #multiplicativeExpression
 *   | expression ('+' | '-' | '&') expression            #additiveExpression
 *   | expression '|' expression                          #unionExpression
 *   | expression ('<=' | '<' | '>' | '>=') expression    #inequalityExpression
 *   | expression ('is' | 'as') typeSpecifier             #typeExpression
 *   | expression ('=' | '~' | '!=' | '!~') expression    #equalityExpression
 *   | expression ('in' | 'contains') expression          #membershipExpression
 *   | expression 'and' expression                        #andExpression
 *   | expression ('or' | 'xor') expression               #orExpression
 *   | expression 'implies' expression                    #impliesExpression
 *   //| (IDENTIFIER)? '=>' expression                    #lambdaExpression
 *   ;
 */
#[derive(Debug)]
pub struct EntireExpression {
    pub children: Vec<Box<Expression>>,
}

#[derive(Debug)]
pub enum Expression {
    TermExpression(Box<TermExpression>),
    InvocationExpression(Box<InvocationExpression>),
    IndexerExpression(Box<IndexerExpression>),
    PolarityExpression(Box<PolarityExpression>),
    MultiplicativeExpression(Box<MultiplicativeExpression>),
    AdditiveExpression(Box<AdditiveExpression>),
    UnionExpression(Box<UnionExpression>),
    InequalityExpression(Box<InequalityExpression>),
    TypeExpression(Box<TypeExpression>),
    EqualityExpression(Box<EqualityExpression>),
    MembershipExpression(Box<MembershipExpression>),
    AndExpression(Box<AndExpression>),
    OrExpression(Box<OrExpression>),
    ImpliesExpression(Box<ImpliesExpression>),
}

#[derive(Debug)]
pub struct ParenthesizedTerm {
    pub children: Vec<Box<Expression>>,
}

#[derive(Debug)]
pub enum Term {
    InvocationTerm(Box<InvocationTerm>),
    LiteralTerm(Box<LiteralTerm>),
    ExternalConstantTerm(Box<ExternalConstantTerm>),
    ParenthesizedTerm(Box<ParenthesizedTerm>),
}

#[derive(Debug)]
pub struct TermExpression {
    pub children: Vec<Box<Term>>,
}

#[derive(Debug)]
pub enum ExpressionAndInvocation {
    Expression(Expression),
    Invocation(Invocation),
}

#[derive(Debug)]
pub struct InvocationExpression {
    pub children: Vec<Box<ExpressionAndInvocation>>,
}

#[derive(Debug)]
pub struct IndexerExpression {
    pub children: Vec<Box<Expression>>,
}

#[derive(Debug)]
pub struct PolarityExpression {
    pub op: String,
    pub children: Vec<Box<Expression>>,
}

#[derive(Debug)]
struct OpParsedTerms {
    children: Vec<Box<Expression>>,
    op: String,
}

#[derive(Debug)]
pub struct MultiplicativeExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

#[derive(Debug)]
pub struct AdditiveExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

#[derive(Debug)]
pub struct UnionExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

#[derive(Debug)]
pub struct InequalityExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

#[derive(Debug)]
pub struct EqualityExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

#[derive(Debug)]
pub struct MembershipExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

#[derive(Debug)]
pub struct AndExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

#[derive(Debug)]
pub struct OrExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

#[derive(Debug)]
pub struct ImpliesExpression {
    pub children: Vec<Box<Expression>>,
    pub op: String,
}

#[derive(Debug)]
pub enum ExpressionAndTypeSpecifier {
    Expression(Box<Expression>),
    TypeSpecifier(Box<TypeSpecifier>),
}

#[derive(Debug)]
pub struct TypeExpression {
    pub op: String,
    pub children: Vec<Box<ExpressionAndTypeSpecifier>>,
}

#[derive(Debug)]
pub enum IdentifierOrStringLiteral {
    Identifier(Box<Identifier>),
    StringLiteral(Box<StringLiteral>),
}

#[derive(Debug)]
pub struct ExternalConstantTerm {
    pub children: Vec<Box<IdentifierOrStringLiteral>>,
}
