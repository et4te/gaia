use colored::*;
use expression::*;
use context::*;

type Identifier = String;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Dimension {
    pub i: u32,
    pub v: Value,
}

pub fn print_dimension(d: Dimension) -> String {
    let s = print_value(d.v);
    let i = format!("{}", d.i).bright_white();
    format!("({}:{})", i, s)
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Intension {
    pub k: Context,
    pub d: Vec<Dimension>,
    pub x: Box<Expression>,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Value {
    Literal(Literal),
    Dimension(Box<Dimension>),
    Intension(Box<Intension>),
    BaseAbstraction(Box<BaseAbstraction>),
    ValueAbstraction(Box<ValueAbstraction>),
    NameAbstraction(Box<NameAbstraction>),
    Identifier(String),
    Context(Context),
    PrimOp(Identifier),
}

impl Value {
    pub fn expect_integer(&self) -> u32 {
        match self {
            &Value::Literal(Literal::Int32(n)) => n,
            other => panic!("Expected u32 but here found {:?}", other),
        }
    }

    pub fn expect_dimension(&self) -> Dimension {
        match self {
            &Value::Dimension(ref di) => *di.clone(),

            other => panic!("Expected dimension but here found {:?}", other),
        }
    }

    pub fn expect_base_abstraction(&self) -> BaseAbstraction {
        match self {
            &Value::BaseAbstraction(ref base_abstraction) => *base_abstraction.clone(),

            _ => panic!("Expected base_abstraction."),
        }
    }

    pub fn expect_intension(&self) -> Intension {
        match self {
            &Value::Intension(ref intens) => *intens.clone(),

            _ => panic!("Expected intension."),
        }
    }
}

pub fn print_value(v: Value) -> String {
    match v.clone() {
        Value::Literal(lit) => match lit {
            Literal::Bool(b) => {
                let s = format!("{:?}", b).bright_cyan();
                format!("{}", s)
            }
            Literal::Int32(i) => {
                let s = format!("{:?}", i).bright_cyan();
                format!("{}", s)
            }
        },

        Value::Dimension(di) => print_dimension(*di),

        Value::Identifier(id) => format!("{}", id.bright_yellow()),

        Value::BaseAbstraction(base_abstraction) => {
            let mut s = format!(".\\ {:?} -> ", base_abstraction.dimensions.clone());
            s = format!(
                "{} {}",
                s,
                super::print_expression(base_abstraction.body, 0)
            );
            s
        }

        Value::ValueAbstraction(value_abstraction) => {
            let mut s = format!("!\\ {:?} -> ", value_abstraction.dimensions.clone());
            s = format!(
                "{} {}",
                s,
                super::print_expression(value_abstraction.body, 0)
            );
            s
        }

        Value::NameAbstraction(name_abstraction) => {
            let mut s = format!(" \\ {:?} -> ", name_abstraction.dimensions.clone());
            s = format!(
                "{} {}",
                s,
                super::print_expression(name_abstraction.body, 0)
            );
            s
        }

        Value::Intension(intens) => {
            let mut s = format!("{}", intens.clone().k.clone().domain().print());
            s = format!("{} {}", s, super::print_expression(*intens.clone().x, 0));
            s = format!("{} {}", s, intens.k.clone().print());
            s
        }

        Value::Context(k) => format!("{:?}", k),

        Value::PrimOp(op) => format!("{}", op.bright_white()),
    }
}
