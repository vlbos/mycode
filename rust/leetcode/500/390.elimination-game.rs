/*
 * @lc app=leetcode id=390 lang=rust
 *
 * [390] Elimination Game
 */

// @lc code=start
impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if  n==1 {1}else{2*(n/2+1-Self::last_remaining(n/2))}
    }
}
// @lc code=end

