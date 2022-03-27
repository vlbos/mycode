/*
 * @lc app=leetcode.cn id=338 lang=rust
 *
 * [338] 比特位计数
 */

// @lc code=start
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = (n + 1) as usize; // Thank you Leetcode for your terrible method signatures in Rust.
        let mut count = vec![0; n];
        for i in 1..n {
            count[i] = count[i & (i - 1)] + 1;
        }
        count
    }
}
// @lc code=end
