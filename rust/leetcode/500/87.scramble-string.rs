/*
 * @lc app=leetcode id=87 lang=rust
 *
 * [87] Scramble String
 */

// @lc code=start
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        fn dfs(
            i1: usize,
            i2: usize,
            len: usize,
            s1: &[u8],
            s2: &[u8],
            memo: &mut Vec<Vec<Vec<i32>>>,
        ) -> bool {
            let check_if_similar = || -> bool {
                let mut f = std::collections::HashMap::new();
                for i in i1..i1 + len {
                    *f.entry(s1[i]).or_insert(0) += 1;
                }
                for i in i2..i2 + len {
                    *f.entry(s2[i]).or_insert(0) -= 1;
                }
                f.iter().all(|x| *x.1 == 0)
            };
            if memo[i1][i2][len] != 0 {
                return memo[i1][i2][len] == 1;
            }
            if s1[i1..i1 + len] == s2[i2..i2 + len] {
                memo[i1][i2][len] = 1;
                return true;
            }
            if !check_if_similar() {
                memo[i1][i2][len] = -1;
                return false;
            }
            for i in 1..len {
                if dfs(i1, i2, i, s1, s2, memo) && dfs(i1 + i, i2 + i, len - i, s1, s2, memo) {
                    memo[i1][i2][len] = 1;
                    return true;
                }
                if dfs(i1, i2 + len - i, i, s1, s2, memo) && dfs(i1 + i, i2, len - i, s1, s2, memo)
                {
                    memo[i1][i2][len] = 1;
                    return true;
                }
            }
            memo[i1][i2][len] = -1;
            return false;
        }
        let mut memo = vec![vec![vec![0; 31]; 30]; 30];
        dfs(0, 0, s1.len(), s1.as_bytes(), s2.as_bytes(), &mut memo)
    }
}
// @lc code=end
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n=s1.len();
        let (bs1,bs2)=(s1.as_bytes(),s2.as_bytes());
        let mut dp=vec![vec![vec![false;n+1];n];n];
        for i in 0..n{
            for j in 0..n{
                dp[i][j][1]=bs1[i]==bs2[j];
            }
        }
        for  len in 2..=n{
            for i in 0..=n-len{
                for j in 0..=n-len{
                    for k in 1..len{
                        if dp[i][j][k] && dp[i+k][j+k][len-k]{
                            dp[i][j][len]=true;
                            break
                        }
                        if dp[i][j+len-k][k] && dp[i+k][j][len-k]{
                            dp[i][j][len]=true;
                            break
                        }
                    }
                }
            }
        }
        dp[0][0][n]
    }
}