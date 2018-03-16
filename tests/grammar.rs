extern crate colored;
extern crate gaia;

use gaia::evaluate;
use gaia::value::Value;
use gaia::expression::Literal;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use gaia::transform_l1_dimensions;

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

    let p1 = scope(tuple_source_1.as_ref()).unwrap();

    evaluate(p1[0].clone());
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

    let p1 = scope(perturb_source_1.as_ref()).unwrap();
    evaluate(p1[0].clone());

    let p1 = scope(perturb_source_2.as_ref()).unwrap();
    evaluate(p1[0].clone());
}

#[test]
fn test_query() {
    assert!(expression("#.t").is_ok());
}

#[test]
fn test_variable_declaration() {
    assert!(function_or_variable_declaration("x = 0").is_ok());
    assert!(function_or_variable_declaration("x = x + y").is_ok());
    assert!(function_or_variable_declaration("A [x <- 0, y <- 0] = 0").is_ok());
}

#[test]
fn test_function_declaration() {
    assert!(function_or_variable_declaration("f.d = 0").is_ok());
    // println!("{:?}", function_or_variable_declaration("f.d = 0"));

    assert!(
        function_or_variable_declaration("fby.t X Y = if #.t <= 0 then X else Y @ [t <- #.t - 1]")
            .is_ok()
    );
    // println!("{:?}", function_or_variable_declaration("f.d X = X"));
}

#[test]
fn test_function_application() {
    assert!(expression("f.d").is_ok());
    assert!(expression("f.x.y").is_ok());
    assert!(expression("f.x.y!z A").is_ok());
    assert!(expression("f A").is_ok());
    assert!(expression_where("fby.t X Y").is_ok());
    assert!(expression("f.d + g A").is_ok());
    assert!(expression("wvr.d (next.d X) (next.d Y)").is_ok());
}

#[test]
fn test_intension() {
    let intension_source_1 = read_source("./isrc/intension_1.i");
    let intension_source_2 = read_source("./isrc/intension_2.i");

    let intension_test_1 = scope(intension_source_1.as_ref()).unwrap();
    let intension_test_2 = scope(intension_source_2.as_ref()).unwrap();

    let intension_test_1_result = evaluate(intension_test_1[0].clone()).expect_value();

    assert_eq!(Value::Literal(Literal::Int32(0)), intension_test_1_result);

    let intension_test_2_result = evaluate(intension_test_2[0].clone()).expect_value();

    assert_eq!(Value::Literal(Literal::Int32(3)), intension_test_2_result);
}

#[test]
fn test_fib_stream() {
    let fib_source = read_source("./isrc/fib_stream.i");

    assert!(scope(fib_source.as_ref()).is_ok());
    let body = scope(fib_source.as_ref()).unwrap();
    // println!("body == {:?}", body[0].clone());
    evaluate(body[0].clone());
}

#[test]
fn test_fib_function() {
    let fib_function_source = read_source("./isrc/fib_function.i");
    assert!(scope(fib_function_source.as_ref()).is_ok());
    let body = scope(fib_function_source.as_ref()).unwrap();
    let mut dimensions = HashMap::new();
    let mut names = HashSet::new();
    let q_dimensions = HashSet::new();
    let (x, _) = transform_l1_dimensions(
        body[0].clone(),
        &mut dimensions,
        &mut names,
        0,
        q_dimensions,
    );
    // println!("body == {:?}", x.clone());
    let result = evaluate(body[0].clone());
    // println!("result == {:?}", result.clone());
}

#[test]
fn test_naturals() {
    let naturals_source = read_source("./isrc/naturals.i");
    assert!(scope(naturals_source.as_ref()).is_ok());
    let body = scope(naturals_source.as_ref()).unwrap();
    let mut dimensions = HashMap::new();
    let mut names = HashSet::new();
    let q_dimensions = HashSet::new();
    let (x, _) = transform_l1_dimensions(
        body[0].clone(),
        &mut dimensions,
        &mut names,
        0,
        q_dimensions,
    );
    //println!("body == {:?}", x.clone());
    let result = evaluate(body[0].clone());
    // println!("result == {:?}", result.clone());
}

#[test]
fn test_wvr() {
    let wvr_source = read_source("./isrc/wvr.i");
    assert!(scope(wvr_source.as_ref()).is_ok());
    let body = scope(wvr_source.as_ref()).unwrap();
    let mut dimensions = HashMap::new();
    let mut names = HashSet::new();
    let q_dimensions = HashSet::new();
    let (x, _) = transform_l1_dimensions(
        body[0].clone(),
        &mut dimensions,
        &mut names,
        0,
        q_dimensions,
    );
    // println!("body == {:?}", x.clone());
    let result = evaluate(body[0].clone());
    // println!("result == {:?}", result.clone());
}

#[test]
fn test_fib_sum() {
    let fib_sum_source = read_source("./isrc/fib_sum.i");
    assert!(scope(fib_sum_source.as_ref()).is_ok());
    let body = scope(fib_sum_source.as_ref()).unwrap();
    let mut dimensions = HashMap::new();
    let mut names = HashSet::new();
    let q_dimensions = HashSet::new();
    let (x, _) = transform_l1_dimensions(
        body[0].clone(),
        &mut dimensions,
        &mut names,
        0,
        q_dimensions,
    );
    // println!("body == {:?}", x.clone());
    let result = evaluate(body[0].clone());
    // println!("result == {:?}", result.clone());
}

#[test]
fn test_prelude() {
    let prelude_source = read_source("./isrc/prelude.i");
    assert!(scope(prelude_source.as_ref()).is_ok());
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
