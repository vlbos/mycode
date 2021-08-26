/*
 * @lc app=leetcode.cn id=191 lang=rust
 *
 * [191] 位1的个数
 */

// @lc code=start
impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut n = n;
        let mut c = 0;
        while n > 0 {
            n &= n - 1;
            c += 1;
        }
        c
    }
}
// @lc code=end
