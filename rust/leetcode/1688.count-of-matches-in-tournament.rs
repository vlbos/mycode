/*
 * @lc app=leetcode id=1688 lang=rust
 *
 * [1688] Count of Matches in Tournament
 */

// @lc code=start
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut nn = n;
        let mut m = 0;
        while nn>1{
            m+= nn/2;
            nn=nn/2+nn%2;
        }
        m
    }
}
// @lc code=end

