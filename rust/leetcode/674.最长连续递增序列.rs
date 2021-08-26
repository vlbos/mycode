/*
 * @lc app=leetcode.cn id=674 lang=rust
 *
 * [674] 最长连续递增序列
 */

// @lc code=start
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut pre = i32::MIN;
        let mut max = r;
        for n in &nums {
            if *n > pre {
                r += 1;
            } else {
                if r > max {
                    max = r;
                }
                r = 1;
            }
            pre = *n;
        }
        if r > max {
            max = r;
        }
        max
    }
}
// @lc code=end
