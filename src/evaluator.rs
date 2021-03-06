use std::collections::HashSet;
use environment::Environment;
use expression::{Expression, Literal};
use context::Context;
use domain::Domain;
use cache::Cache;
use either::Either;
use tuple::Tuple;
use value::*;

type Identifier = String;

pub fn generate_dimension(i: u32, q: u32, d: u32) -> (Dimension, Value) {
    let di = Dimension {
        i: i,
        v: Value::Literal(Literal::Int32(q + d)),
    };
    (di.clone(), Value::Dimension(Box::new(di)))
}

pub fn evaluate_id1(
    x: Identifier,
    e: &mut Environment,
    k: Context,
    init_d: Domain,
    dd: Domain,
    d: Domain,
    c: &mut Cache,
) -> Either<Value, Domain> {
    let v0 = evaluate_id2(x.clone(), e, k.clone(), init_d.clone(), dd.clone(), c);
    match v0.clone() {
        Either::Left(_) => v0,

        Either::Right(mut d0) => {
            if d0.clone().is_subset(d.clone()) {
                // println!("dd => {} is a subset of d => {}",
                //          dd.clone().print(),
                //          d.clone().print());
                if d0.is_subset(k.clone().domain()) {
                    // println!("d0 => {} is a subset of kd => {}",
                    //          d0.clone().print(),
                    //          k.clone().domain().print());
                    evaluate_id1(x, e, k, init_d, dd.clone().union(d0), d, c)
                } else {
                    println!(
                        "i: {} is not a subset of {}",
                        d0.clone().print(),
                        k.clone().domain().print()
                    );
                    v0
                }
            } else {
                println!(
                    "j: {} is not a subset of {}",
                    d0.clone().print(),
                    d.clone().print()
                );
                Either::Right(d0.difference(d))
            }
        }
    }
}

pub fn evaluate_id2(
    x: Identifier,
    e: &mut Environment,
    k: Context,
    init_d: Domain,
    d: Domain,
    c: &mut Cache,
) -> Either<Value, Domain> {
    let v0 = c.find(x.clone(), k.clone().restrict(d.clone()))
        .map(|v| v.clone());
    match v0 {
        Some(v) => v.clone(),

        None => {
            let v1 = evaluate(
                e.lookup(x.clone()).clone(),
                e,
                k.clone(),
                init_d,
                d.clone(),
                c,
            );
            match v1.clone() {
                Either::Left(v) => {
                    // println!(
                    //     "{} {} <| {} = {}",
                    //     x,
                    //     k.clone().restrict(d.clone()).print(),
                    //     d.clone().print(),
                    //     print_value(v)
                    // );
                }
                Either::Right(d1) => {
                    // println!(
                    //     "{} {} <| {} = {}",
                    //     x,
                    //     k.clone().restrict(d.clone()).print(),
                    //     d.clone().print(),
                    //     d1.clone().print()
                    // );
                }
            }
            c.add(x, k.clone().restrict(d), v1.clone());
            v1
        }
    }
}

