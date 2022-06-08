/*
 * @lc app=leetcode id=1975 lang=rust
 *
 * [1975] Maximum Matrix Sum
 */

// @lc code=start
impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut cnt = 0;
        let mut total = 0i64;
        let mut mn = i32::MAX;
        for r in &matrix {
            for c in r {
                let ac = c.abs();
                mn = mn.min(ac);
                if *c < 0 {
                    cnt += 1;
                }
                total += ac as i64;
            }
        }
        if cnt % 2 == 0 {
            total
        } else {
            total - 2 * mn as i64
        }
    }
}
// @lc code=end
