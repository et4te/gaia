use std::collections::HashMap;
use expression::*;

type Identifier = String;

#[derive(PartialEq, Clone, Debug)]
pub struct L1Environment(pub HashMap<Identifier, L1Expression>);

impl L1Environment {
    pub fn new() -> L1Environment {
        L1Environment(HashMap::new())
    }

    pub fn lookup(&self, x: Identifier) -> &L1Expression {
        match self.0.get(&x) {
            Some(xi) => xi,
            None => panic!(format!("Undefined identifier {}", x)),
        }
    }

    pub fn define(&mut self, id: Identifier, x: L1Expression) {
        self.0.insert(id, x);
    }

    pub fn merge(&mut self, other: L1Environment) {
        self.0.extend(other.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Definition {
    pub id: Identifier,
    pub equation: Expression,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Environment(pub Vec<Definition>);

impl Environment {
    pub fn new() -> Environment {
        Environment(vec![])
    }

    pub fn lookup(&self, id: Identifier) -> Expression {
        for x in self.0.clone() {
            if x.id == id {
                return x.equation.clone();
            }
        }
        panic!("Undefined identifier {}")
    }

    pub fn define(&mut self, id: Identifier, x: Expression) {
        self.0.push(Definition {
            id: id,
            equation: x,
        })
    }

    pub fn merge(&mut self, other: Environment) {
        self.0.extend(other.0)
    }
}
