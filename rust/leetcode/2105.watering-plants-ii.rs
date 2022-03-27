/*
 * @lc app=leetcode id=2105 lang=rust
 *
 * [2105] Watering Plants II
 */

// @lc code=start
impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let mut ans = 0;
        let n = plants.len();
        let (mut pos_a, mut pos_b) = (0, n - 1);
        let (mut val_a, mut val_b) = (capacity_a, capacity_b);
        while pos_a < pos_b {
            if val_a < plants[pos_a] {
                ans += 1;
                val_a = capacity_a - plants[pos_a];
            } else {
                val_a -= plants[pos_a];
            }
            pos_a += 1;
            if val_b < plants[pos_b] {
                ans += 1;
                val_b = capacity_b - plants[pos_b];
            } else {
                val_b -= plants[pos_b];
            }
            pos_b -= 1;
        }
        if pos_a == pos_b {
            if val_a < val_b {
                if val_b < plants[pos_b] {
                    ans += 1;
                }
            } else {
                if val_a < plants[pos_a] {
                    ans += 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
