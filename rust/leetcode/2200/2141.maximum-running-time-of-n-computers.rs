/*
 * @lc app=leetcode id=2141 lang=rust
 *
 * [2141] Maximum Running Time of N Computers
 */

// @lc code=start
impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let mut ans = 0;
        let n = n as i64;
        let (mut left, mut right) = (0, batteries.iter().map(|x| *x as i64).sum::<i64>() / n);
        while left <= right {
            let mid = (left + right) / 2;
            let total = batteries.iter().map(|x| mid.min(*x as i64)).sum::<i64>();
            if total >= n * mid {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }
}
// @lc code=end
