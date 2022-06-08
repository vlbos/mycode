/*
 * @lc app=leetcode id=2044 lang=rust
 *
 * [2044] Count Number of Maximum Bitwise-OR Subsets
 */

// @lc code=start
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_value = 0;
        let mut cnt = 0;
        let n1 = 1 << n;
        for i in 1..n1 {
            let or_val = nums
                .iter()
                .enumerate()
                .filter(|(j, _)| ((i >> j) & 1) > 0)
                .map(|x|x.1).cloned().reduce(|acc, v| acc|v).unwrap();
            if or_val > max_value {
                max_value = or_val;
                cnt = 1;
            } else if or_val == max_value {
                cnt += 1;
            }
        }
        cnt
    }
}
// @lc code=end
