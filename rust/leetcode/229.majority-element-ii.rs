/*
 * @lc app=leetcode id=229 lang=rust
 *
 * [229] Majority Element II
 */

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();
        for n in &nums {
            *m.entry(n).or_insert(0) += 1;
        }
        let mut ans = Vec::new();
        let n3 = nums.len() / 3;
        for (k, v) in &m {
            if *v > n3 {
                ans.push(**k);
            }
        }
        ans
    }
}
// @lc code=end
