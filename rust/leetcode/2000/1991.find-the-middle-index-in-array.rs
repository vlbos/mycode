/*
 * @lc app=leetcode id=1991 lang=rust
 *
 * [1991] Find the Middle Index in Array
 */

// @lc code=start
impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let total = nums.iter().sum::<i32>();
        let mut sum = 0;
        for (i, num) in nums.iter().enumerate() {
            if sum * 2 + num == total {
                return i as i32;
            }
            sum += num;
        }
        -1
    }
}
// @lc code=end
