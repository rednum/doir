use rand::distributions::{IndependentSample, Range};
use rand::{Rng, ThreadRng};
use rand;

use std::cmp::max;

use ::problems::magic_squares::MagicSquare;

pub fn solve(n: usize) -> MagicSquare {
    let retries = 10_000_000;
    let timeout = n.pow(4) * 5;
    let mut rng = rand::thread_rng();

    for _ in 0..retries {
        let between = Range::new(0, n);
        let mut rows = vec![0; n];
        let mut cols = vec![0; n];
        let mut diags = vec![0, 0];
        let mut board = make_board(&mut rng, &n);
        let mut score = initial_score(&mut board, &mut rows, &mut cols, &mut diags, &n);
        let mut stuck = 0;
        let mut max_stuck = 0;
        loop {
            let x1 = between.ind_sample(&mut rng);
            let x2 = between.ind_sample(&mut rng);
            let y1 = between.ind_sample(&mut rng);
            let y2 = between.ind_sample(&mut rng);

            let new_score = swap_nubmers(x1,
                                         y1,
                                         x2,
                                         y2,
                                         &mut board,
                                         &mut rows,
                                         &mut cols,
                                         &mut diags,
                                         &score,
                                         &n);
            if new_score == 0 {
                return MagicSquare::new(board).unwrap();
            }
            if new_score < score {
                stuck = 0;
            } else {
                stuck += 1;
                max_stuck = max(max_stuck, stuck);
            }
            if new_score <= score {
                score = new_score;
                continue;
            }
            if stuck > timeout {
                // restart
                break;
            }
            // undo swap
            score = swap_nubmers(x1,
                                 y1,
                                 x2,
                                 y2,
                                 &mut board,
                                 &mut rows,
                                 &mut cols,
                                 &mut diags,
                                 &new_score,
                                 &n);
        }
    }
    panic!("Did not find a solution");
}


fn swap_nubmers(x1: usize,
                y1: usize,
                x2: usize,
                y2: usize,
                board: &mut Vec<Vec<i64>>,
                rows: &mut Vec<i64>,
                cols: &mut Vec<i64>,
                diags: &mut Vec<i64>,
                score: &i64,
                n: &usize)
                -> i64 {
    let mut new_score: i64 = *score;
    let ni = *n as i64;

    new_score -= score_sum(&rows[x1], &ni);
    new_score -= score_sum(&rows[x2], &ni);
    new_score -= score_sum(&cols[y1], &ni);
    new_score -= score_sum(&cols[y2], &ni);
    new_score -= score_sum(&diags[0], &ni);
    new_score -= score_sum(&diags[1], &ni);

    // diagonals
    if x1 == y1 {
        diags[0] -= board[x1][y1];
    }
    if x1 + y1 == n - 1 {
        diags[1] -= board[x1][y1];
    }
    if x2 == y2 {
        diags[0] -= board[x2][y2];
    }
    if x2 + y2 == n - 1 {
        diags[1] -= board[x2][y2];
    }

    rows[x1] -= board[x1][y1];
    rows[x2] -= board[x2][y2];
    cols[y1] -= board[x1][y1];
    cols[y2] -= board[x2][y2];

    let tmp = board[x1][y1];
    board[x1][y1] = board[x2][y2];
    board[x2][y2] = tmp;

    rows[x1] += board[x1][y1];
    rows[x2] += board[x2][y2];
    cols[y1] += board[x1][y1];
    cols[y2] += board[x2][y2];

    if x1 == y1 {
        diags[0] += board[x1][y1];
    }
    if x1 + y1 == n - 1 {
        diags[1] += board[x1][y1];
    }
    if x2 == y2 {
        diags[0] += board[x2][y2];
    }
    if x2 + y2 == n - 1 {
        diags[1] += board[x2][y2];
    }

    new_score += score_sum(&rows[x1], &ni);
    new_score += score_sum(&rows[x2], &ni);
    new_score += score_sum(&cols[y1], &ni);
    new_score += score_sum(&cols[y2], &ni);
    new_score += score_sum(&diags[0], &ni);
    new_score += score_sum(&diags[1], &ni);

    new_score
}

fn score_sum(s: &i64, n: &i64) -> i64 {
    let e = n * (n * n + 1) / 2;
    (s - e).abs() // * (s - e).abs() // * (s - e).abs()
}

fn score_vector(v: &Vec<i64>, n: &i64) -> i64 {
    v.iter().map(|x| score_sum(x, n)).sum()
}

fn initial_score(board: &mut Vec<Vec<i64>>,
                 rows: &mut Vec<i64>,
                 cols: &mut Vec<i64>,
                 diags: &mut Vec<i64>,
                 n: &usize)
                 -> i64 {
    let ni = *n as i64;
    for i in 0..*n {
        for j in 0..*n {
            if i == j {
                diags[0] += board[i][j];
            }
            if i + j == n - 1 {
                diags[1] += board[i][j];
            }
            rows[i] += board[i][j];
            cols[j] += board[i][j];
        }
    }
    score_vector(&rows, &ni) + score_vector(&cols, &ni) + score_vector(&diags, &ni)
}

fn make_board(rng: &mut ThreadRng, n: &usize) -> Vec<Vec<i64>> {
    let ni = *n as i64;
    let mut numbers: Vec<i64> = (1..ni * ni + 1).collect();
    rng.shuffle(numbers.as_mut_slice());
    let mut result: Vec<Vec<i64>> = vec![vec![0; *n]; *n];
    for i in 0..*n {
        for j in 0..*n {
            result[i][j] = numbers[i + n * j];
        }
    }
    result
}
