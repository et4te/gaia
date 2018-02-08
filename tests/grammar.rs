extern crate colored;
extern crate gaia;

use gaia::transform_l1_dimensions;
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

const TUPLE_TEST_1: &str = "
[t <- 0, s <- 0]
where
  dim t <- 0
  dim s <- 0
end
";

#[test]
fn test_tuple_builder() {
    assert!(tuple_builder("[ t <- 0 ]").is_ok());
    assert!(tuple_builder("[t <- 0, s <- 1]").is_ok());

    let p1 = top_level(TUPLE_TEST_1).unwrap();
    let mut dims = HashMap::new();
    let q_dimensions = HashSet::new();
    let (p2, _) = transform_l1_dimensions(p1.clone(), &mut dims, 0, q_dimensions);
    println!("");
    println!("{}", print_expression(p2.clone(), 0));
    println!("");
    evaluate(p1);
}

#[test]
fn test_if() {
    assert!(conditional("if X then Y else Z").is_ok());
}

const PERTURB_TEST_1: &str = "
X @ [t <- 0]
where
  dim t <- 0

  X = #.t
end
";

const PERTURB_TEST_2: &str = "
X @ [t <- 3, s <- 3]
where
  dim t <- 0
  dim s <- 0

  X = #.t + #.s
end
";

#[test]
fn test_perturb() {
    assert!(expression("A @ [t <- 0]").is_ok());
    assert!(expression("A @ [t <- #.t]").is_ok());

    let p1 = top_level(PERTURB_TEST_1).unwrap();
    evaluate(p1);

    let p1 = top_level(PERTURB_TEST_2).unwrap();
    evaluate(p1);
}

#[test]
fn test_query() {
    assert!(expression("#.t").is_ok());
}

#[test]
fn test_let() {
    assert!(variable_declaration("x = 0").is_ok());
    assert!(variable_declaration("x = x + y").is_ok());
    assert!(variable_declaration("A [x <- 0, y <- 0] = 0").is_ok());
    //println!("{:?}", variable_declaration("let x = x + y").is_ok());
}

const INTENSION_TEST_1: &str = "
=> time_three @ [t <- 0]
where
  dim t <- 0

  time_three = time @ [t <- 3]
  time = {t} #.t
end
";

#[test]
fn test_intension() {
    let p1 = top_level(INTENSION_TEST_1).unwrap();
    let mut dims = HashMap::new();
    let q_dimensions = HashSet::new();
    let (p2, _) = transform_l1_dimensions(p1.clone(), &mut dims, 0, q_dimensions);
    // println!("");
    // println!("{}", print_expression(p2.clone(), 0));
    // println!("");
    println!("{:?}", evaluate(p1));
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
fib @ [n <- 30]
where
  dim n <- 0

  fib =
    if #.n <= 1 then
      #.n
    else
      (fib @ [n <- #.n - 1]) + (fib @ [n <- #.n - 2])
end
";

// "
// W
// where
//   dim d <- 2
//   dim a <- 0
//   dim b <- 0

//   let A = if #.a <= 0 then 1 else A @ [a <- #.a - 1] + 1
//   let B = if #.b <= 0 then 1 else B @ [b <- #.b - 1] + 1

//   let Ar = A @ [a <- #.d]
//   let Br = B @ [b <- #.d]

//   let Z = Ar * Br

//   let W = if #.d <= 0 then Z else W @ [d <- #.d - 1] + Z
// end
// ";

// const SOURCE: &str = "
// wvr @ [d <- 0, x <- ({} X), y <- ({} Y)]
// where
//   dim d <- 0
//   dim x <- 0
//   dim y <- 0

//   let X = if #.d <= 0 then false else true
//   let Y = if #.d <= 0 then false else true

//   let wvr =
//     if |> (#.y @ [d <- 0]) then
//       if #.d <= 0 then
//         (|> #.x)
//       else
//         wvr @ [d <- #.d - 1, x <- #.x @ [d <- #.d + 1], y <- #.y @ [d <- #.d + 1]]
//     else
//       wvr @ [d <- #.d, x <- #.x @ [d <- #.d + 1], y <- #.y @ [d <- #.d + 1]]
// end
// ";

// const SOURCE: &str = "
// ((\\ p -> p) . x)
// where
//   dim x <- 0
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
