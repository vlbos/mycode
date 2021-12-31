/*
 * @lc app=leetcode id=492 lang=rust
 *
 * [492] Construct the Rectangle
 */

// @lc code=start
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
         let mut w = (area as f64).sqrt() as i32;
        while area % w != 0 {
            w -= 1;
        }
        vec![area / w, w].to_vec()
    }
}
// @lc code=end

