/*
 * @lc app=leetcode.cn id=367 lang=rust
 *
 * [367] 有效的完全平方数
 */

// @lc code=start
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut i = 0 as i64;
        let mut j = (num / 2 + 1) as i64;
        let num = num as i64;
        while i <= j {
            let mid = (i + j) / 2;
            let mut r = mid * mid;
            if r > num {
                j = mid - 1;
            } else if r < num {
                i = mid + 1;
            } else {
                return true;
            }
        }
        false
    }
}
// @lc code=end
