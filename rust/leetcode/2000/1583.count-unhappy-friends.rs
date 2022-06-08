/*
 * @lc app=leetcode id=1583 lang=rust
 *
 * [1583] Count Unhappy Friends
 */

// @lc code=start
impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
       let n = n as usize;
        let mut order = vec![vec![0; n];n];
        for i in 0..n {
            for j in 0..(n-1) {
                order[i][preferences[i][j] as usize] = j;
            }
        }
        let mut matches = vec![0; n];
        for p in &pairs {
            matches[p[0] as usize] = p[1] as usize;
            matches[p[1] as usize] = p[0] as usize;
        }
        let mut ans = 0;
        for x in 0..n {
            let y = matches[x];
            let index = order[x][y];
            for i in 0..index {
                let u = preferences[x][i] as usize;
                let v = matches[u];
                if order[u][x] < order[u][v] {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}
// @lc code=end
