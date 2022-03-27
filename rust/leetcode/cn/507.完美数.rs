/*
 * @lc app=leetcode.cn id=507 lang=rust
 *
 * [507] 完美数
 */

// @lc code=start
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let mut i = 2;
        let mut sum = 1;
        while i * i < num {
            if num % i == 0 {
                sum += i;
                if i * i != num {
                    sum += num / i;
                }
            }
            i += 1;
        }
        sum == num
    }
}
// @lc code=end
