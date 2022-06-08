/*
 * @lc app=leetcode id=1547 lang=rust
 *
 * [1547] Minimum Cost to Cut a Stick
 */

// @lc code=start
impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let m = cuts.len();
        let mut cuts = cuts;
        cuts.sort();
        cuts.insert(0, 0);
        cuts.push(n);
        let mut f = vec![vec![0; m + 2]; m + 2];
        for i in (1..=m).rev() {
            for j in i..=m {
                f[i][j] = if i == j {
                    0
                } else {
                    (i..=j).map(|k| f[i][k - 1] + f[k + 1][j]).min().unwrap()
                };
                f[i][j] += cuts[j + 1]-cuts[i - 1] ;
            }
        }
        f[1][m]
    }
}
// @lc code=end
