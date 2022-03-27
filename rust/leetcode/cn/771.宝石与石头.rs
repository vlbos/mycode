/*
 * @lc app=leetcode.cn id=771 lang=rust
 *
 * [771] 宝石与石头
 */

// @lc code=start
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut sum = 0;
        for c in stones.chars() {
            if jewels.contains(&c.to_string()) {
                sum += 1;
            }
        }
        sum
    }
}
// @lc code=end
