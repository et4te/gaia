use value::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Tuple {
    pub dim: Dimension,
    pub ord: Value,
}

impl Tuple {
    pub fn new(dim: Dimension, ord: Value) -> Tuple {
        Tuple { dim: dim, ord: ord }
    }
}
