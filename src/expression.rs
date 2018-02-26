use environment::{Environment, L1Environment};
use value::Dimension;

type Identifier = String;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Literal {
    Bool(bool),
    Int32(u32),
}

#[derive(PartialEq, Clone, Debug)]
pub struct L1TupleExpression {
    pub lhs: L1Expression,
    pub rhs: L1Expression,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct TupleExpression {
    pub lhs: Expression,
    pub rhs: Expression,
}

#[derive(PartialEq, Clone, Debug)]
pub struct L1BaseAbstraction {
    pub id: Identifier,
    pub expression: L1Expression,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct BaseAbstraction {
    pub param: Dimension,
    pub dimensions: Vec<Dimension>,
    pub expression: Expression,
}

#[derive(PartialEq, Clone, Debug)]
pub struct L1IntensionExpression {
    pub domain: Vec<L1Expression>,
    pub value: L1Expression,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct IntensionExpression {
    pub domain: Vec<Expression>,
    pub value: Expression,
}

#[derive(PartialEq, Clone, Debug)]
pub struct L1IfExpression {
    pub condition: L1Expression,
    pub consequent: L1Expression,
    pub alternate: L1Expression,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct IfExpression {
    pub condition: Expression,
    pub consequent: Expression,
    pub alternate: Expression,
}

#[derive(PartialEq, Clone, Debug)]
pub struct L1WhereVarExpression {
    pub lhs: L1Expression,
    pub rhs: L1Environment,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct WhereVarExpression {
    pub lhs: Expression,
    pub rhs: Environment,
}

#[derive(PartialEq, Clone, Debug)]
pub struct L1PerturbExpression {
    pub lhs: L1Expression,
    pub rhs: L1Expression,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct PerturbExpression {
    pub lhs: Expression,
    pub rhs: Expression,
}

#[derive(PartialEq, Clone, Debug)]
pub struct L1DimensionExpression {
    pub lhs: Identifier,
    pub rhs: L1Expression,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct DimensionExpression {
    pub lhs: Dimension,
    pub rhs: Expression,
}

#[derive(PartialEq, Clone, Debug)]
pub struct L1ContextExpression(pub Vec<L1DimensionExpression>);

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct ContextExpression(pub Vec<DimensionExpression>);

#[derive(PartialEq, Clone, Debug)]
pub struct L1WhereDimExpression {
    pub lhs: L1Expression,
    pub rhs: L1ContextExpression,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct WhereDimExpression {
    pub nat_q: u32,       // The wheredim label q E N
    pub dim_q: Dimension, // A unique dimension (since q is unique)
    pub lhs: Expression,
    pub rhs: ContextExpression,
}

#[derive(PartialEq, Clone, Debug)]
pub struct L1DeclarationExpression {
    pub lhs: L1Expression,
    pub tuple_builder: Option<L1Expression>,
    pub rhs: L1Expression,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct DeclarationExpression {
    pub lhs: Expression,
    pub rhs: Expression,
}

#[derive(PartialEq, Clone, Debug)]
pub struct L1WhereExpression {
    pub lhs: L1Expression,
    pub rhs: Vec<L1Expression>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum L1Expression {
    DimDeclaration(Box<L1DeclarationExpression>),
    VarDeclaration(Box<L1DeclarationExpression>),
    Literal(Literal),
    Operator(Identifier),
    Sequence(Vec<L1Expression>),
    TupleBuilder(Vec<L1TupleExpression>),
    BaseAbstraction(Box<L1BaseAbstraction>),
    IntensionBuilder(Box<L1IntensionExpression>),
    IntensionApplication(Box<L1Expression>),
    Application(Vec<L1Expression>),
    If(Box<L1IfExpression>),
    WhereVar(Box<L1WhereVarExpression>),
    Query(Box<L1Expression>),
    Perturb(Box<L1PerturbExpression>),
    Identifier(Identifier),
    WhereDim(Box<L1WhereDimExpression>),
}

impl L1Expression {
    pub fn as_identifier(&self) -> Identifier {
        match self.clone() {
            L1Expression::Identifier(id) => id,

            _ => panic!("Expected identifier"),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Expression {
    Literal(Literal),
    Dimension(Dimension),
    Operator(Identifier),
    Sequence(Vec<Expression>),
    TupleBuilder(Vec<TupleExpression>),
    BaseAbstraction(Box<BaseAbstraction>),
    IntensionBuilder(Box<IntensionExpression>),
    IntensionApplication(Box<Expression>),
    Application(Vec<Expression>),
    If(Box<IfExpression>),
    WhereVar(Box<WhereVarExpression>),
    Query(Box<Expression>),
    Perturb(Box<PerturbExpression>),
    Identifier(Identifier),
    WhereDim(Box<WhereDimExpression>),
}

impl Expression {
    pub fn as_identifier(&self) -> Identifier {
        match self.clone() {
            Expression::Identifier(id) => id,

            _ => panic!("Expected identifier"),
        }
    }
}
