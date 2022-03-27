/*
 * @lc app=leetcode id=1222 lang=rust
 *
 * [1222] Queens That Can Attack the King
 */

// @lc code=start
impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let n = 8;
        let mut ans = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let (mut nx, mut ny) = (king[0], king[1]);
                loop {
                    nx += i;
                    ny += j;
                    if nx < 0 || nx >= n || ny < 0 || ny >= n {
                        break;
                    }
                    let np = vec![nx, ny];
                    if queens.contains(&np) {
                        ans.push(np);
                        break;
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
