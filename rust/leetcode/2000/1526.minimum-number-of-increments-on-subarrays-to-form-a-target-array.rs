/*
 * @lc app=leetcode id=1526 lang=rust
 *
 * [1526] Minimum Number of Increments on Subarrays to Form a Target Array
 */

// @lc code=start
impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let n = target.len();
        let mut ans = target[0];
        for i in 1..n {
            ans += 0i32.max(target[i] - target[i - 1]);
        }
        ans
    }
}
// @lc code=end
