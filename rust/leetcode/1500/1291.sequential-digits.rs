/*
 * @lc app=leetcode id=1291 lang=rust
 *
 * [1291] Sequential Digits
 */

// @lc code=start
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 1..=9 {
            let mut num = i;
            for j in i + 1..=9 {
                num = num * 10 + j;
                if num >= low && num <= high {
                    ans.push(num);
                }
            }
        }
        ans.sort();
        ans
    }
}
// @lc code=end
