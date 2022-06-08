/*
 * @lc app=leetcode id=1431 lang=rust
 *
 * [1431] Kids With the Greatest Number of Candies
 */

// @lc code=start
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max:i32 = *(candies.iter().max().unwrap());
        candies
            .iter()
            .map(|c| *c + extra_candies >= max)
            .collect::<Vec<bool>>()
    }
}
// @lc code=end
