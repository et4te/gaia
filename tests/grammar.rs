extern crate gaia;
extern crate colored;

use gaia::{transform_l1_dimensions};
use gaia::evaluate;
use gaia::print_expression;
use std::collections::{HashMap, HashSet};

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

use self::grammar::*;

#[test]
fn test_boolean() {
    assert!(boolean("true").is_ok());
    assert!(boolean("false").is_ok());

    let expr = boolean("true").unwrap();

    println!("boolean => {:?}", evaluate(expr));
}

#[test]
fn test_integer() {
    assert!(integer("1234").is_ok());

    let expr = integer("1234").unwrap();

    println!("integer => {:?}", evaluate(expr));
}

#[test]
fn test_expression() {
    assert!(expression("100 + 100").is_ok());

    let expr = expression("100 + 100").unwrap();

    println!("expression => {:?}", evaluate(expr));
}

#[test]
fn test_tuple_builder() {
    assert!(tuple_builder("[ t <- 0 ]").is_ok());
    assert!(tuple_builder("[t <- 0, s <- 1]").is_ok());

    let expr = tuple_builder("[0 <- 0, 1 <- 1]").unwrap();

    println!("tuple_builder => {:?}", evaluate(expr));
}

#[test]
fn test_if() {
    assert!(conditional("if X then Y else Z").is_ok());
}

#[test]
fn test_perturb() {
    assert!(expression("A @ [t <- 0]").is_ok());
    assert!(expression("A @ [t <- #.t]").is_ok());
    //println!("{:?}", expression("A @ [t <- 0]").unwrap());
}

#[test]
fn test_query() {
    assert!(expression("#.t").is_ok());
}

#[test]
fn test_let() {
    assert!(variable_declaration("let x = 0").is_ok());
    assert!(variable_declaration("let x = x + y").is_ok());
    //println!("{:?}", variable_declaration("let x = x + y").is_ok());
}

// const M: &str = "
// M @ [a <- 5, b <- 6]
// where
//   dim a <- 0
//   dim b <- 0

//   let M = #.a * #.b
// end
// ";

// const M: &str = "
// N @ [t <- 4, s <- 2]
// where
//   dim t <- 0
//   dim s <- 0

//   let N = if #.s > 0 then #.s else #.s + #.t
// end
// ";

// const EX1: &str = "
// WA @ [t <- 0, x <- X, y <- Y]
// where
//   dim t <- 0
//   dim x <- 0
//   dim y <- 0

//   let Y = if #.t <= 1 then false else true
//   let X = if #.t <= 1 then false else true

//   let WB =
//     if #.t <= 0 then
//       #.x
//     else
//       WA @ [x <- #.x @ [t <- #.t + 1], y <- #.y @ [t <- #.t + 1]]

//   let WA =
//     if #.y @ [t <- 0] then
//       WB
//     else
//       WA @ [x <- #.x @ [t <- #.t + 1], y <- #.y @ [t <- #.t + 1]]
// end
// ";

const FIB: &str = "
F @ [n <- 20]
where
  dim n <- 0

  let F =
    if #.n <= 1 then
      #.n
    else
      F @ [n <- #.n - 1] + F @ [n <- #.n - 2]
end
";

// const SOURCE: &str = "
// X @ [t <- 3]
// where
//   dim t <- 0
//   dim s <- 0

//   let Y = X @ [t <- #.t - 1, s <- #.s + 1]
//   let X =
//     if #.t <= 0 then
//       #.s
//     else
//       Y + 1
// end
// ";

#[test]
fn test_top_level() {
    assert!(top_level(FIB).is_ok());
    let p1 = top_level(FIB).unwrap();
    let mut dims = HashMap::new();
    let q_dimensions = HashSet::new();
    let (p2, _) = transform_l1_dimensions(p1.clone(), &mut dims, 0, q_dimensions);
    println!("");
    println!("{}", print_expression(p2.clone(), 0));
    println!("");
    println!("{:?}", evaluate(p1));
}
