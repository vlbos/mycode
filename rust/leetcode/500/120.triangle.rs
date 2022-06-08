/*
 * @lc app=leetcode id=120 lang=rust
 *
 * [120] Triangle
 */

// @lc code=start
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle=triangle;
         for y in (0..triangle.len() - 1).rev() {
            for x in (0..triangle[y].len()) {
                triangle[y][x] += triangle[y + 1][x].min(triangle[y + 1][x + 1]);
            }
        }
        triangle[0][0]
    }
}
// @lc code=end

