extern crate gaia;

use gaia::context::Context;
use gaia::domain::Domain;
use gaia::value::{Dimension, Value};
use gaia::expression::Literal;

//fn linear_context(dims: [&str], vals: [i32]) -> Context {
//    let mut k = Context::new();
//    for i in 0.. {
//        k.push(
//            Dimension {
//                i: i,
//                v: Value::Identifier(dims[i].to_string()),
//            },
//            Value::Literal(Literal::Int32(vals[i])),
//        );
//    }
//    k
//}

//fn build_domain(dims: Vector<&str>) -> Domain {
//    let mut d = Domain::new();
//    for i in 0.. {
//        d.push(Dimension {
//            i: i,
//            v: Value::Identifier(dims[i].to_string()),
//        })
//    }
//    d
//}

//#[test]
//fn test_restrict() {
//    // k : [t <- 0]
//    let k = linear_context(vec!["t"], vec![0]);
//    // d : {t}
//    let d = build_domain(vec!["t"]);
//    let r = k.clone().restrict(d);
//    assert_eq!(k, r);
//}

// #[test]
// fn test_perturb() {
//    let k1 = Context::new();
//    let k2 = Context::new();
//}

//#[test]
//fn test_lookup() {
//    let k = Context::new();
//}
