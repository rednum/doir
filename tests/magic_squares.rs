extern crate discrete_optimization;

use discrete_optimization::problems::magic_squares::MagicSquare;
use discrete_optimization::solutions::magic_squares::solve;

#[test]
fn check_3x3_square() {
    let numbers = vec![vec![4, 9, 2],
                       vec![3, 5, 7],
                       vec![8, 1, 6]];
    let b = MagicSquare::new(numbers);
    assert!(b.is_ok());
}

#[test]
fn malformed_square_errs() {
    let numbers = vec![vec![4, 9, 2],
                       vec![3, 5],
                       vec![8, 1, 6]];
    let b = MagicSquare::new(numbers);
    assert!(b.is_err());
}

#[test]
fn square_with_duplicates_fails() {
    let numbers = vec![vec![4, 4, 4],
                       vec![4, 4, 4],
                       vec![4, 4, 4]];
    let b = MagicSquare::new(numbers);
    assert!(b.is_err());
}

#[test]
fn square_with_wrong_sums_fails() {
    let numbers = vec![vec![4, 2, 9],
                       vec![3, 5, 7],
                       vec![8, 1, 6]];
    let b = MagicSquare::new(numbers);
    assert!(b.is_err());
}

#[test]
fn generate_7x7_square() {
    let _ = solve(7);
}
