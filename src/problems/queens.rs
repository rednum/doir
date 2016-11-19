use std::collections::HashSet;
use std::cmp::Ordering;

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
                if queen.0 >= size || queen.1 >= size {
                    return Result::Err(format!("Queen position {:?} out of bounds (should have
                        both coordinates between 0 and {})",
                                               queen,
                                               size));
                }
            }
            // check exactly n columns
            if columns.len() != size {
                return Result::Err(format!("Expected exactly {} columns, got {}.",
                                           size,
                                           columns.len()));
            }
            // check exactly n rows
            if rows.len() != size {
                return Result::Err(format!("Expected exactly {} rows, got {}.", size, rows.len()));
            }
            // check exactly n south-east diagionals
            if se_diagonals.len() != size {
                return Result::Err(format!("Expected exactly {} south east diagonals, got {}.",
                                           size,
                                           se_diagonals.len()));
            }
            // check exactly n ne_diagonals
            if ne_diagonals.len() != size {
                return Result::Err(format!("Expected exactly {} north east diagonals, got {}.",
                                           size,
                                           ne_diagonals.len()));
            }
        }
        return Result::Ok(Board {
            size: size,
            positions: positions,
        });
    }
}
