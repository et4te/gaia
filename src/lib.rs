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

use std::collections::{HashMap,HashSet};
use expression::*;
use environment::*;
use cache::Cache;
use context::Context;
use value::{Value, Dimension, print_dimension};
use domain::Domain;
use either::Either;

type Identifier = String;

pub fn evaluate(expr: L1Expression) -> Either<Value, Domain> {
    let mut c = Cache::new();
    let mut e = Environment::new();
    e.define("/".to_string(), Expression::Operator("/".to_string()));
    e.define("*".to_string(), Expression::Operator("*".to_string()));
    e.define("+".to_string(), Expression::Operator("+".to_string()));
    e.define("-".to_string(), Expression::Operator("-".to_string()));
    e.define("<=".to_string(), Expression::Operator("<=".to_string()));
    e.define(">".to_string(), Expression::Operator(">".to_string()));
    let mut k = Context::new();
    let mut d = Domain::new();
    let mut dimensions = HashMap::new();
    let q_dimensions = HashSet::new();
    let (x, q_dims) = transform_l1_dimensions(expr, &mut dimensions, 0, q_dimensions);
    for q_dim in q_dims.clone() {
        k.push(q_dim.clone(), Value::Literal(Literal::Int32(0)));
        d.push(q_dim);
    }
    println!("K :: {}", k.clone().print());
    println!("D :: {}\n", d.clone().print());
    evaluator::evaluate(x, &mut e, k.clone(), d.clone(), d.clone(), &mut c)
}

