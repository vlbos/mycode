/*
 * @lc app=leetcode id=1383 lang=rust
 *
 * [1383] Maximum Performance of a Team
 */

// @lc code=start
impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut se: Vec<(i32, i32)> = speed.into_iter().zip(efficiency).collect();
        se.sort_by_key(|x| -x.1);
        let mut q = std::collections::BinaryHeap::new();
        use std::cmp::Reverse;
        let mut ans = 0;
        let mut sum = 0;
        for (i, &(s, e)) in se.iter().enumerate() {
            let (min_e, sum_s) = (e as i64, sum + s as i64);
            ans = ans.max(min_e * sum_s);
            q.push(Reverse((s, e)));
            sum += s as i64;
            if q.len() == k as usize {
                sum -= q.pop().unwrap().0.0 as i64;
            }
        }
        (ans % 1_000_000_007) as _
    }
}
// @lc code=end
