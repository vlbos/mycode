/*
 * @lc app=leetcode id=2099 lang=rust
 *
 * [2099] Find Subsequence of Length K With the Largest Sum
 */

// @lc code=start
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut sorted_nums: Vec<(i32, usize)> =
        nums.iter().enumerate().map(|(i, &v) | (v, i)).collect();
        sorted_nums.sort();
        let n = sorted_nums.len();
        sorted_nums[n - k as usize..].sort_by_key(|x|x.1);
        sorted_nums[n - k as usize..].iter().map(|(v,_)|*v).collect()
    }
}
// @lc code=end
