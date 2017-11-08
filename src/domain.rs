use colored::*;
use std::collections::HashSet;

use value::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Domain(pub HashSet<Dimension>);

impl Domain {

    pub fn new() -> Domain {
        Domain(HashSet::new())
    }

    pub fn len(&mut self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, dim: Dimension) -> bool {
        self.0.insert(dim)
    }

    pub fn contains(&self, dim: Dimension) -> bool {
        self.0.contains(&dim)
    }

    pub fn is_subset(&mut self, other: Domain) -> bool {
        self.0.is_subset(&other.0)
    }

    pub fn union(&self, other: Domain) -> Domain {
        let u = self.0.union(&other.0).cloned().collect();
        Domain(u)
    }

    pub fn difference(&self, other: Domain) -> Domain {
        let d = self.0.difference(&other.0).cloned().collect();
        Domain(d)
    }

    pub fn print(&self) -> String {
        let mut s = "{".bright_white().to_string();
        let d = self.0.clone();
        if d.clone().len() > 0 {
            if d.clone().len() > 1 {
                let mut i = 0;
                for di in d.clone() {
                    if i == 0 {
                        s = format!("{}{}, ", s, print_dimension(di));
                        i += 1;
                    } else if i == (d.len() - 1) {
                        s = format!("{}{}", s, print_dimension(di));
                        i += 1;
                    } else {
                        s = format!("{}{}, ", s, print_dimension(di));
                        i += 1;
                    }
                }
            } else {
                let di: Vec<Dimension> = d.iter().cloned().collect();
                s = format!("{}{}", s, print_dimension(di[0].clone()));
            }
        }
        format!("{}{}", s, "}".bright_white())
    }
}