pub fn transform_l1_dimensions(expr: L1Expression, dimensions: &mut HashMap<Identifier,Dimension>, q: u32, q_dimensions: HashSet<Dimension>) -> (Expression, HashSet<Dimension>) {
    match expr {
        L1Expression::Literal(literal) =>
            (Expression::Literal(literal), q_dimensions),

        L1Expression::Operator(id) =>
            (Expression::Operator(id), q_dimensions),

        L1Expression::Sequence(expr_vec) => {
            let mut r = vec![];
            let mut q_dimensions = q_dimensions;
            for expr in expr_vec {
                let (vi, q_dims) = transform_l1_dimensions(expr, dimensions, q, q_dimensions);
                q_dimensions = q_dims;
                r.push(vi);
            }
            (Expression::Sequence(r), q_dimensions)
        },

        L1Expression::TupleBuilder(tuple_expr) => {
            let mut r = vec![];
            let mut q_dimensions: HashSet<Dimension> = q_dimensions;
            for tuple in tuple_expr {
                let (lhs, q_dims_lhs) = transform_l1_dimensions(tuple.lhs, dimensions, q, q_dimensions.clone());
                let (rhs, q_dims_rhs) = transform_l1_dimensions(tuple.rhs, dimensions, q, q_dimensions.clone());
                q_dimensions = q_dims_lhs.union(&q_dims_rhs).cloned().collect();
                let tup = TupleExpression {
                    lhs: lhs,
                    rhs: rhs,
                };
                r.push(tup)
            }
            (Expression::TupleBuilder(r), q_dimensions)
        },

        L1Expression::Application(application_expr) => {
            let mut r: Vec<Expression> = vec![];
            let mut q_dimensions = q_dimensions;
            let op = application_expr[0].clone();
            let (op, q_dims1) = transform_l1_dimensions(op, dimensions, q, q_dimensions.clone());
            for i in 1..application_expr.len() {
                let (arg, q_dims2) = transform_l1_dimensions(application_expr[i].clone(), dimensions, q, q_dims1.clone());
                q_dimensions = q_dims2;
                r.push(arg);
            }
            (Expression::Application([vec![op], r].concat()), q_dimensions)
        },

        L1Expression::If(if_expr) => {
            let (l1_condition, q_dims1) = transform_l1_dimensions(if_expr.condition.clone(), dimensions, q, q_dimensions.clone());
            let (l1_consequent, q_dims2) = transform_l1_dimensions(if_expr.consequent.clone(), dimensions, q, q_dimensions.clone());
            let (l1_alternate, q_dims3) = transform_l1_dimensions(if_expr.alternate, dimensions, q, q_dimensions.clone());
            let l1_if = IfExpression {
                condition: l1_condition,
                consequent: l1_consequent,
                alternate: l1_alternate,
            };
            let q_dims1: HashSet<Dimension>
                = q_dims1.union(&q_dims2).cloned().collect();
            let q_dims2: HashSet<Dimension>
                = q_dims1.union(&q_dims3).cloned().collect();
            let q_dimensions = q_dimensions.union(&q_dims2).cloned().collect();
            (Expression::If(Box::new(l1_if)), q_dimensions)
        },

        L1Expression::WhereVar(wv) => {
            let mut e = Environment::new();
            let mut q_dimensions = q_dimensions;
            for (id, expr) in wv.rhs.0.clone() {
                let (expr, q_dims) = transform_l1_dimensions(expr, dimensions, q, q_dimensions);
                q_dimensions = q_dims;
                e.define(id, expr.clone());
            }
            let (lhs, q_dims) = transform_l1_dimensions(wv.lhs.clone(), dimensions, q, q_dimensions);
            let wv = WhereVarExpression {
                lhs: lhs,
                rhs: e.clone(),
            };
            (Expression::WhereVar(Box::new(wv)), q_dims)
        },

        L1Expression::Query(expr) => {
            let expr = Box::into_raw(expr);
            let expr = unsafe { (*expr).clone() };
            let (expr, q_dims) = transform_l1_dimensions(expr.clone(), dimensions, q, q_dimensions);
            (Expression::Query(Box::new(expr)), q_dims)
        },

        L1Expression::Perturb(perturb_expr) => {
            let lhs = perturb_expr.clone().lhs;
            let rhs = perturb_expr.rhs;
            let (lhs, q_dims1)
                = transform_l1_dimensions(lhs, dimensions, q, q_dimensions.clone());
            let (rhs, q_dims2)
                = transform_l1_dimensions(rhs, dimensions, q, q_dimensions);
            let q_dims = q_dims1.union(&q_dims2).cloned().collect();
            let perturb_expr = PerturbExpression {
                lhs: lhs,
                rhs: rhs,
            };
            (Expression::Perturb(Box::new(perturb_expr)), q_dims)
        },

        L1Expression::BaseAbstraction(base_abstraction) => {
            let di = Dimension {
                i: 0,
                v: Value::Identifier(base_abstraction.id.clone()),
            };
            let (expr, q_dims) = transform_l1_dimensions(base_abstraction.expression, dimensions, q, q_dimensions.clone());
            let base_abstr = BaseAbstraction {
                param: di.clone(),
                dimensions: vec![],
                expression: expr,
            };
            (Expression::BaseAbstraction(Box::new(base_abstr)), q_dims)
        },

        L1Expression::IntensionBuilder(intens_expr) => {
            let mut r = vec![];
            let mut q_dimensions = q_dimensions;
            let domain = intens_expr.domain.clone();
            for dim in domain {
                let (di, q_dims) = transform_l1_dimensions(dim, dimensions, q, q_dimensions.clone());
                q_dimensions = q_dims;
                r.push(di);
            }
            let (e0, q_dims) = transform_l1_dimensions(intens_expr.value.clone(), dimensions, q, q_dimensions.clone());
            let intens_expr = IntensionExpression {
                domain: r.clone(),
                value: e0,
            };
            (Expression::IntensionBuilder(Box::new(intens_expr)), q_dims)
        },

        L1Expression::IntensionApplication(intens_app) => {
            let (expr, q_dims) = transform_l1_dimensions(*intens_app, dimensions, q, q_dimensions.clone());
            (Expression::IntensionApplication(Box::new(expr)), q_dims)
        },

        L1Expression::Identifier(id) => {
            if dimensions.contains_key(&id) {
                // lookup unique hidden dimension at id
                let di = dimensions.get(&id).unwrap();
                (Expression::Dimension(di.clone()), q_dimensions)
            } else {
                // id is a free variable
                (Expression::Identifier(id), q_dimensions)
            }
        },

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
                let (expr, q_dims) = transform_l1_dimensions(expr, dimensions, q, q_dimensions);
                q_dimensions = q_dims;
                let dim_expr = DimensionExpression {
                    lhs: di,
                    rhs: expr,
                };
                dimension_exprs.push(dim_expr);
            }

            let (lhs, q_dims) = transform_l1_dimensions(lhs, dimensions, q + 1, q_dimensions);
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
        },

        _ =>
            panic!("Unrecognised expression"),
    }
}

