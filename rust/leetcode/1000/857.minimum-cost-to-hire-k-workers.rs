/*
 * @lc app=leetcode id=857 lang=rust
 *
 * [857] Minimum Cost to Hire K Workers
 */

// @lc code=start
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut workers: Vec<(f64, i32, i32)> = quality
            .into_iter()
            .zip(wage)
            .map(|(q, w)| ((w as f64) / (q as f64), q, w))
            .collect();
        workers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut ans = f64::MAX;
        let mut pool = std::collections::BinaryHeap::new();
        let mut sumq = 0;
        for &(ratio, q, w) in &workers {
            pool.push(q);
            sumq += q;
            if pool.len() > k as usize {
                sumq -= pool.pop().unwrap();
            }
            if pool.len() == k as usize {
                ans = ans.min(ratio * sumq as f64);
            }
        }
        ans
    }
}
// @lc code=end
