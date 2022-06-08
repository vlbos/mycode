/*
 * @lc app=leetcode id=1894 lang=rust
 *
 * [1894] Find the Student that Will Replace the Chalk
 */

// @lc code=start
impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let sum = chalk.iter().map(|x|(*x) as i64).sum::<i64>();
        let mut r = k as i64 % sum;
        for (i, &v) in chalk.iter().enumerate() {
            r -= v as i64;
            if r < 0 {
                return i as i32;
            }
        }
        0
    }
}
// @lc code=end
