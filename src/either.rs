use value::Value;
use domain::Domain;

#[derive(Clone, Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl Either<Value, Domain> {
    pub fn expect_value(&self) -> Value {
        match *self {
            Either::Left(ref l) => l.clone(),

            Either::Right(ref r) => panic!("Expected left but here found {:?}", r.clone()),
        }
    }
}
