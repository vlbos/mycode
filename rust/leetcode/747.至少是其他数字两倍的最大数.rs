/*
 * @lc app=leetcode.cn id=747 lang=rust
 *
 * [747] 至少是其他数字两倍的最大数
 */

// @lc code=start
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut m = 0;
        let mut index = -1i32;
        for (i, n) in nums.iter().enumerate() {
            if m < *n {
                m = *n;
                index = i as i32;
            }
        }
        for n in &nums {
            if m != *n && m < 2 * *n {
                return -1;
            }
        }
        index
    }
}
// @lc code=end
