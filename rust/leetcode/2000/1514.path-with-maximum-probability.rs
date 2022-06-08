/*
 * @lc app=leetcode id=1514 lang=rust
 *
 * [1514] Path with Maximum Probability
 */

// @lc code=start
#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn max_probability(
        n: i32,
        mut edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let mut x = std::collections::BinaryHeap::new();
        let mut ed: Vec<Vec<(MinNonNan, i32)>> = vec![Vec::new(); n as usize];
        edges.drain(..).zip(succ_prob).for_each(|(x, p)| {
            let (x, y, p) = (x[0], x[1], MinNonNan(p));
            ed[x as usize].push((p, y));
            ed[y as usize].push((p, x));
        });
        let mut probG = vec![0f64; n as usize];
        probG[start as usize] = 1.0f64;
        x.push((MinNonNan(1.0f64), start));
        while let Some(r) = x.pop() {
            for (prob, to) in ed[r.1 as usize].iter().copied() {
                if (r.0).0 * prob.0 > probG[to as usize] {
                    probG[to as usize] = (r.0).0 * prob.0;
                    x.push((MinNonNan(probG[to as usize]), to));
                }
            }
        }
        probG[end as usize]
    }
}
// @lc code=end
