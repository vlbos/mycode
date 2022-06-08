/*
 * @lc app=leetcode id=1792 lang=rust
 *
 * [1792] Maximum Average Pass Ratio
 */

// @lc code=start
impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut q = std::collections::BinaryHeap::new();
        use std::cmp::Ordering;
        #[derive(PartialEq, PartialOrd)]
        struct Rate(f64);
        impl Ord for Rate {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.partial_cmp(&other.0).unwrap_or(Ordering::Less)
            }
        }
        impl Eq for Rate {}
        for (i, c) in classes.iter().enumerate() {
            let (p, t) = (c[0] as f64, c[1] as f64);
            q.push((Rate((t - p) / (t + 1.0) / t), Rate(p), Rate(t)));
        }
        for _ in 0..extra_students {
            if let Some((_, p, t)) = q.pop() {
                let (p, t) = (p.0 + 1.0, t.0 + 1.0);
                q.push((Rate((t - p) / (t + 1.0) / t), Rate(p), Rate(t)));
            }
        }
        let mut ans = q.iter().map(|x| x.1 .0 / x.2 .0).sum::<f64>();
        ans / classes.len() as f64
    }
}
// @lc code=end
