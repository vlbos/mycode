/*
 * @lc app=leetcode id=1872 lang=rust
 *
 * [1872] Stone Game VIII
 */

// @lc code=start
impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut pre = Vec::new();
        let mut sum = 0;
        for &stone in &stones {
            sum += stone;
            pre.push(sum);
        }
        let mut f = vec![0; n];
        f[n - 1] = pre[n - 1];
        for i in (1..n - 1).rev() {
            f[i] = f[i + 1].max(pre[i] - f[i + 1]);
        }
        f[1]
    }
}
// @lc code=end
