/*
 * @lc app=leetcode id=1137 lang=rust
 *
 * [1137] N-th Tribonacci Number
 */

// @lc code=start
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut r = vec![0, 1, 1, 0];
        if n < 1 {
            return 0;
        } else if n < 3 {
            return 1;
        }
        let mut i = 3;
        while i <= n {
            r[3] = r[2] + r[1] + r[0];
            for j in 1..4 {
                r[j - 1] = r[j];
            }
            i += 1;
        }
        r[3]
    }
}
// @lc code=end