pub fn evaluate(
    x: Expression,
    e: &mut Environment,
    k: Context,
    init_d: Domain,
    d: Domain,
    c: &mut Cache,
) -> Either<Value, Domain> {
    match x {
        Expression::Literal(lit) => Either::Left(Value::Literal(lit)),

        // Constant dimensional query
        Expression::Dimension(di) => {
            if d.contains(di.clone()) {
                let v = k.lookup(di).expect("Expected dimension in context.");
                Either::Left(v)
            } else {
                let mut h = HashSet::new();
                h.insert(di);
                Either::Right(Domain(h))
            }
        }

        Expression::Operator(id) => Either::Left(Value::Identifier(id)),

        Expression::Sequence(expr_vec) => {
            let mut r = Either::Right(Domain::new());
            for expr in expr_vec {
                r = evaluate(expr, e, k.clone(), init_d.clone(), d.clone(), c);
            }
            r
        }

        Expression::TupleBuilder(tuple_expr) => {
            let mut result: Vec<Tuple> = vec![];
            let mut missing: Domain = Domain::new();
            for tuple in tuple_expr {
                let lhs = evaluate(tuple.lhs, e, k.clone(), init_d.clone(), d.clone(), c);
                let rhs = evaluate(tuple.rhs, e, k.clone(), init_d.clone(), d.clone(), c);
                match lhs {
                    Either::Left(l) => match rhs {
                        Either::Left(rl) => {
                            let dim = l.expect_dimension();
                            result.push(Tuple::new(dim, rl))
                        }

                        Either::Right(rr) => missing = missing.union(rr),
                    },
                    Either::Right(r) => match rhs {
                        Either::Left(_) => missing = missing.union(r),
                        Either::Right(rr) => missing = missing.union(r).union(rr),
                    },
                }
            }

            if missing.len() > 0 {
                Either::Right(missing)
            } else {
                Either::Left(Value::Context(Context(result)))
            }
        }

        Expression::Application(application_expr) => {
            let e0 = evaluate(
                application_expr[0].clone(),
                e,
                k.clone(),
                init_d.clone(),
                d.clone(),
                c,
            );
            let mut params: Vec<Value> = vec![];
            let mut missing: Domain = Domain::new();
            for i in 1..application_expr.len() {
                let param = evaluate(
                    application_expr[i].clone(),
                    e,
                    k.clone(),
                    init_d.clone(),
                    d.clone(),
                    c,
                );
                match param {
                    Either::Left(l) => params.push(l),
                    Either::Right(r) => {
                        missing = missing.union(r);
                    }
                }
            }

            if missing.len() > 0 {
                match e0 {
                    Either::Left(_) => Either::Right(missing),
                    Either::Right(r) => Either::Right(missing.union(r)),
                }
            } else {
                match e0 {
                    Either::Left(l) => {
                        match l {
                            Value::Identifier(op) => {
                                // lookup primitive operator & apply
                                match op.as_ref() {
                                    "*" => {
                                        let a = params[0].expect_integer();
                                        let b = params[1].expect_integer();
                                        Either::Left(Value::Literal(Literal::Int32(a * b)))
                                    }

                                    "+" => {
                                        let paramsi: Vec<
                                            u32,
                                        > = params.iter().map(|p| p.expect_integer()).collect();
                                        Either::Left(Value::Literal(Literal::Int32(
                                            paramsi.iter().fold(0, |sum, n| sum + n),
                                        )))
                                    }

                                    "-" => {
                                        let a = params[0].expect_integer();
                                        let b = params[1].expect_integer();
                                        Either::Left(Value::Literal(Literal::Int32(a - b)))
                                    }

                                    "<=" => {
                                        let a = params[0].expect_integer();
                                        let b = params[1].expect_integer();
                                        Either::Left(Value::Literal(Literal::Bool(a <= b)))
                                    }

                                    ">" => {
                                        let a = params[0].expect_integer();
                                        let b = params[1].expect_integer();
                                        Either::Left(Value::Literal(Literal::Bool(a > b)))
                                    }

                                    _ => panic!("Unrecognised primitive."),
                                }
                            }

                            other => panic!("Expected operator but here found {:?}", other),
                        }
                    }
                    Either::Right(r) => Either::Right(r),
                }
            }
        }

        Expression::If(if_expr) => {
            let condition = if_expr.condition.clone();
            let consequent = if_expr.consequent.clone();
            let alternate = if_expr.alternate.clone();
            let condition = evaluate(condition, e, k.clone(), init_d.clone(), d.clone(), c);
            match condition {
                Either::Left(l) => match l {
                    Value::Literal(Literal::Bool(true)) => {
                        evaluate(consequent, e, k.clone(), init_d, d.clone(), c)
                    }

                    Value::Literal(Literal::Bool(false)) => {
                        let x = evaluate(alternate, e, k.clone(), init_d, d.clone(), c);
                        x
                    }

                    _ => panic!("Expected boolean expression"),
                },

                Either::Right(r) => Either::Right(r),
            }
        }

        Expression::WhereVar(wv) => {
            e.merge(wv.rhs.clone());
            evaluate(wv.lhs.clone(), e, k.clone(), init_d, d.clone(), c)
        }

        Expression::Query(e0) => {
            let e0 = (*e0).clone();
            let v0 = evaluate(e0, e, k.clone(), init_d, d.clone(), c);
            match v0 {
                Either::Left(l) => {
                    let di = l.expect_dimension();
                    if d.contains(di.clone()) {
                        let v = k.lookup(di).expect("Expected dimension in context.");
                        Either::Left(v)
                    } else {
                        let mut h = HashSet::new();
                        h.insert(di);
                        Either::Right(Domain(h))
                    }
                }

                Either::Right(d0) => Either::Right(d0),
            }
        }

        Expression::Perturb(perturb_expr) => {
            let rhs = evaluate(
                perturb_expr.clone().rhs,
                e,
                k.clone(),
                init_d.clone(),
                d.clone(),
                c,
            );
            match rhs.clone() {
                Either::Left(l) => match l {
                    Value::Context(v1) => {
                        let v = evaluate(
                            perturb_expr.lhs,
                            e,
                            k.clone().perturb(v1.clone()),
                            init_d,
                            d.union(v1.domain()),
                            c,
                        );
                        v
                    }

                    _ => panic!("Invalid expression in rhs of perturbation."),
                },

                Either::Right(_) => rhs.clone(),
            }
        }

        Expression::BaseAbstraction(base_abstraction) => {
            Either::Left(Value::BaseAbstraction(base_abstraction.clone()))
        }

        Expression::BaseApplication(base_application) => {
            let base_abstraction = evaluate(
                base_application.lhs.clone(),
                e,
                k.clone(),
                init_d.clone(),
                d.clone(),
                c,
            );
            let mut args: Vec<Value> = vec![];
            let mut missing: Domain = Domain::new();
            for i in 0..base_application.args.len() {
                let arg = evaluate(
                    base_application.args[i].clone(),
                    e,
                    k.clone(),
                    init_d.clone(),
                    d.clone(),
                    c,
                );
                match arg {
                    Either::Left(l) => args.push(l),
                    Either::Right(r) => {
                        missing = missing.union(r);
                    }
                }
            }

            if missing.len() > 0 {
                match base_abstraction {
                    Either::Left(_) => Either::Right(missing),
                    Either::Right(r) => Either::Right(missing.union(r)),
                }
            } else {
                // Verify that base_application.lhs is a BaseAbstraction.
                match base_abstraction {
                    Either::Left(l) => match l {
                        Value::BaseAbstraction(base_abstraction) => {
                            // Check that the arity of BaseAbstraction matches the application.
                            if base_abstraction.dimensions.len() == args.len() {
                                let mut context = Context::new();
                                for i in 0..args.len() {
                                    context.push(
                                        base_abstraction.dimensions[i].clone(),
                                        args[i].clone(),
                                    );
                                }
                                // let domain = Domain::new();
                                // let domain = domain.from_vec(base_abstraction.dimensions);
                                // Apply the base application to the arguments.
                                evaluate(
                                    base_abstraction.body.clone(),
                                    e,
                                    k.clone(),
                                    init_d.clone(),
                                    d.clone(),
                                    c,
                                )
                            } else {
                                panic!("Invalid arity in base application.")
                            }
                        }

                        _ => panic!("Expected base abstraction but here found other."),
                    },

                    Either::Right(r) => Either::Right(r),
                }
            }
        }

        Expression::ValueAbstraction(value_abstraction) => {
            Either::Left(Value::ValueAbstraction(value_abstraction.clone()))
        }

        Expression::ValueApplication(value_application) => {
            let value_abstraction = evaluate(
                value_application.lhs.clone(),
                e,
                k.clone(),
                init_d.clone(),
                d.clone(),
                c,
            );
            let mut args: Vec<Value> = vec![];
            let mut missing: Domain = Domain::new();
            for i in 0..value_application.args.len() {
                let arg = evaluate(
                    value_application.args[i].clone(),
                    e,
                    k.clone(),
                    init_d.clone(),
                    d.clone(),
                    c,
                );
                match arg {
                    Either::Left(l) => args.push(l),
                    Either::Right(r) => {
                        missing = missing.union(r);
                    }
                }
            }

            if missing.len() > 0 {
                match value_abstraction {
                    Either::Left(_) => Either::Right(missing),
                    Either::Right(r) => Either::Right(missing.union(r)),
                }
            } else {
                // Verify that value_application.lhs is a ValueAbstraction.
                match value_abstraction {
                    Either::Left(l) => match l {
                        Value::ValueAbstraction(value_abstraction) => {
                            // Check that the arity of ValueAbstraction matches the application.
                            if value_abstraction.dimensions.len() == args.len() {
                                let mut context = Context::new();
                                for i in 0..args.len() {
                                    context.push(
                                        value_abstraction.dimensions[i].clone(),
                                        args[i].clone(),
                                    );
                                }
                                // let domain = Domain::new();
                                // let domain = domain.from_vec(base_abstraction.dimensions);
                                // Apply the value application to the arguments.
                                evaluate(
                                    value_abstraction.body.clone(),
                                    e,
                                    k.clone().perturb(context),
                                    init_d.clone(),
                                    d.clone(),
                                    c,
                                )
                            } else {
                                panic!("Invalid arity in value application.")
                            }
                        }

                        _ => panic!("Expected value abstraction but here found other."),
                    },

                    Either::Right(r) => Either::Right(r),
                }
            }
        }

        Expression::FunctionApplication(function_application) => {
            let abstraction = e.lookup(function_application.id.clone());

            // Evaluate the function in order to obtain a set of abstractions.
            let abstraction = evaluate(
                abstraction.clone(),
                e,
                k.clone(),
                init_d.clone(),
                d.clone(),
                c,
            );

            // Evaluate the function arguments.
            let mut base_args: Vec<Value> = vec![];
            let mut value_args: Vec<Value> = vec![];
            let mut missing: Domain = Domain::new();

            // Evaluate the base arguments.
            for i in 0..function_application.base_args.len() {
                let base_arg = evaluate(
                    function_application.base_args[i].clone(),
                    e,
                    k.clone(),
                    init_d.clone(),
                    d.clone(),
                    c,
                );
                match base_arg {
                    Either::Left(l) => base_args.push(l),
                    Either::Right(r) => missing = missing.union(r),
                }
            }

            // Evaluate the value arguments.
            for i in 0..function_application.value_args.len() {
                let value_arg = evaluate(
                    function_application.value_args[i].clone(),
                    e,
                    k.clone(),
                    init_d.clone(),
                    d.clone(),
                    c,
                );
                match value_arg {
                    Either::Left(l) => value_args.push(l),
                    Either::Right(r) => missing = missing.union(r),
                }
            }

            // TODO: Ignore the name arguments.

            if missing.len() > 0 {
                match abstraction {
                    Either::Left(_) => Either::Right(missing),
                    Either::Right(r) => Either::Right(missing.union(r)),
                }
            } else {
                match abstraction {
                    Either::Left(l) => match l {
                        Value::BaseAbstraction(base_abstraction) => {
                            // Apply base args to the abstraction then apply value args if there are any.
                            let mut context = Context::new();
                            for i in 0..base_args.len() {
                                context.push(
                                    base_abstraction.dimensions[i].clone(),
                                    base_args[i].clone(),
                                );
                            }
                            // Apply the application to the arguments.
                            let maybe_value_abstraction = evaluate(
                                base_abstraction.body.clone(),
                                e,
                                k.clone(),
                                init_d.clone(),
                                d.clone(),
                                c,
                            );

                            match maybe_value_abstraction {
                                Either::Left(l) => match l {
                                    Value::ValueAbstraction(value_abstraction) => {
                                        // Apply value args to the abstraction.
                                        let mut context = Context::new();
                                        let mut domain = Domain::new();
                                        for i in 0..value_args.len() {
                                            context.push(
                                                value_abstraction.dimensions[i].clone(),
                                                value_args[i].clone(),
                                            );
                                            domain.push(value_abstraction.dimensions[i].clone());
                                        }
                                        // Apply the application to the arguments.
                                        evaluate(
                                            value_abstraction.body.clone(),
                                            e,
                                            k.clone().perturb(context),
                                            init_d.clone(),
                                            d.clone().union(domain),
                                            c,
                                        )
                                    }

                                    other_value => Either::Left(other_value),
                                },

                                Either::Right(r) => Either::Right(r),
                            }
                        }

                        Value::ValueAbstraction(value_abstraction) => {
                            // Apply value args to the abstraction.
                            let mut context = Context::new();
                            let mut domain = Domain::new();
                            //println!("dimensions: {:?}", value_abstraction.dimensions.clone());
                            //println!("value_args: {:?}", value_args.clone());
                            for i in 0..value_args.len() {
                                context.push(
                                    value_abstraction.dimensions[i].clone(),
                                    value_args[i].clone(),
                                );
                                domain.push(value_abstraction.dimensions[i].clone());
                            }
                            //println!("context: {:?}", k.clone().perturb(context.clone()));
                            // Apply the application to the arguments.
                            evaluate(
                                value_abstraction.body.clone(),
                                e,
                                k.clone().perturb(context),
                                init_d.clone(),
                                d.clone().union(domain),
                                c,
                            )
                        }

                        other => panic!("Expected abstraction but here found {:?}", other),
                    },

                    Either::Right(r) => Either::Right(r),
                }
            }
        }

        Expression::IntensionBuilder(intens_expr) => {
            let mut dimensions = Domain::new();
            let mut missing = Domain::new();
            for expr in intens_expr.domain.clone() {
                let vi = evaluate(expr, e, k.clone(), init_d.clone(), d.clone(), c);
                match vi {
                    Either::Left(di) => {
                        dimensions.push(di.expect_dimension());
                    }

                    Either::Right(d) => missing = missing.union(d).clone(),
                }
            }
            // println!("evaluated dimensions = {:?}", dimensions.clone());
            // println!("K = {:?}", k.clone().restrict(dimensions.clone()));
            if missing.len() > 0 {
                Either::Right(missing)
            } else {
                let intens = Intension {
                    k: k.clone().restrict(dimensions.clone()),
                    d: d.clone().union(dimensions.clone()).to_vec(),
                    x: Box::new(intens_expr.value.clone()),
                };
                Either::Left(Value::Intension(Box::new(intens)))
            }
        }

        Expression::IntensionApplication(intens_app) => {
            let intens_app = (*intens_app).clone();
            let v0 = evaluate(intens_app, e, k.clone(), init_d.clone(), d.clone(), c);
            match v0 {
                Either::Left(v) => {
                    let intens = v.expect_intension();
                    let ik = intens.k.clone();
                    let idom = Domain::new();
                    let idom = idom.from_vec(intens.d.clone());
                    let x = intens.x.clone();
                    evaluate(
                        *x,
                        e,
                        k.clone().perturb(ik),
                        init_d,
                        d.clone().union(idom),
                        c,
                    )
                }

                Either::Right(_) => v0.clone(),
            }
        }

        Expression::Identifier(id) => {
            // println!("{} @ {}", id.clone(), k.clone().restrict(d.clone()).print());
            evaluate_id1(
                id.clone(),
                e,
                k.clone(),
                init_d.clone(),
                Domain::new(),
                d.clone(),
                c,
            )
        }

        Expression::WhereDim(wd) => {
            let rhs = wd.rhs.clone();
            let lhs = wd.lhs.clone();
            // evaluate rhs dimensions into a context
            let mut context = Context::new();
            let mut domain = Domain::new();
            let mut missing = Domain::new();
            for dimension_expr in rhs.0 {
                let vi = evaluate(
                    dimension_expr.rhs,
                    e,
                    k.clone(),
                    init_d.clone(),
                    d.clone(),
                    c,
                );
                match vi {
                    Either::Left(v) => {
                        let xi = dimension_expr.lhs;
                        let depth = k.lookup(wd.dim_q.clone()).unwrap().expect_integer();
                        let (di, div) = generate_dimension(xi.i, wd.nat_q, depth);
                        context.push(xi.clone(), div);
                        context.push(di.clone(), v);
                        domain.push(xi.clone());
                    }

                    Either::Right(dom) => missing = missing.union(dom),
                }
            }
            if missing.len() > 0 {
                println!("missing => {:?}", missing.clone());
                Either::Right(missing)
            } else {
                // println!("wheredim K pre => {}", k.clone());
                // println!("wheredim K post => {}", k.clone().perturb(context.clone()));
                evaluate(
                    lhs,
                    e,
                    k.clone().perturb(context),
                    init_d,
                    d.clone().union(domain),
                    c,
                )
            }
        }
    }
}
