#[macro_use]
extern crate text_io;

extern crate discrete_optimization;

use std::env;

use discrete_optimization::solutions::magic_squares::solve as solve_magic_squares;
use discrete_optimization::solutions::queens::solve as solve_queens;

static PROBLEMS: [&'static str; 2] = ["queens", "magic_squares"];

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
