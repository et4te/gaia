use std::collections::HashMap;
use context::Context;
use domain::Domain;
use either::Either;
use value::Value;

type Identifier = String;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Key {
    pub x: Identifier,
    pub k: Context,
}

#[derive(Clone, Debug)]
pub struct Cache {
    pub cache: HashMap<Key, Either<Value, Domain>>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            cache: HashMap::new(),
        }
    }

    pub fn find(&mut self, x: Identifier, k: Context) -> Option<&Either<Value, Domain>> {
        self.cache.get(&Key { x: x.clone(), k: k })
    }

    pub fn add(
        &mut self,
        x: Identifier,
        k: Context,
        v: Either<Value, Domain>,
    ) -> Either<Value, Domain> {
        // println!("Inserting {} {} = {:?}", x.clone(), k.print(), v.clone());
        self.cache.insert(Key { x: x, k: k }, v.clone());
        v
    }
}
