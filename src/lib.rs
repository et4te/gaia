extern crate colored;

use colored::*;

pub mod evaluator;
pub mod expression;
pub mod environment;
pub mod cache;
pub mod context;
pub mod domain;
pub mod value;
pub mod tuple;
pub mod either;

use std::collections::{HashMap, HashSet};
use expression::*;
use environment::*;
use cache::Cache;
use context::Context;
use value::{print_dimension, Dimension, Value};
use domain::Domain;
use either::Either;

type Identifier = String;

pub fn evaluate(expr: L1Expression) -> Either<Value, Domain> {
    let mut c = Cache::new();
    let mut e = Environment::new();

    // Define VM opcodes

    // Context Idexing
    //   #.t
    e.define("#".to_string(), Expression::Operator("#".to_string()));
    // Context Navigation
    //   X @ [t <- 0]
    e.define("@".to_string(), Expression::Operator("@".to_string()));
    e.define("==".to_string(), Expression::Operator("==".to_string()));
    e.define("/=".to_string(), Expression::Operator("%".to_string()));
    e.define("%".to_string(), Expression::Operator("%".to_string()));
    e.define("^".to_string(), Expression::Operator("^".to_string()));
    e.define("/".to_string(), Expression::Operator("/".to_string()));
    e.define("*".to_string(), Expression::Operator("*".to_string()));
    e.define("+".to_string(), Expression::Operator("+".to_string()));
    e.define("-".to_string(), Expression::Operator("-".to_string()));
    e.define("<".to_string(), Expression::Operator(">=".to_string()));
    e.define("<=".to_string(), Expression::Operator("<=".to_string()));
    e.define(">".to_string(), Expression::Operator(">".to_string()));
    e.define(">=".to_string(), Expression::Operator(">=".to_string()));

    let mut k = Context::new();
    let mut d = Domain::new();
    let mut dimensions = HashMap::new();
    let mut names = HashSet::new();
    let q_dimensions = HashSet::new();
    let (x, q_dims) = transform_l1_dimensions(expr, &mut dimensions, &mut names, 0, q_dimensions);
    for q_dim in q_dims.clone() {
        k.push(q_dim.clone(), Value::Literal(Literal::Int32(0)));
        d.push(q_dim);
    }
    // println!("K :: {}", k.clone().print());
    // println!("D :: {}\n", d.clone().print());
    evaluator::evaluate(x, &mut e, k.clone(), d.clone(), d.clone(), &mut c)
}

pub fn generate_dimensional_map(
    parameters: Vec<L1Expression>,
) -> (Vec<Dimension>, HashMap<Identifier, Dimension>) {
    let mut dimensions = vec![];
    let mut dimensional_map = HashMap::new();

    let mut i: u32 = 0;
    for parameter in parameters {
        let id = parameter.expect_identifier();
        let di = Dimension {
            i: 0, // change this to p
            v: Value::Identifier(id.clone()),
        };
        i = i + 1;
        dimensions.push(di.clone());
        dimensional_map.insert(id.clone(), di);
    }

    (dimensions.clone(), dimensional_map.clone())
}

pub fn generate_name_map(parameters: Vec<L1Expression>) -> HashSet<Identifier> {
    let mut names = HashSet::new();
    for parameter in parameters {
        let id = parameter.expect_identifier();
        names.insert(id.clone());
    }
    names
}

pub fn maybe_name_abstraction_from_fun_declaration(
    fun_declaration: L1FunctionDeclaration,
) -> Option<L1NameAbstraction> {
    let name_parameters: Vec<L1Expression> = fun_declaration.name_parameters.clone();
    if name_parameters.len() > 0 {
        let name_abstraction = L1NameAbstraction {
            formal_parameters: name_parameters.clone(),
            body: fun_declaration.body.clone(),
        };
        Some(name_abstraction.clone())
    } else {
        None
    }
}

pub fn maybe_value_abstraction_from_fun_declaration(
    fun_declaration: L1FunctionDeclaration,
    name_abstraction: Option<L1NameAbstraction>,
) -> Option<L1ValueAbstraction> {
    let value_parameters: Vec<L1Expression> = fun_declaration.value_parameters.clone();
    if value_parameters.len() > 0 {
        match name_abstraction {
            Some(name_abstraction) => {
                let value_abstraction = L1ValueAbstraction {
                    formal_parameters: fun_declaration.value_parameters.clone(),
                    body: L1Expression::NameAbstraction(Box::new(name_abstraction.clone())),
                };
                Some(value_abstraction.clone())
            }

            None => {
                let value_abstraction = L1ValueAbstraction {
                    formal_parameters: fun_declaration.value_parameters.clone(),
                    body: fun_declaration.body.clone(),
                };
                Some(value_abstraction.clone())
            }
        }
    } else {
        None
    }
}

