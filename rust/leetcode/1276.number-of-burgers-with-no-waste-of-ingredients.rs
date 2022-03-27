/*
 * @lc app=leetcode id=1276 lang=rust
 *
 * [1276] Number of Burgers with No Waste of Ingredients
 */

// @lc code=start
impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if tomato_slices%2!=0||tomato_slices<2*cheese_slices||4*cheese_slices<tomato_slices{
            return Vec::new();
        }
        vec![tomato_slices/2-cheese_slices,cheese_slices*2-tomato_slices/2]
    }
}
// @lc code=end

