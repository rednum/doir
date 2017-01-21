use ::problems::tsp::{dist, Graph};
use getopts::Options;
use std::io::Write;
use rand::distributions::{IndependentSample, Range};
use std;
use rand::{Rng, XorShiftRng};
use rand;
use std::mem;
use std::f64;

fn distinct_pair(rng: &mut XorShiftRng, range: Range<usize>) -> (usize, usize) {
    // TODO assert range non empty
    let x = range.ind_sample(rng);
    loop {
        let y = range.ind_sample(rng);
        if x != y {
            return (x, y);
        }
    }
}

fn rotate(path: &mut Vec<usize>, xi: &mut usize, yi: &mut usize) {
    path[*xi..*yi].reverse();
}

fn cost_v(graph: &Graph, path: &Vec<usize>, vi: &usize) -> f64 {
    let prev = if *vi == 0 { graph.n - 1 } else { vi - 1 };
    let next = if *vi + 1 == graph.n { 0 } else { *vi + 1 };
    let p1 = dist(graph.v[path[prev]], graph.v[path[*vi]]);
    let p2 = dist(graph.v[path[next]], graph.v[path[*vi]]);
    p1 + p2
}

#[derive(Debug)]
struct GraphOpts {
    opt_2: i64,
    swap_edge: i64,
    swap_any: i64,
    max_stuck: i64,
    restarts: i64,
    cooldown: f64,
    init_t: f64,
}

fn parse_args(args: &[String]) -> GraphOpts {
    let mut opts = Options::new();
    opts.optopt("t",
                "opt_2",
                "Probability of opt_2 move (should be between 0 and 1000 inclusive",
                "400");
    opts.optopt("e",
                "swap_edge",
                "Probability of swap edge move (should be between 0 and 1000 inclusive",
                "300");
    opts.optopt("a",
                "swap_any",
                "Probability of swap any move (should be between 0 and 1000 inclusive",
                "300");
    opts.optopt("s",
                "max_stuck",
                "Max stuck time without new solution before restarting",
                "1000");
    opts.optopt("r", "restarts", "How many restarts will be done.", "1000");
    opts.optopt("i", "init_t", "Initial temp for SA.", "1000");
    opts.optopt("c", "cooldown", "Cooling factor for SA.", "0.99");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let mut probs = vec![
        matches.opt_str("t").map(|x| x.parse().unwrap()),
        matches.opt_str("e").map(|x| x.parse().unwrap()),
        matches.opt_str("a").map(|x| x.parse().unwrap()),
    ];
    let fixed_probs = fix_probs(&mut probs);
    GraphOpts {
        opt_2: fixed_probs[0],
        swap_edge: fixed_probs[1],
        swap_any: fixed_probs[2],
        max_stuck: matches.opt_str("s").map(|x| x.parse().unwrap()).unwrap_or(1000),
        restarts: matches.opt_str("r").map(|x| x.parse().unwrap()).unwrap_or(1000),
        init_t: matches.opt_str("i").map(|x| x.parse().unwrap()).unwrap_or(1000.),
        cooldown: matches.opt_str("c").map(|x| x.parse().unwrap()).unwrap_or(0.999),
    }
}

fn keep(new: &f64, old: &f64, temp: &f64, rng: &mut XorShiftRng) -> bool {
    if *temp > 0. {
        let e = f64::consts::E;
        let thres = e.powf(-1. * f64::max(0., *new - *old) / temp);
        let jump = rng.next_f64();
        let res = jump < thres;
        return res;
    } else {
        new < old
    }
}

fn fix_probs(probs: &mut Vec<Option<i64>>) -> Vec<i64> {
    let mut total = 1000;
    for p in &*probs {
        if let Some(pp) = *p {
            total -= pp;
        }
    }
    if total < 0 {
        panic!("Invalid args!");
    }
    // distribute rest of probability
    let mut result: Vec<i64> = vec![];
    for p in probs {
        if let Some(pp) = *p {
            result.push(pp);
        } else {
            result.push(0);
        }
    }
    'outer: loop {
        for mut r in &mut result {
            *r += 1;
            total -= 1;
            if total <= 0 {
                break 'outer;
            }
        }
    }
    result
}

