#[macro_use]
extern crate text_io;
extern crate discrete_optimization;

use std::env;
use discrete_optimization::solutions::magic_squares::solve as solve_magic_squares;
use discrete_optimization::solutions::queens::solve as solve_queens;
use discrete_optimization::solutions::tsp::solve as solve_tsp;
use discrete_optimization::problems::tsp::Graph;

static PROBLEMS: [&'static str; 3] = ["queens", "magic_squares", "tsp"];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Specify problem name. Avaliable problems: {:?}.", PROBLEMS);
    }

    match args[1].as_ref() {
        "queens" => {
            let n: usize;
            scan!("{}", n);

            println!("{:?}", n);
            let solution = solve_queens(n);
            println!("{:?}", solution);
        }
        "magic_squares" => {
            let n: usize;
            scan!("{}", n);
            let solution = solve_magic_squares(n);
            println!("{:?}", solution);
        }
        "tsp" => {
            let graph = Graph::read_from_stdin();
            let solution = solve_tsp(&graph, &args);
            println!("{:?}", solution);
        }
        arg => {
            if PROBLEMS.contains(&arg) {
                panic!("Problem {} is on list of supported problems but there's no code to handle
                       it.",
                       arg);
            }
            panic!("Unrecognized problem: {}, supported problems: {:?}",
                   arg,
                   PROBLEMS);
        }
    }
}
