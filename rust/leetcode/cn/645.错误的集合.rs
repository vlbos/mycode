/*
 * @lc app=leetcode.cn id=645 lang=rust
 *
 * [645] 错误的集合
 */

// @lc code=start
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut r = 0;
        for n in &nums {
            r ^= n;
        }
        for i in 1..=nums.len() {
            r ^= i as i32;
        }
        let lowbit = r & (-r);
        let mut num1 = 0;
        let mut num2 = 0;
        for n in &nums {
            if n & lowbit == 0 {
                num1 ^= n;
            } else {
                num2 ^= n;
            }
        }
        for i in 1..=nums.len() {
            if i as i32 & lowbit == 0 {
                num1 ^= i as i32;
            } else {
                num2 ^= i as i32;
            }
        }
        if nums.contains(&num1) {
            vec![num1, num2]
        } else {
            vec![num2, num1]
        }
    }
}
// @lc code=end