pub fn l1_abstraction_from_fun_declaration(
    fun_declaration: L1FunctionDeclaration,
    value_abstraction: Option<L1ValueAbstraction>,
    name_abstraction: Option<L1NameAbstraction>,
) -> L1Expression {
    let base_parameters = fun_declaration.base_parameters.clone();
    if base_parameters.len() > 0 {
        match value_abstraction {
            Some(value_abstraction) => match name_abstraction {
                Some(name_abstraction) => {
                    let base_abstraction = L1BaseAbstraction {
                        formal_parameters: fun_declaration.base_parameters.clone(),
                        body: L1Expression::ValueAbstraction(Box::new(value_abstraction.clone())),
                    };
                    L1Expression::BaseAbstraction(Box::new(base_abstraction))
                }

                None => {
                    let base_abstraction = L1BaseAbstraction {
                        formal_parameters: fun_declaration.base_parameters.clone(),
                        body: L1Expression::ValueAbstraction(Box::new(value_abstraction.clone())),
                    };
                    L1Expression::BaseAbstraction(Box::new(base_abstraction))
                }
            },
            None => match name_abstraction {
                Some(name_abstraction) => {
                    let base_abstraction = L1BaseAbstraction {
                        formal_parameters: fun_declaration.base_parameters.clone(),
                        body: L1Expression::NameAbstraction(Box::new(name_abstraction.clone())),
                    };
                    L1Expression::BaseAbstraction(Box::new(base_abstraction))
                }

                None => {
                    let base_abstraction = L1BaseAbstraction {
                        formal_parameters: fun_declaration.base_parameters.clone(),
                        body: fun_declaration.body.clone(),
                    };
                    L1Expression::BaseAbstraction(Box::new(base_abstraction))
                }
            },
        }
    } else {
        match value_abstraction {
            Some(value_abstraction) => {
                L1Expression::ValueAbstraction(Box::new(value_abstraction.clone()))
            }
            None => match name_abstraction {
                Some(name_abstraction) => {
                    L1Expression::NameAbstraction(Box::new(name_abstraction.clone()))
                }
                None => {
                    let base_abstraction = L1BaseAbstraction {
                        formal_parameters: fun_declaration.base_parameters.clone(),
                        body: fun_declaration.body.clone(),
                    };
                    L1Expression::BaseAbstraction(Box::new(base_abstraction.clone()))
                }
            },
        }
    }
}

fn merge(
    map1: HashMap<Identifier, Dimension>,
    map2: HashMap<Identifier, Dimension>,
) -> HashMap<Identifier, Dimension> {
    map1.into_iter().chain(map2).collect()
}

