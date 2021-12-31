/*
 * @lc app=leetcode id=1128 lang=rust
 *
 * [1128] Number of Equivalent Domino Pairs
 */

// @lc code=start
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut r = vec![vec![0; 10]; 10];
        for i in 0..dominoes.len() {
            r[dominoes[i][0] as usize][dominoes[i][1] as usize] += 1;
        }
        let mut ans = 0;
        for i in 0..10 {
            for j in 0..=i {
                let mut s = r[i][j];
                if i != j {
                    s += r[j][i];
                }
                if s >= 2 {
                    let mut n = 1;
                    let ss = s;
                    while s > ss - 2 {
                        n *= s;
                        s -= 1;
                    }
                    ans += n / 2;
                }
            }
        }
        ans
    }
}
// @lc code=end
