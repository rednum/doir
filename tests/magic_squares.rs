extern crate discrete_optimization;
extern crate ndarray;

use ndarray::arr2;

use discrete_optimization::problems::magic_squares::MagicSquare;
use discrete_optimization::solutions::magic_squares::solve;

#[test]
fn check_3x3_square() {
    let numbers = arr2(&[[4, 9, 2],
                         [3, 5, 7],
                         [8, 1, 6]]);
    let b = MagicSquare::new(numbers);
    assert!(b.is_ok());
}

#[test]
fn malformed_square_errs() {
    let numbers = arr2(&[[4, 9, 2],
                         [3, 5, 4]]);
    let b = MagicSquare::new(numbers);
    assert!(b.is_err());
}

#[test]
fn square_with_duplicates_fails() {
    let numbers = arr2(&[[4, 4, 4],
                         [4, 4, 4],
                         [4, 4, 4]]);
    let b = MagicSquare::new(numbers);
    assert!(b.is_err());
}

#[test]
fn square_with_wrong_sums_fails() {
    let numbers = arr2(&[[4, 2, 9],
                         [3, 5, 7],
                         [8, 1, 6]]);
    let b = MagicSquare::new(numbers);
    assert!(b.is_err());
}

#[test]
fn generate_7x7_square() {
    let _ = solve(7);
}
