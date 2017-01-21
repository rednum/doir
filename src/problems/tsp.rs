use std::collections::HashSet;
use itertools::{zip, chain};
use simplesvg::{Fig, Svg, Attr, Color};
use std::fs::File;
use std::io::Write;


#[derive(Debug)]
pub struct Graph {
    pub v: Vec<(f64, f64)>,
    pub n: usize,
}

pub fn dist(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let d0 = p1.0 - p2.0;
    let d1 = p1.1 - p2.1;
    (d0 * d0 + d1 * d1).sqrt()
}

impl Graph {
    pub fn read_from_stdin() -> Graph {
        let mut v: Vec<(f64, f64)> = vec![];
        let n: usize;
        scan!("{}", n);
        for _ in 0..n {
            let x: f64;
            let y: f64;
            scan!("{} {}", x, y);
            v.push((x, y));
        }
        Graph { v: v, n: n }
    }

    pub fn score_path(&self, path: &Vec<usize>) -> f64 {
        let mut result = dist(self.v[path[0]], self.v[path[self.n - 1]]);
        for i in 0..self.n - 1 {
            result += dist(self.v[path[i]], self.v[path[i + 1]]);
        }
        result
    }

    pub fn verify_path(&self, path: &Vec<usize>) {
        if self.n != path.len() {
            panic!("Path for graph of size {} should have {} vertices but instead got {}",
                   self.n,
                   self.n,
                   path.len());
        }
        let path_set: HashSet<usize> = path.iter().cloned().collect();
        let expected_set: HashSet<usize> = (0..self.n).step_by(1).collect();
        if path_set != expected_set {
            panic!("Expected path to contain each vertex exactly once, however path is {:?}",
                   path);
        }
    }

    pub fn visualize(&self, path: &Vec<usize>, filename: &str) {
        let caption = format!("Score: {}", self.score_path(path));
        let side = 1000.0;
        let margin = 50.0;
        let mut min_x = self.v[0].0;
        let mut max_x = self.v[0].0;
        // let mut max_y = self.v[0].1;
        for p in &self.v {
            min_x = min_x.min(p.0);
            max_x = max_x.max(p.0);
            // max_y = max_x.max(p.1);
        }
        let mut contents = vec![];
        let r = 2.0;
        // TODO add caption on the bottom or top
        contents.push(Fig::Text((margin + side / 2.0) as f32, margin as f32, caption));
        for (i1, i2) in zip(path.iter(),
                            chain(path.iter().skip(1), Some(path[0]).iter())) {
            let q1 = self.v[*i1];
            let q2 = self.v[*i2];
            let p1 = ((q1.0 * side + margin) as f32, (q1.1 * side + margin) as f32);
            let p2 = ((q2.0 * side + margin) as f32, (q2.1 * side + margin) as f32);
            contents.push(Fig::Circle(p1.0 as f32, p1.1 as f32, r));
            contents.push(Fig::Circle(p2.0 as f32, p2.1 as f32, r));
            contents.push(Fig::Line(p1.0 as f32, p1.1 as f32, p2.0 as f32, p2.1 as f32)
                .styled(Attr::default().stroke(Color(0, 0, 0)).stroke_width(r)));
            let mid = (((p1.0 + p2.0) / 2.0) as f32, ((p1.1 + p2.1) / 2.0) as f32);
            contents.push(Fig::Text(mid.0 as f32, mid.1 as f32, format!("{:.3}", dist(q1, q2))));
        }
        let data_s = format!("{:.3}",
                             Svg(contents,
                                 (4.0 * margin + side) as u32,
                                 (2.0 * margin + side) as u32));
        let data_b = data_s.as_bytes();
        let mut f = File::create(filename)
            .expect(&format!("Couldn't open file for writing: {}", filename));
        f.write_all(data_b).expect(&format!("Unable to write data to file: {}", filename));
    }
}
