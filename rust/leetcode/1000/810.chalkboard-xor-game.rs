/*
 * @lc app=leetcode id=810 lang=rust
 *
 * [810] Chalkboard XOR Game
 */

// @lc code=start
impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        if nums.len() % 2 == 0 {
            return true;
        }
        let xorsum = nums.into_iter().reduce(|sum, num| sum ^num).unwrap();
        xorsum == 0
    }
}
// @lc code=end
