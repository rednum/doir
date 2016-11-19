use std::collections::HashSet;

/// Solve magic squares problem.
///
/// Given integer N generate a magic square:
/// a square array NxN containing integers from 1 to N^2
/// where sum of each row, each column and each diagonal
/// is the same.
///
/// Hint: use hill climbing.

#[derive(Debug)]
pub struct MagicSquare {
    pub size: usize,
    pub numbers: Vec<Vec<i64>>,
}

impl MagicSquare {
    pub fn new(numbers: Vec<Vec<i64>>) -> Result<MagicSquare, String> {
        let n = numbers.len();
        for i in 0..n {
            try_eq!(numbers[i].len(), n);
        }
        let mut rows = vec![0; n];
        let mut cols = vec![0; n];
        let mut numbers_set = HashSet::new();
        let mut d_1 = 0;
        let mut d_2 = 0;
        let e_sum = ((n * (n * n + 1)) / 2) as i64;
        for i in 0..n {
            for j in 0..n {
                rows[i] += numbers[i][j];
                cols[j] += numbers[i][j];
                numbers_set.insert(numbers[i][j]);
            }
            d_1 += numbers[i][i];
            d_2 += numbers[n - i - 1][i];
        }
        try_eq!(d_1, e_sum);
        try_eq!(d_2, e_sum);
        for i in 0..n {
            try_eq!(rows[i], e_sum);
            try_eq!(cols[i], e_sum);
        }
        for i in 1..(n * n + 1) as i64 {
            try_eq!(numbers_set.contains(&i), true);
        }
        Ok(MagicSquare {
            size: n,
            numbers: numbers,
        })
    }
}
