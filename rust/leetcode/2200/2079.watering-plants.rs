/*
 * @lc app=leetcode id=2079 lang=rust
 *
 * [2079] Watering Plants
 */

// @lc code=start
impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut ans = 0;
        let mut rest = capacity;
        for (i, &p) in plants.iter().enumerate() {
            if rest >= p {
                ans += 1;
                rest -= p;
            } else {
                ans += i as i32 * 2 + 1;
                rest = capacity - p;
            }
        }
        ans
    }
}
// @lc code=end
