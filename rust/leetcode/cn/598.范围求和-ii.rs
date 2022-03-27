/*
 * @lc app=leetcode.cn id=598 lang=rust
 *
 * [598] 范围求和 II
 */

// @lc code=start
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut min = vec![m, n];
        let mut count = 0;
        for rc in &ops {
            for j in 0..rc.len() {
                if min[j] > rc[j] {
                    min[j] = rc[j];
                }
            }
        }
        min[0] * min[1]
    }
}
// @lc code=end
