extern crate discrete_optimization;

use discrete_optimization::problems::queens::Board;
use discrete_optimization::solutions::queens::solve;

#[test]
fn check_4x4_board() {
    let n = 4;
    let positions = vec![(0, 1), (1, 3), (2, 0), (3, 2)].into_iter().collect();
    let b = Board::new(n, positions);
    assert!(b.is_ok());
}

#[test]
fn incomplete_board_errs() {
    let n = 4;
    let positions = vec![(1, 3), (2, 0), (3, 2)].into_iter().collect();
    let b = Board::new(n, positions);
    assert!(b.is_err());
}

#[test]
fn board_with_collision_errs() {
    let n = 4;
    let positions = vec![(1, 1), (1, 3), (2, 0), (3, 2)].into_iter().collect();
    let b = Board::new(n, positions);
    assert!(b.is_err());
}

#[test]
fn generate_8x8_board() {
    let _ = solve(8);
}