pub fn solve(graph: &Graph, args: &[String]) -> (f64, Vec<usize>) {
    let mut best_result: Vec<usize> = (0..graph.n).step_by(1).collect();
    let mut rng = rand::weak_rng();
    let mut best_cost = 1_000_000_000_f64;
    let op_range = Range::new(0, 1000);
    let v_range = Range::new(0, graph.n);

    let o = parse_args(args);
    println!("{:?}", o);


    for r in 0..o.restarts {
        let mut try_t = 0_f64;
        let mut try_e = 0_f64;
        let mut try_a = 0_f64;
        let mut suc_t = 0_f64;
        let mut suc_e = 0_f64;
        let mut suc_a = 0_f64;
        let mut temp = o.init_t;
        let mut result: Vec<usize> = (0..graph.n).step_by(1).collect();
        rng.shuffle(result.as_mut_slice());
        let mut old_cost = graph.score_path(&result);
        let mut stuck = 0;
        loop {
            let mut new_cost = old_cost;
            let op = op_range.ind_sample(&mut rng);
            if op < o.swap_edge + o.swap_any {
                let (xi, yi) = if op < o.swap_edge {
                    try_e += 1.;
                    let xi = v_range.ind_sample(&mut rng);
                    let yi = (xi + 1) % graph.n;
                    (xi, yi)
                } else {
                    // swap_any
                    try_a += 1.;
                    distinct_pair(&mut rng, v_range)
                };
                new_cost -= cost_v(&graph, &result, &xi);
                new_cost -= cost_v(&graph, &result, &yi);
                result.swap(xi, yi);
                new_cost += cost_v(&graph, &result, &xi);
                new_cost += cost_v(&graph, &result, &yi);

                stuck += 1;

                if keep(&new_cost, &old_cost, &temp, &mut rng) {
                    if op < o.swap_edge {
                        suc_e += 1.;
                    } else {
                        suc_a += 1.;
                    }
                    old_cost = new_cost;
                    if new_cost < old_cost {
                        stuck = 0;
                    }
                } else {
                    // undo swap
                    result.swap(xi, yi);
                }
            } else {
                // 2-opt
                let (mut xi, mut yi) = distinct_pair(&mut rng, v_range);
                try_t += 1.;
                if yi < xi {
                    mem::swap(&mut yi, &mut xi);
                }
                new_cost -= cost_v(&graph, &result, &xi);
                new_cost -= cost_v(&graph, &result, &(yi - 1));
                rotate(&mut result, &mut xi, &mut yi);
                new_cost += cost_v(&graph, &result, &xi);
                new_cost += cost_v(&graph, &result, &(yi - 1));

                stuck += 1;

                if keep(&new_cost, &old_cost, &temp, &mut rng) {
                    if new_cost < 0.0 {
                        panic!("{:?}", new_cost);
                    }
                    old_cost = new_cost;
                    if new_cost < old_cost {
                        stuck = 0;
                    }
                    suc_t += 1.;
                } else {
                    // undo swap
                    rotate(&mut result, &mut xi, &mut yi);
                }

            }
            // TODO: proper termination
            if stuck == o.max_stuck {
                if best_cost > old_cost {
                    graph.visualize(&result, "output.svg");
                    println!("t: {:.5} e: {:.5} a: {:.5}",
                             suc_t / try_t,
                             suc_e / try_e,
                             suc_a / try_a);
                    println!("{}: updating from {:.6} to {:.6}", r, best_cost, old_cost);
                    best_cost = old_cost;
                    best_result = result;
                }
                break;
            }
            if 15 == (stuck & 15) { writeln!(&mut std::io::stderr(), "{:.12} {:.12}", old_cost, temp); }
            temp *= o.cooldown;
        }
    }
    return (best_cost, best_result);
}