// Traverse the abstraction chain and generate dimensions for each parameter
// whilst transforming the L1Expressions into evaluatable Expressions. The
// generated dimensions are retained within the abstractions.
pub fn abstraction_from_l1(
    fun_declaration: L1FunctionDeclaration,
    abstraction: L1Expression,
    dimensions: &mut HashMap<Identifier, Dimension>,
    names: &mut HashSet<Identifier>,
    q: u32,
    q_dimensions: HashSet<Dimension>,
) -> (Expression, HashSet<Dimension>) {
    match abstraction {
        L1Expression::BaseAbstraction(base_abstraction) => {
            let (base_dimensions, base_dimensional_map) =
                generate_dimensional_map(base_abstraction.formal_parameters.clone());
            match base_abstraction.body {
                L1Expression::ValueAbstraction(value_abstraction) => {
                    let (value_dimensions, value_dimensional_map) =
                        generate_dimensional_map(value_abstraction.formal_parameters.clone());
                    match value_abstraction.body {
                        L1Expression::NameAbstraction(name_abstraction) => {
                            let name_map =
                                generate_name_map(name_abstraction.formal_parameters.clone());
                            let (name_dimensions, _) = generate_dimensional_map(
                                name_abstraction.formal_parameters.clone(),
                            );
                            // Merge all dimensional maps and transform the L1Expression body to Expression.
                            let dimensional_map =
                                merge(base_dimensional_map, value_dimensional_map);
                            let mut dimensional_map = merge(dimensional_map, dimensions.clone());
                            // Add the name_dimensions to the name_map in order to substitute names for intension applications.
                            let mut names: HashSet<Identifier> =
                                names.union(&name_map).cloned().collect();
                            let (body, q_dimensions) = transform_l1_dimensions(
                                name_abstraction.body,
                                &mut dimensional_map,
                                &mut names,
                                q,
                                q_dimensions.clone(),
                            );
                            let base_abstraction = BaseAbstraction {
                                dimensions: base_dimensions,
                                body: Expression::ValueAbstraction(Box::new(ValueAbstraction {
                                    dimensions: [value_dimensions, name_dimensions].concat(),
                                    body: body,
                                })),
                            };
                            (
                                Expression::BaseAbstraction(Box::new(base_abstraction)),
                                q_dimensions,
                            )
                        }
                        _ => {
                            let dimensional_map =
                                merge(base_dimensional_map, value_dimensional_map);
                            let mut dimensional_map = merge(dimensional_map, dimensions.clone());
                            let mut names = names;
                            let (body, q_dimensions) = transform_l1_dimensions(
                                value_abstraction.body,
                                &mut dimensional_map,
                                &mut names,
                                q,
                                q_dimensions.clone(),
                            );
                            let base_abstraction = BaseAbstraction {
                                dimensions: base_dimensions,
                                body: Expression::ValueAbstraction(Box::new(ValueAbstraction {
                                    dimensions: value_dimensions,
                                    body: body,
                                })),
                            };
                            (
                                Expression::BaseAbstraction(Box::new(base_abstraction)),
                                q_dimensions,
                            )
                        }
                    }
                }
                L1Expression::NameAbstraction(name_abstraction) => {
                    // Merge together the dimensions.
                    let name_map = generate_name_map(name_abstraction.formal_parameters.clone());
                    let (name_dimensions, _) =
                        generate_dimensional_map(name_abstraction.formal_parameters.clone());
                    let mut dimensional_map = merge(base_dimensional_map, dimensions.clone());
                    let mut names: HashSet<Identifier> = names.union(&name_map).cloned().collect();
                    let (body, q_dimensions) = transform_l1_dimensions(
                        name_abstraction.body,
                        &mut dimensional_map,
                        &mut names,
                        q,
                        q_dimensions.clone(),
                    );
                    let base_abstraction = BaseAbstraction {
                        dimensions: base_dimensions,
                        body: Expression::ValueAbstraction(Box::new(ValueAbstraction {
                            dimensions: name_dimensions,
                            body: body,
                        })),
                    };
                    (
                        Expression::BaseAbstraction(Box::new(base_abstraction)),
                        q_dimensions,
                    )
                }
                _ => {
                    let mut dimensional_map = merge(base_dimensional_map, dimensions.clone());
                    let mut names = names;
                    let (body, q_dimensions) = transform_l1_dimensions(
                        base_abstraction.body,
                        &mut dimensional_map,
                        &mut names,
                        q,
                        q_dimensions.clone(),
                    );
                    let base_abstraction = BaseAbstraction {
                        dimensions: base_dimensions,
                        body: body,
                    };
                    (
                        Expression::BaseAbstraction(Box::new(base_abstraction)),
                        q_dimensions,
                    )
                }
            }
        }
        L1Expression::ValueAbstraction(value_abstraction) => {
            let (value_dimensions, value_dimensional_map) =
                generate_dimensional_map(value_abstraction.formal_parameters.clone());
            match value_abstraction.body {
                L1Expression::NameAbstraction(name_abstraction) => {
                    let name_map = generate_name_map(name_abstraction.formal_parameters.clone());
                    let (name_dimensions, _) =
                        generate_dimensional_map(name_abstraction.formal_parameters.clone());
                    // Merge all dimensional maps and transform the L1Expression body to Expression.
                    let mut dimensional_map = merge(value_dimensional_map, dimensions.clone());
                    let mut names: HashSet<Identifier> = names.union(&name_map).cloned().collect();
                    let (body, q_dimensions) = transform_l1_dimensions(
                        name_abstraction.body,
                        &mut dimensional_map,
                        &mut names,
                        q,
                        q_dimensions.clone(),
                    );
                    let value_abstraction = ValueAbstraction {
                        dimensions: [value_dimensions, name_dimensions].concat(),
                        body: body,
                    };
                    (
                        Expression::ValueAbstraction(Box::new(value_abstraction)),
                        q_dimensions,
                    )
                }
                _ => {
                    let mut dimensional_map = merge(value_dimensional_map, dimensions.clone());
                    let mut names = names;
                    let (body, q_dimensions) = transform_l1_dimensions(
                        value_abstraction.body,
                        &mut dimensional_map,
                        &mut names,
                        q,
                        q_dimensions.clone(),
                    );
                    let value_abstraction = ValueAbstraction {
                        dimensions: value_dimensions,
                        body: body,
                    };
                    (
                        Expression::ValueAbstraction(Box::new(value_abstraction)),
                        q_dimensions,
                    )
                }
            }
        }
        L1Expression::NameAbstraction(name_abstraction) => {
            let name_map = generate_name_map(name_abstraction.formal_parameters.clone());
            let mut names: HashSet<Identifier> = names.union(&name_map).cloned().collect();
            let (name_dimensions, _) =
                generate_dimensional_map(name_abstraction.formal_parameters.clone());
            let mut dimensions = dimensions;
            let (body, q_dimensions) = transform_l1_dimensions(
                name_abstraction.body,
                &mut dimensions,
                &mut names,
                q,
                q_dimensions.clone(),
            );
            let value_abstraction = ValueAbstraction {
                dimensions: name_dimensions,
                body: body,
            };
            (
                Expression::ValueAbstraction(Box::new(value_abstraction)),
                q_dimensions,
            )
        }
        _ => panic!("Expected abstraction but here found a different expression type."),
    }
}

