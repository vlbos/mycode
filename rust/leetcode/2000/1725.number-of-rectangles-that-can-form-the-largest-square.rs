/*
 * @lc app=leetcode id=1725 lang=rust
 *
 * [1725] Number Of Rectangles That Can Form The Largest Square
 */

// @lc code=start
impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let v = rectangles.iter().map(|x| x[0].min(x[1])).max().unwrap();
        rectangles.iter().filter(|x| x[0].min(x[1])==v ).count() as i32
    }
}
// @lc code=end

