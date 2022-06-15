/*
 * @lc app=leetcode id=2006 lang=rust
 *
 * [2006] Count Number of Pairs With Absolute Difference K
 */

// @lc code=start
impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut cnt = std::collections::HashMap::new();
        for &num in &nums {
            ans += *cnt.get(&(num - k)).unwrap_or(&0) + *cnt.get(&(num + k)).unwrap_or(&0);
            *cnt.entry(num).or_insert(0) += 1;
        }
        ans
    }
}
// @lc code=end