pub fn transform_l1_dimensions(
    expr: L1Expression,
    dimensions: &mut HashMap<Identifier, Dimension>,
    names: &mut HashSet<Identifier>,
    q: u32,
    q_dimensions: HashSet<Dimension>,
) -> (Expression, HashSet<Dimension>) {
    match expr {
        L1Expression::Literal(literal) => (Expression::Literal(literal), q_dimensions),

        L1Expression::Operator(id) => (Expression::Operator(id), q_dimensions),

        L1Expression::Sequence(expr_vec) => {
            let mut r = vec![];
            let mut q_dimensions = q_dimensions;
            for expr in expr_vec {
                let (vi, q_dims) =
                    transform_l1_dimensions(expr, dimensions, names, q, q_dimensions);
                q_dimensions = q_dims;
                r.push(vi);
            }
            (Expression::Sequence(r), q_dimensions)
        }

        L1Expression::TupleBuilder(tuple_expr) => {
            let mut r = vec![];
            let mut q_dimensions: HashSet<Dimension> = q_dimensions;
            for tuple in tuple_expr {
                let (lhs, q_dims_lhs) =
                    transform_l1_dimensions(tuple.lhs, dimensions, names, q, q_dimensions.clone());
                let (rhs, q_dims_rhs) =
                    transform_l1_dimensions(tuple.rhs, dimensions, names, q, q_dimensions.clone());
                q_dimensions = q_dims_lhs.union(&q_dims_rhs).cloned().collect();
                let tup = TupleExpression { lhs: lhs, rhs: rhs };
                r.push(tup)
            }
            (Expression::TupleBuilder(r), q_dimensions)
        }

        L1Expression::Application(application_expr) => {
            let mut r: Vec<Expression> = vec![];
            let mut q_dimensions = q_dimensions;
            let op = application_expr[0].clone();
            let (op, q_dims1) =
                transform_l1_dimensions(op, dimensions, names, q, q_dimensions.clone());
            for i in 1..application_expr.len() {
                let (arg, q_dims2) = transform_l1_dimensions(
                    application_expr[i].clone(),
                    dimensions,
                    names,
                    q,
                    q_dims1.clone(),
                );
                q_dimensions = q_dims2;
                r.push(arg);
            }
            (
                Expression::Application([vec![op], r].concat()),
                q_dimensions,
            )
        }

        L1Expression::If(if_expr) => {
            let (l1_condition, q_dims1) = transform_l1_dimensions(
                if_expr.condition.clone(),
                dimensions,
                names,
                q,
                q_dimensions.clone(),
            );
            let (l1_consequent, q_dims2) = transform_l1_dimensions(
                if_expr.consequent.clone(),
                dimensions,
                names,
                q,
                q_dimensions.clone(),
            );
            let (l1_alternate, q_dims3) = transform_l1_dimensions(
                if_expr.alternate,
                dimensions,
                names,
                q,
                q_dimensions.clone(),
            );
            let l1_if = IfExpression {
                condition: l1_condition,
                consequent: l1_consequent,
                alternate: l1_alternate,
            };
            let q_dims1: HashSet<Dimension> = q_dims1.union(&q_dims2).cloned().collect();
            let q_dims2: HashSet<Dimension> = q_dims1.union(&q_dims3).cloned().collect();
            let q_dimensions = q_dimensions.union(&q_dims2).cloned().collect();
            (Expression::If(Box::new(l1_if)), q_dimensions)
        }

        L1Expression::WhereVar(wv) => {
            let mut e = Environment::new();
            let mut q_dimensions = q_dimensions;
            for (id, expr) in wv.rhs.0.clone() {
                let (expr, q_dims) =
                    transform_l1_dimensions(expr, dimensions, names, q, q_dimensions);
                q_dimensions = q_dims;
                e.define(id, expr.clone());
            }
            let (lhs, q_dims) =
                transform_l1_dimensions(wv.lhs.clone(), dimensions, names, q, q_dimensions);
            let wv = WhereVarExpression {
                lhs: lhs,
                rhs: e.clone(),
            };
            (Expression::WhereVar(Box::new(wv)), q_dims)
        }

        L1Expression::Query(expr) => {
            let expr = *expr.clone();
            // let expr = Box::into_raw(expr);
            // let expr = unsafe { (*expr).clone() };
            let (expr, q_dims) =
                transform_l1_dimensions(expr.clone(), dimensions, names, q, q_dimensions);
            (Expression::Query(Box::new(expr)), q_dims)
        }

        L1Expression::Perturb(perturb_expr) => {
            let lhs = perturb_expr.clone().lhs;
            let rhs = perturb_expr.rhs;
            let (lhs, q_dims1) =
                transform_l1_dimensions(lhs, dimensions, names, q, q_dimensions.clone());
            let (rhs, q_dims2) = transform_l1_dimensions(rhs, dimensions, names, q, q_dimensions);
            let q_dims = q_dims1.union(&q_dims2).cloned().collect();
            let perturb_expr = PerturbExpression { lhs: lhs, rhs: rhs };
            (Expression::Perturb(Box::new(perturb_expr)), q_dims)
        }

        L1Expression::FunctionApplication(function_application_l1) => {
            // println!("lhs = {:?}", function_application_l1.lhs.clone());
            // Transform the lhs
            let (expr, _) = transform_l1_dimensions(
                function_application_l1.lhs.clone(),
                dimensions,
                names,
                q,
                q_dimensions.clone(),
            );
            // println!("expr = {:?}", expr.clone());
            let mut base_args = vec![];
            for i in 0..function_application_l1.base_args.len() {
                let (base_arg, _) = transform_l1_dimensions(
                    function_application_l1.base_args[i].clone(),
                    dimensions,
                    names,
                    q,
                    q_dimensions.clone(),
                );
                base_args.push(base_arg);
            }
            let mut value_args = vec![];
            for i in 0..function_application_l1.value_args.len() {
                let (value_arg, _) = transform_l1_dimensions(
                    function_application_l1.value_args[i].clone(),
                    dimensions,
                    names,
                    q,
                    q_dimensions.clone(),
                );
                value_args.push(value_arg);
            }
            for i in 0..function_application_l1.name_args.len() {
                let (value_arg, _) = transform_l1_dimensions(
                    function_application_l1.name_args[i].clone(),
                    dimensions,
                    names,
                    q,
                    q_dimensions.clone(),
                );
                let intension = IntensionExpression {
                    domain: vec![],
                    value: value_arg.clone(),
                };
                let arg = Expression::IntensionBuilder(Box::new(intension));
                value_args.push(arg)
            }
            let function_application = FunctionApplication {
                id: expr.as_identifier(),
                base_args: base_args,
                value_args: value_args,
            };
            (
                Expression::FunctionApplication(Box::new(function_application)),
                q_dimensions,
            )
        }

        L1Expression::BaseAbstraction(base_abstraction_l1) => {
            // Generate dimensions from parameters
            let mut i = 0;
            let mut dims = vec![];
            for param in base_abstraction_l1.formal_parameters.clone() {
                let id = param.clone().expect_identifier();
                let di = Dimension {
                    i: i.clone(),
                    v: Value::Identifier(id.clone()),
                };
                dimensions.insert(id.clone(), di.clone());
                dims.push(di);
                i += 1;
            }
            let (expr, q_dims) = transform_l1_dimensions(
                base_abstraction_l1.body,
                dimensions,
                names,
                q,
                q_dimensions.clone(),
            );
            let base_abstr = BaseAbstraction {
                dimensions: dims.clone(),
                body: expr,
            };
            (Expression::BaseAbstraction(Box::new(base_abstr)), q_dims)
        }

        L1Expression::BaseApplication(base_application_l1) => {
            let (base_abstraction, _) = transform_l1_dimensions(
                base_application_l1.lhs.clone(),
                dimensions,
                names,
                q,
                q_dimensions.clone(),
            );
            let mut base_application = BaseApplication {
                lhs: base_abstraction,
                args: vec![],
            };

            loop {
                match base_application_l1.rhs.clone() {
                    L1Expression::BaseApplication(base_application_l1) => {
                        let (arg, _) = transform_l1_dimensions(
                            base_application_l1.rhs,
                            dimensions,
                            names,
                            q,
                            q_dimensions.clone(),
                        );
                        base_application.args.push(arg);
                        continue;
                    }

                    other => {
                        let (arg, _) = transform_l1_dimensions(
                            other,
                            dimensions,
                            names,
                            q,
                            q_dimensions.clone(),
                        );
                        base_application.args.push(arg);
                        break;
                    }
                }
            }

            (
                Expression::BaseApplication(Box::new(base_application)),
                q_dimensions,
            )
        }

        L1Expression::ValueAbstraction(value_abstraction_l1) => {
            // Generate dimensions from parameters
            let mut i = 0;
            let mut dims = vec![];
            for param in value_abstraction_l1.formal_parameters.clone() {
                let id = param.clone().expect_identifier();
                let di = Dimension {
                    i: 0, // i.clone(),
                    v: Value::Identifier(id.clone()),
                };
                dimensions.insert(id.clone(), di.clone());
                dims.push(di);
                i += 1;
            }
            let (expr, q_dims) = transform_l1_dimensions(
                value_abstraction_l1.body,
                dimensions,
                names,
                q,
                q_dimensions.clone(),
            );
            let value_abstr = ValueAbstraction {
                dimensions: dims.clone(),
                body: expr,
            };
            (Expression::ValueAbstraction(Box::new(value_abstr)), q_dims)
        }

        L1Expression::ValueApplication(value_application_l1) => {
            let (value_abstraction, _) = transform_l1_dimensions(
                value_application_l1.lhs.clone(),
                dimensions,
                names,
                q,
                q_dimensions.clone(),
            );
            let mut value_application = ValueApplication {
                lhs: value_abstraction,
                args: vec![],
            };

            loop {
                match value_application_l1.rhs.clone() {
                    L1Expression::ValueApplication(value_application_l1) => {
                        let (arg, _) = transform_l1_dimensions(
                            value_application_l1.rhs,
                            dimensions,
                            names,
                            q,
                            q_dimensions.clone(),
                        );
                        value_application.args.push(arg);
                        continue;
                    }

                    other => {
                        let (arg, _) = transform_l1_dimensions(
                            other,
                            dimensions,
                            names,
                            q,
                            q_dimensions.clone(),
                        );
                        value_application.args.push(arg);
                        break;
                    }
                }
            }

            (
                Expression::ValueApplication(Box::new(value_application)),
                q_dimensions,
            )
        }

        L1Expression::NameAbstraction(name_abstraction_l1) => {
            // Generate dimensions from parameters
            let mut i = 0;
            let mut dims = vec![];
            for param in name_abstraction_l1.formal_parameters.clone() {
                let id = param.clone().expect_identifier();
                let di = Dimension {
                    i: 0, // i.clone(),
                    v: Value::Identifier(id.clone()),
                };
                names.insert(id.clone());
                dims.push(di);
                i += 1;
            }
            let (expr, q_dims) = transform_l1_dimensions(
                name_abstraction_l1.body,
                dimensions,
                names,
                q,
                q_dimensions.clone(),
            );
            let value_abstr = ValueAbstraction {
                dimensions: dims.clone(),
                body: expr,
            };
            (Expression::ValueAbstraction(Box::new(value_abstr)), q_dims)
        }

        L1Expression::NameApplication(name_application_l1) => {
            let (value_abstraction, _) = transform_l1_dimensions(
                name_application_l1.lhs.clone(),
                dimensions,
                names,
                q,
                q_dimensions.clone(),
            );
            let mut value_application = ValueApplication {
                lhs: value_abstraction,
                args: vec![],
            };

            loop {
                match name_application_l1.rhs.clone() {
                    L1Expression::NameApplication(name_application_l1) => {
                        let (arg, _) = transform_l1_dimensions(
                            name_application_l1.rhs,
                            dimensions,
                            names,
                            q,
                            q_dimensions.clone(),
                        );
                        let intension = IntensionExpression {
                            domain: vec![],
                            value: arg.clone(),
                        };
                        let expression = Expression::IntensionBuilder(Box::new(intension));
                        value_application.args.push(expression);
                        continue;
                    }

                    other => {
                        let (arg, _) = transform_l1_dimensions(
                            other,
                            dimensions,
                            names,
                            q,
                            q_dimensions.clone(),
                        );
                        value_application.args.push(arg);
                        break;
                    }
                }
            }

            (
                Expression::ValueApplication(Box::new(value_application)),
                q_dimensions,
            )
        }

        L1Expression::IntensionBuilder(intens_expr) => {
            let mut r = vec![];
            let mut q_domain = q_dimensions;
            let domain = intens_expr.domain.clone();
            // println!(
            //    "dimensions = {:?}, q_dimensions = {:?}",
            //    dimensions.clone(),
            //    q_domain.clone()
            // );
            for dim in domain {
                let (di, q_dims) =
                    transform_l1_dimensions(dim, dimensions, names, q, q_domain.clone());
                q_domain = q_dims;
                r.push(di);
            }
            // println!(
            //     "dimensions = {:?}, q_dimensions = {:?}",
            //     dimensions.clone(),
            //     q_domain.clone()
            // );
            let (e0, q_dims) = transform_l1_dimensions(
                intens_expr.value.clone(),
                dimensions,
                names,
                q,
                q_domain.clone(),
            );
            let intens_expr = IntensionExpression {
                domain: r.clone(),
                value: e0,
            };
            (Expression::IntensionBuilder(Box::new(intens_expr)), q_dims)
        }

        L1Expression::IntensionApplication(intens_app) => {
            let (expr, q_dims) =
                transform_l1_dimensions(*intens_app, dimensions, names, q, q_dimensions.clone());
            (Expression::IntensionApplication(Box::new(expr)), q_dims)
        }

        L1Expression::Identifier(id) => {
            if dimensions.contains_key(&id) {
                // lookup unique hidden dimension at id
                let di = dimensions.get(&id).unwrap();
                (Expression::Dimension(di.clone()), q_dimensions)
            } else {
                if names.contains(&id) {
                    let di = Dimension {
                        i: 0,
                        v: Value::Identifier(id.clone()),
                    };
                    (
                        Expression::IntensionApplication(Box::new(Expression::Dimension(di.clone()))),
                        q_dimensions,
                    )
                } else {
                    // id is a free variable
                    (Expression::Identifier(id), q_dimensions)
                }
            }
        }

        L1Expression::WhereDim(decl_expr) => {
            let rhs = decl_expr.rhs.clone();
            let lhs = decl_expr.lhs;
            let mut dimension_exprs = vec![];

            let mut q_dimensions = q_dimensions;
            let mut i = 0;
            for dimension_expr in rhs.clone().0 {
                // Insert dimension identifier into list of known dimensions
                let id = dimension_expr.lhs.clone();
                let di = Dimension {
                    i: i,
                    v: Value::Identifier(id.clone()),
                };
                dimensions.insert(id.clone(), di.clone());
                i += 1;

                // Transform expressions according to updated dimensions
                let expr = dimension_expr.rhs.clone();
                let (expr, q_dims) =
                    transform_l1_dimensions(expr, dimensions, names, q, q_dimensions);
                q_dimensions = q_dims;
                let dim_expr = DimensionExpression { lhs: di, rhs: expr };
                dimension_exprs.push(dim_expr);
            }

            let (lhs, q_dims) =
                transform_l1_dimensions(lhs, dimensions, names, q + 1, q_dimensions);
            let mut q_dimensions = q_dims;
            let dim_q = Dimension {
                i: q,
                v: Value::Identifier("φ".to_string()),
            };
            q_dimensions.insert(dim_q.clone());

            let wd = WhereDimExpression {
                nat_q: q,
                dim_q: dim_q,
                lhs: lhs,
                rhs: ContextExpression(dimension_exprs),
            };

            (Expression::WhereDim(Box::new(wd)), q_dimensions)
        }

        // Transform a variable declaration
        // L1Expression::VariableDeclaration(var_declaration) => {
        //     let wherevar_expression = WhereVarExpression {};
        //     (
        //         Expression::WhereVar(Box::new(wherevar_expression)),
        //         q_dimensions,
        //     )
        // }

        // Transform the function declaration into a base abstraction,
        // perhaps containing value and name abstractions.
        L1Expression::FunctionDeclaration(fun_declaration) => {
            let name_abstraction =
                maybe_name_abstraction_from_fun_declaration(*fun_declaration.clone());
            let value_abstraction = maybe_value_abstraction_from_fun_declaration(
                *fun_declaration.clone(),
                name_abstraction.clone(),
            );
            let l1_abstraction = l1_abstraction_from_fun_declaration(
                *fun_declaration.clone(),
                value_abstraction.clone(),
                name_abstraction.clone(),
            );

            abstraction_from_l1(
                *fun_declaration,
                l1_abstraction.clone(),
                dimensions,
                names,
                q,
                q_dimensions,
            )
        }

        other => panic!("Unrecognised expression {:?}", other),
    }
}

