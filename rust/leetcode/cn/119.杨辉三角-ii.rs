/*
 * @lc app=leetcode.cn id=119 lang=rust
 *
 * [119] 杨辉三角 II
 */

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let len = (row_index + 1) as usize;
        let mut a = Vec::<i32>::with_capacity(len);
        a.resize(len, 0);
        a[0] = 1;
        for i in 1..=row_index {
            let n = (row_index - i + 1) as i64;
            a[i as usize] = ((a[(i - 1) as usize] as i64) * n / (i as i64)) as i32;
        }
        a
    }
}
// @lc code=end