pub fn print_expression(expr: Expression, indent: u32) -> String {
    match expr {
        Expression::Literal(lit) => {
            match lit {
                Literal::Bool(b) => {
                    let s = format!("{:?}", b).bright_cyan();
                    format!("{}", s)
                }
                Literal::Int32(i) => {
                    let s = format!("{:?}", i).bright_cyan();
                    format!("{}", s)
                }
            }
        },

        Expression::Dimension(di) => {
            print_dimension(di)
            //format!("{}", di.v.bright_magenta())
        },

        Expression::Operator(op) => {
            format!("{}", op.bright_white())
        },

        Expression::Sequence(exprs) => {
            let mut s = "".to_string();
            for expr in exprs {
                s = format!("{} {};", s, print_expression(expr, indent))
            }
            s
        },

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
        },

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
        },

        Expression::If(if_expr) => {
            let condition = if_expr.condition.clone();
            let consequent = if_expr.consequent.clone();
            let alternate = if_expr.alternate.clone();
            format!("\n{}{} {} {} {}{} \n{}{} {}{} \n{}{}",
                    print_spaces(indent+2),
                    "if".bright_cyan(),
                    print_expression(condition, indent),
                    "then\n".bright_cyan(),
                    print_spaces(indent+4),
                    print_expression(consequent, indent),
                    print_spaces(indent+2),
                    "else\n".bright_cyan(),
                    print_spaces(indent+4),
                    print_expression(alternate, indent),
                    print_spaces(indent+2),
                    "end".bright_cyan())
        },

        Expression::WhereVar(wv) => {
            let indent_s = print_spaces(indent);
            let rhs = wv.rhs.clone();
            let lhs = wv.lhs.clone();
            let lhs = print_expression(lhs, indent);
            let mut s = format!("{}{}\n{}{}\n",
                                indent_s, lhs,
                                indent_s, "wherevar".bright_white());
            for def in rhs.0 {
                s = format!("{}{}{} = {}\n",
                            s,
                            print_spaces(indent + 2),
                            def.id.bright_yellow(),
                            print_expression(def.equation, indent + 2));
            }
            s
        },

        Expression::Query(e0) => {
            let e0 = Box::into_raw(e0);
            let e0 = unsafe { (*e0).clone() };
            format!("#.{}", print_expression(e0, indent))
        },

        Expression::Perturb(perturb_expr) => {
            let lhs = print_expression(perturb_expr.clone().lhs, indent);
            let rhs = print_expression(perturb_expr.rhs, indent);
            format!("{} {} {}", lhs, "@".bright_white(), rhs)
        },

        Expression::BaseAbstraction(base_abstraction) => {
            let mut s = "(λ ".bright_white().to_string();
            s = format!("{}{} ->", s, print_dimension(base_abstraction.param.clone()));
            format!("{} {})", s, print_expression(base_abstraction.expression.clone(), indent))
        },

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
            format!("{} {}", s, print_expression(intens_expr.value.clone(), indent))
        },

        Expression::IntensionApplication(intens_app) =>
            format!("|> {}", print_expression((*intens_app).clone(), indent)),

        Expression::Identifier(id) => {
            format!("{}", id.bright_yellow())
        },

        Expression::WhereDim(wd) => {
            let indent_s = print_spaces(indent);
            let rhs = wd.rhs.clone();
            let lhs = wd.lhs.clone();
            let lhs = print_expression(lhs, indent);
            let nat_q_s = format!("{}", wd.nat_q.clone());
            let dim_q_s = print_dimension(wd.dim_q.clone());
            let mut s = format!("{}\n{}{} <{},{}>\n",
                                lhs,
                                indent_s,
                                "wheredim".bright_white(),
                                nat_q_s.bright_white(),
                                dim_q_s);
            for tup in rhs.0 {
                s = format!("{}{}{} {} {}\n",
                            s,
                            print_spaces(indent + 2),
                            print_dimension(tup.lhs),
                            "<-".bright_white(),
                            print_expression(tup.rhs, indent));
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
