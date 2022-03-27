/*
 * @lc app=leetcode id=1679 lang=rust
 *
 * [1679] Max Number of K-Sum Pairs
 */

// @lc code=start
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut m = std::collections::HashMap::new();
        for &n in &nums {
            *m.entry(n).or_insert(0) += 1;
        }
        for (&n, &c) in &m {
            if let Some(cnt) = m.get(&(k - n)) {
                ans += c.min(*cnt);
            }
        }
        ans / 2
    }
}
// @lc code=end
