/*
 * @lc app=leetcode id=1433 lang=rust
 *
 * [1433] Check If a String Can Break Another String
 */

// @lc code=start
impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let (mut v1,mut v2)=(s1.bytes().collect::<Vec<u8>>(),s2.bytes().collect::<Vec<u8>>());
        v1.sort();
        v2.sort();
        v1.iter().zip(&v2).all(|x|*x.0>=*x.1)||v1.iter().zip(&v2).all(|x|*x.1>=*x.0)
    }
}
// @lc code=end

