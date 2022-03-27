/*
 * @lc app=leetcode.cn id=485 lang=rust
 *
 * [485] 最大连续 1 的个数
 */

// @lc code=start
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut s = 0;
        let mut max = 0;
        for n in nums {
            if (n == 1) {
                s += 1;
            } else {
                if s > max {
                    max = s;
                }
                s = 0;
            }
        }
        if s > max {
            max = s;
        }
        max
    }
}
// @lc code=end
