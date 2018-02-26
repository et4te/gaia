extern crate colored;
extern crate gaia;

use gaia::evaluate;
use gaia::value::Value;
use gaia::expression::Literal;
use std::fs::File;
use std::io::prelude::*;

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

use self::grammar::*;

fn read_source(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut source = String::new();
    f.read_to_string(&mut source)
        .expect("Something went wrong reading source");
    source
}

#[test]
fn test_boolean() {
    assert!(boolean("true").is_ok());
    assert!(boolean("false").is_ok());

    let test_true = evaluate(boolean("true").unwrap()).expect_value();
    let test_false = evaluate(boolean("false").unwrap()).expect_value();

    assert_eq!(test_true, Value::Literal(Literal::Bool(true)));
    assert_eq!(test_false, Value::Literal(Literal::Bool(false)));
}

#[test]
fn test_integer() {
    assert!(integer("1234").is_ok());

    let test_small_integer = evaluate(integer("1234").unwrap()).expect_value();

    assert_eq!(test_small_integer, Value::Literal(Literal::Int32(1234)));
}

#[test]
fn test_expression() {
    assert!(expression("100 + 100").is_ok());

    let test_addition = evaluate(expression("100 + 100").unwrap()).expect_value();

    assert_eq!(test_addition, Value::Literal(Literal::Int32(200)));
}

#[test]
fn test_tuple_builder() {
    assert!(tuple_builder("[ t <- 0 ]").is_ok());
    assert!(tuple_builder("[t <- 0, s <- 1]").is_ok());

    let tuple_source_1 = read_source("./isrc/tuple_1.i");

    let p1 = top_level(tuple_source_1.as_ref()).unwrap();

    evaluate(p1);
}

#[test]
fn test_if() {
    assert!(conditional("if X then Y else Z").is_ok());
}

#[test]
fn test_perturb() {
    assert!(expression("A @ [t <- 0]").is_ok());
    assert!(expression("A @ [t <- #.t]").is_ok());

    let perturb_source_1 = read_source("./isrc/perturb_1.i");
    let perturb_source_2 = read_source("./isrc/perturb_2.i");

    let p1 = top_level(perturb_source_1.as_ref()).unwrap();
    evaluate(p1);

    let p1 = top_level(perturb_source_2.as_ref()).unwrap();
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

#[test]
fn test_intension() {
    let intension_source_1 = read_source("./isrc/intension_1.i");
    let intension_source_2 = read_source("./isrc/intension_2.i");

    let intension_test_1 = top_level(intension_source_1.as_ref()).unwrap();
    let intension_test_2 = top_level(intension_source_2.as_ref()).unwrap();

    let intension_test_1_result = evaluate(intension_test_1).expect_value();

    assert_eq!(Value::Literal(Literal::Int32(0)), intension_test_1_result);

    let intension_test_2_result = evaluate(intension_test_2).expect_value();

    assert_eq!(Value::Literal(Literal::Int32(3)), intension_test_2_result);
}

#[test]
fn test_fib() {
    let fib_source = read_source("./isrc/fib.i");

    assert!(top_level(fib_source.as_ref()).is_ok());
    let p1 = top_level(fib_source.as_ref()).unwrap();

    evaluate(p1);
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
