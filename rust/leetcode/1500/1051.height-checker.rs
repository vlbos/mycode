/*
 * @lc app=leetcode id=1051 lang=rust
 *
 * [1051] Height Checker
 */

// @lc code=start
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut expected = heights.clone();
        expected.sort();
        for i in 0..heights.len(){
            if heights[i]!=expected[i]{
             r+=1;
            }
        }
        r
    }
}
// @lc code=end

