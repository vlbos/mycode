/*
 * @lc app=leetcode id=1977 lang=rust
 *
 * [1977] Number of Ways to Separate Numbers
 */

// @lc code=start
impl Solution {
    pub fn number_of_combinations(num: String) -> i32 {
        let bnum = num.as_bytes();
        if bnum[0] == b'0' {
            return 0;
        }
        let n = num.len();
        let mut lcp = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            if bnum[i] == bnum[n - 1] {
                lcp[i][n - 1] = 1;
            }
            for j in i + 1..n - 1 {
                lcp[i][j] = if bnum[i] == bnum[j] {
                    lcp[i + 1][j + 1] + 1
                } else {
                    0
                };
            }
        }
        let p = 1_000_000_007;
        let update = |x: &mut i32, y: i32| {
            *x += y;
            if *x >= p {
                *x -= p;
            }
        };
        let mut f = vec![vec![0; n]; n];
        for i in 0..n {
            f[0][i] = 1;
        }
        for i in 1..n {
            if bnum[i] == b'0' {
                continue;
            }
            let mut presum = 0;
            for j in i..n {
                let k = j - i + 1;
                f[i][j] = presum;
                if i < k {
                    continue;
                }
                if lcp[i - k][i] >= k || bnum[i - k + lcp[i - k][i]] < bnum[i + lcp[i - k][i]] {
                    let v=f[i - k][i - 1];
                    update(&mut f[i][j], v);
                }
                update(&mut presum, f[i - k][i - 1]);
            }
        }
        let mut ans = 0;
        for i in 0..n {
            update(&mut ans, f[i][n - 1]);
        }
        ans
    }
}
// @lc code=end
