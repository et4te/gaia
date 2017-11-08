use colored::*;

use domain::*;
use tuple::*;
use value::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Context(pub Vec<Tuple>);

impl Context {
    pub fn new() -> Context {
        Context(vec![])
    }

    pub fn push(&mut self, d: Dimension, v: Value) {
        self.0.push(Tuple{dim: d, ord: v})
    }

    pub fn domain(&self) -> Domain {
        let mut d = Domain::new();
        for tup in self.0.clone() {
            d.push(tup.dim);
        }
        d
    }

    pub fn restrict(&mut self, d: Domain) -> Context {
        let mut r = vec![];
        for tup in self.0.clone() {
            if d.clone().contains(tup.clone().dim) {
                r.push(tup);
            }
        }
        Context(r)
    }

    pub fn perturb(&mut self, other: Context) -> Context {
        let mut c = vec![];
        let diff = self.domain().difference(other.domain());
        for tup in self.0.clone() {
            if diff.contains(tup.clone().dim) {
                c.push(tup)
            }
        }

        for tup in other.0.clone() {
            c.push(tup)
        }

        Context(c)
    }

    pub fn lookup(&self, dim: Dimension) -> Option<Value> {
        for tup in self.0.clone() {
            if tup.dim == dim.clone() {
                return Some(tup.ord);
            }
        }
        None
    }

    pub fn print(&self) -> String {
        let mut s = "[".bright_white().to_string();
        let tuples = self.0.clone();
        if tuples.len() > 0 {
            for i in 0..tuples.len() {
                if i == (tuples.len() - 1) {
                    let dim = print_dimension(tuples[i].clone().dim);
                    let ord = print_value(tuples[i].clone().ord);
                    s = format!("{}{} {} {}", s, dim, "<-".bright_white(), ord);
                } else {
                    let lhs = print_dimension(tuples[i].clone().dim);
                    let rhs = print_value(tuples[i].clone().ord);
                    s = format!("{}{} <- {}, ", s, lhs, rhs);
                }
            }
        }
        format!("{}{}", s, "]".bright_white())
    }
}