pub fn print_expression(expr: Expression, indent: u32) -> String {
    match expr {
        Expression::Literal(lit) => match lit {
            Literal::Bool(b) => {
                let s = format!("{:?}", b).bright_cyan();
                format!("{}", s)
            }
            Literal::Int32(i) => {
                let s = format!("{:?}", i).bright_cyan();
                format!("{}", s)
            }
        },

        Expression::Dimension(di) => {
            print_dimension(di)
            //format!("{}", di.v.bright_magenta())
        }

        Expression::Operator(op) => format!("{}", op.bright_white()),

        Expression::Sequence(exprs) => {
            let mut s = "".to_string();
            for expr in exprs {
                s = format!("{} {};", s, print_expression(expr, indent))
            }
            s
        }

        Expression::TupleBuilder(tuple_expr) => {
            let mut s = "[".bright_white().to_string();
            for i in 0..tuple_expr.len() {
                if i == (tuple_expr.len() - 1) {
                    let lhs = print_expression(tuple_expr[i].clone().lhs, indent);
                    let rhs = print_expression(tuple_expr[i].clone().rhs, indent);
                    s = format!("{}{} {} {}", s, lhs, "<-".bright_white(), rhs);
                } else {
                    let lhs = print_expression(tuple_expr[i].clone().lhs, indent);
                    let rhs = print_expression(tuple_expr[i].clone().rhs, indent);
                    s = format!("{}{} <- {}, ", s, lhs, rhs);
                }
            }
            format!("{}{}", s, "]".bright_white())
        }

        Expression::Application(app_expr) => {
            let op = print_expression(app_expr[0].clone(), indent);
            let mut args = "".to_string();
            for i in 1..app_expr.len() {
                if i == 1 {
                    args = format!("{}", print_expression(app_expr[i].clone(), indent));
                } else {
                    args = format!("{} {}", args, print_expression(app_expr[i].clone(), indent));
                }
            }
            format!("{}({})", op.yellow(), args)
        }

        Expression::If(if_expr) => {
            let condition = if_expr.condition.clone();
            let consequent = if_expr.consequent.clone();
            let alternate = if_expr.alternate.clone();
            format!(
                "\n{}{} {} {} {}{} \n{}{} {}{} \n{}{}",
                print_spaces(indent + 2),
                "if".bright_cyan(),
                print_expression(condition, indent),
                "then\n".bright_cyan(),
                print_spaces(indent + 4),
                print_expression(consequent, indent),
                print_spaces(indent + 2),
                "else\n".bright_cyan(),
                print_spaces(indent + 4),
                print_expression(alternate, indent),
                print_spaces(indent + 2),
                "end".bright_cyan()
            )
        }

        Expression::WhereVar(wv) => {
            let indent_s = print_spaces(indent);
            let rhs = wv.rhs.clone();
            let lhs = wv.lhs.clone();
            let lhs = print_expression(lhs, indent);
            let mut s = format!(
                "{}{}\n{}{}\n",
                indent_s,
                lhs,
                indent_s,
                "wherevar".bright_white()
            );
            for def in rhs.0 {
                s = format!(
                    "{}{}{} = {}\n",
                    s,
                    print_spaces(indent + 2),
                    def.id.bright_yellow(),
                    print_expression(def.equation, indent + 2)
                );
            }
            s
        }

        Expression::Query(e0) => {
            let e0 = Box::into_raw(e0);
            let e0 = unsafe { (*e0).clone() };
            format!("#.{}", print_expression(e0, indent))
        }

        Expression::Perturb(perturb_expr) => {
            let lhs = print_expression(perturb_expr.clone().lhs, indent);
            let rhs = print_expression(perturb_expr.rhs, indent);
            format!("{} {} {}", lhs, "@".bright_white(), rhs)
        }

        Expression::FunctionApplication(function_application) => format!("@function_application@"),

        Expression::BaseAbstraction(base_abstraction) => {
            let mut s = "(λb ".bright_white().to_string();
            for param in base_abstraction.dimensions.clone() {
                s = format!("{}{} ->", s, print_dimension(param));
            }
            format!(
                "{} {})",
                s,
                print_expression(base_abstraction.body.clone(), indent)
            )
        }

        Expression::BaseApplication(base_application) => {
            let op = print_expression(base_application.lhs.clone(), indent);
            let mut args = "".to_string();
            for i in 0..base_application.args.len() {
                if i == 0 {
                    args = format!(
                        "{}",
                        print_expression(base_application.args[i].clone(), indent)
                    );
                } else {
                    args = format!(
                        "{} {}",
                        args,
                        print_expression(base_application.args[i].clone(), indent)
                    );
                }
            }
            format!("{}({})", op.yellow(), args)
        }

        Expression::ValueAbstraction(value_abstraction) => {
            let mut s = "(λv ".bright_white().to_string();
            for param in value_abstraction.dimensions.clone() {
                s = format!("{}{} ->", s, print_dimension(param));
            }
            format!(
                "{} {})",
                s,
                print_expression(value_abstraction.body.clone(), indent)
            )
        }

        Expression::ValueApplication(value_application) => {
            let op = print_expression(value_application.lhs.clone(), indent);
            let mut args = "".to_string();
            for i in 0..value_application.args.len() {
                if i == 0 {
                    args = format!(
                        "{}",
                        print_expression(value_application.args[i].clone(), indent)
                    );
                } else {
                    args = format!(
                        "{} {}",
                        args,
                        print_expression(value_application.args[i].clone(), indent)
                    );
                }
            }
            format!("{}({})", op.yellow(), args)
        }

        Expression::IntensionBuilder(intens_expr) => {
            let mut s = "{".bright_white().to_string();
            if intens_expr.domain.len() > 0 {
                for i in 0..intens_expr.domain.len() {
                    if i == (intens_expr.domain.len() - 1) {
                        let ei = print_expression(intens_expr.domain[i].clone(), indent);
                        s = format!("{}{}", s, ei);
                    } else {
                        let ei = print_expression(intens_expr.domain[i].clone(), indent);
                        s = format!("{}{}", s, ei);
                    }
                }
            } else {
                return print_expression(intens_expr.value.clone(), indent);
            }
            s = format!("{}{}", s, "}".bright_white());
            format!(
                "{} {}",
                s,
                print_expression(intens_expr.value.clone(), indent)
            )
        }

        Expression::IntensionApplication(intens_app) => {
            format!("=> {}", print_expression((*intens_app).clone(), indent))
        }

        Expression::Identifier(id) => format!("{}", id.bright_yellow()),

        Expression::WhereDim(wd) => {
            let indent_s = print_spaces(indent);
            let rhs = wd.rhs.clone();
            let lhs = wd.lhs.clone();
            let lhs = print_expression(lhs, indent);
            let nat_q_s = format!("{}", wd.nat_q.clone());
            let dim_q_s = print_dimension(wd.dim_q.clone());
            let mut s = format!(
                "{}\n{}{} <{},{}>\n",
                lhs,
                indent_s,
                "wheredim".bright_white(),
                nat_q_s.bright_white(),
                dim_q_s
            );
            for tup in rhs.0 {
                s = format!(
                    "{}{}{} {} {}\n",
                    s,
                    print_spaces(indent + 2),
                    print_dimension(tup.lhs),
                    "<-".bright_white(),
                    print_expression(tup.rhs, indent)
                );
            }
            s = format!("{}{}{}", s, indent_s, "end".bright_white());
            s
        }
    }
}

pub fn print_spaces(lim: u32) -> String {
    let mut s = "".to_string();
    for _ in 0..lim {
        s = format!("{} ", s);
    }
    s
}
