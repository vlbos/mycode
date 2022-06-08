/*
 * @lc app=leetcode id=357 lang=rust
 *
 * [357] Count Numbers with Unique Digits
 */

// @lc code=start
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n==0{
        return 1;
        }
        let n = n as usize;
        let mut predp=9;
        let mut ans = 10;
        for i in 1..n{
            predp=predp*(10-i);
            ans+=predp;
        }
        ans as i32
    }
}
// @lc code=end

