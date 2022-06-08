/*
 * @lc app=leetcode id=1689 lang=rust
 *
 * [1689] Partitioning Into Minimum Number Of Deci-Binary Numbers
 */

// @lc code=start
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        ( n.bytes().max().unwrap()-b'0') as _
    }
}
// @lc code=end

