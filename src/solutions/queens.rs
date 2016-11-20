use std::collections::HashSet;

use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use rand;

use ::problems::queens::{Board, ne_d_index, se_d_index};

fn swap_and_score(a: &(usize, usize),
                  b: &(usize, usize),
                  ne_diagonals: &mut Vec<i64>,
                  se_diagonals: &mut Vec<i64>,
                  n: usize,
                  old_score: i64)
                  -> i64 {
    let mut new_score = old_score.clone();

    new_score -= conflicts(&ne_diagonals[ne_d_index(a)]);
    new_score -= conflicts(&se_diagonals[se_d_index(a, n)]);
    ne_diagonals[ne_d_index(a)] -= 1;
    se_diagonals[se_d_index(a, n)] -= 1;
    new_score += conflicts(&ne_diagonals[ne_d_index(a)]);
    new_score += conflicts(&se_diagonals[se_d_index(a, n)]);

    new_score -= conflicts(&ne_diagonals[ne_d_index(b)]);
    new_score -= conflicts(&se_diagonals[se_d_index(b, n)]);
    ne_diagonals[ne_d_index(b)] -= 1;
    se_diagonals[se_d_index(b, n)] -= 1;
    new_score += conflicts(&ne_diagonals[ne_d_index(b)]);
    new_score += conflicts(&se_diagonals[se_d_index(b, n)]);

    let c = (a.0, b.1);
    let d = (b.0, a.1);

    new_score -= conflicts(&ne_diagonals[ne_d_index(&c)]);
    new_score -= conflicts(&se_diagonals[se_d_index(&c, n)]);
    ne_diagonals[ne_d_index(&c)] += 1;
    se_diagonals[se_d_index(&c, n)] += 1;
    new_score += conflicts(&ne_diagonals[ne_d_index(&c)]);
    new_score += conflicts(&se_diagonals[se_d_index(&c, n)]);

    new_score -= conflicts(&ne_diagonals[ne_d_index(&d)]);
    new_score -= conflicts(&se_diagonals[se_d_index(&d, n)]);
    ne_diagonals[ne_d_index(&d)] += 1;
    se_diagonals[se_d_index(&d, n)] += 1;
    new_score += conflicts(&ne_diagonals[ne_d_index(&d)]);
    new_score += conflicts(&se_diagonals[se_d_index(&d, n)]);

    new_score
}

fn conflicts(x: &i64) -> i64 {
    (x - 1) * x
}

fn score(ne_diagonals: &Vec<i64>, se_diagonals: &Vec<i64>) -> i64 {
    ne_diagonals.iter().map(|x| conflicts(x)).sum::<i64>() +
    se_diagonals.iter().map(|x| conflicts(x)).sum::<i64>()
}

fn to_position_set(positions: Vec<usize>) -> HashSet<(usize, usize)> {
    positions.into_iter().enumerate().collect()
}

pub fn solve(n: usize) -> Board {
    let mut positions = Vec::new();
    let mut ne_diagonals: Vec<i64> = vec![0; 2 * n + 1];
    let mut se_diagonals: Vec<i64> = vec![0; 2 * n + 1];
    let fuel_limit = n * n;
    let restart_limit = n * n * n * n;

    for i in 0..n {
        positions.push(i);
    }
    let between = Range::new(0, n);
    let mut rng = rand::weak_rng();

    for _ in 0..restart_limit {
        se_diagonals.iter_mut().map(|mut x| *x = 0).collect::<Vec<()>>();
        ne_diagonals.iter_mut().map(|mut x| *x = 0).collect::<Vec<()>>();

        rng.shuffle(positions.as_mut_slice());
        for (i, p) in positions.iter().enumerate() {
            let queen = (i, *p);
            se_diagonals[se_d_index(&queen, n)] += 1;
            ne_diagonals[ne_d_index(&queen)] += 1;
        }
        let mut old_score = score(&ne_diagonals, &se_diagonals);

        for _ in 0..fuel_limit {
            let ac = between.ind_sample(&mut rng);
            let bc = between.ind_sample(&mut rng);
            if ac == bc {
                continue;
            }
            let ar = positions[ac];
            let br = positions[bc];
            let new_score = swap_and_score(&(ac, ar),
                                           &(bc, br),
                                           &mut ne_diagonals,
                                           &mut se_diagonals,
                                           n,
                                           old_score);
            if new_score < old_score {
                // complete swap
                positions[ac] = br;
                positions[bc] = ar;
                old_score = new_score;
            } else {
                // undo swap
                let undo = swap_and_score(&(ac, br),
                                          &(bc, ar),
                                          &mut ne_diagonals,
                                          &mut se_diagonals,
                                          n,
                                          new_score);
                assert_eq!(undo, old_score);
            }
            if old_score == 0 {
                return Board::new(n, to_position_set(positions)).unwrap();
            }
        }
    }
    panic!("Restart limit reached!");
}
