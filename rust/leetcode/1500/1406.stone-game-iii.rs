/*
 * @lc app=leetcode id=1406 lang=rust
 *
 * [1406] Stone Game III
 */

// @lc code=start
impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut f = vec![i32::MIN; n + 1];
        f[n] = 0;
        for i in (0..n).rev() {
            let mut pre = 0;
            for j in i + 1..=n.min(i + 3) {
                pre += stone_value[j - 1];
                f[i] = f[i].max(pre - f[j]);
            }
        }
        if f[0] == 0 {
            return "Tie".to_string();
        }
        if f[0] > 0 {
            "Alice".to_string()
        } else {
            "Bob".to_string()
        }
    }
}
// @lc code=end
