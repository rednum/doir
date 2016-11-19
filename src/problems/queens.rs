use std::collections::HashSet;

/// Solve N queens problem.
///
/// Given integer N generate chessboard containing
/// N queens such that none pair of queens attacks
/// each other. The Board datastructure expects
/// a set of queens positions on the board 
/// (see tests/queens.rs).
///
/// Hint: use hill climbing.

#[derive(Debug)]
pub struct Board {
    pub size: usize,
    pub positions: HashSet<(usize, usize)>,
}

// helpers to calculate on which diagonal tile is
pub fn ne_d_index(queen: &(usize, usize)) -> usize {
    return queen.0 + queen.1;
}

pub fn se_d_index(queen: &(usize, usize), n: usize) -> usize {
    return n + queen.0 - queen.1 - 1;
}

impl Board {
    pub fn new(size: usize, positions: HashSet<(usize, usize)>) -> Result<Board, String> {
        try_gt!(size, 0);
        try_eq!(positions.len(), size);
        {
            let mut columns = HashSet::new();
            let mut rows = HashSet::new();
            // check "\" diagonals
            let mut se_diagonals = HashSet::new();
            // check "/" diagonals
            let mut ne_diagonals = HashSet::new();
            for queen in &positions {
                columns.insert(&queen.0);
                rows.insert(&queen.1);
                se_diagonals.insert(se_d_index(queen, size));
                ne_diagonals.insert(ne_d_index(queen));
                try_lt!(queen.0, size);
                try_lt!(queen.1, size);
            }
            // check exactly n columns
            try_eq!(columns.len(), size);
            // check exactly n rows
            try_eq!(rows.len(), size);
            // check exactly n south-east diagionals
            try_eq!(se_diagonals.len(), size);
            // check exactly n ne_diagonals
            try_eq!(ne_diagonals.len(), size);
        }
        return Result::Ok(Board {
            size: size,
            positions: positions,
        });
    }
}
