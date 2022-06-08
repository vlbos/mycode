/*
 * @lc app=leetcode id=1980 lang=rust
 *
 * [1980] Find Unique Binary String
 */

// @lc code=start
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut ns = std::collections::HashSet::new();
        for bin_idx in &nums {
            ns.insert(i32::from_str_radix(bin_idx, 2).unwrap());
        }
        let mut val = 0;
        while ns.contains(&val) {
            val += 1;
        }
       format!("{:0w$b}", val,w=nums.len())
    }
}
// @lc code=end
