/*
 * @lc app=leetcode id=1735 lang=rust
 *
 * [1735] Count Ways to Make Array With Product
 */

// @lc code=start
impl Solution {
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        let (n, m) = (10000, 13);
        let p = 1_000_000_007;
        let mut c = vec![vec![0; m + 1]; n + m];
        c[0][0] = 1;
        for i in 1..n + m {
            c[i][0] = 1;
            for j in 1..=i.min(m) {
                c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
                if c[i][j] >= p {
                    c[i][j] -= p;
                }
            }
        }
        let mut ans = Vec::new();
        for q in queries {
            let (n, mut k) = (q[0], q[1]);
            let mut sum = 1;
            let mut i = 2;
            while i * i <= k {
                if k % i == 0 {
                    let mut y = 0;
                    while k % i == 0 {
                        y += 1;
                        k /= i;
                    }
                    sum = ((sum as i64) * (c[(n + y - 1) as usize][y as usize] as i64) % (p as i64))
                        as i32;
                }
                i += 1;
            }
            if k != 1 {
                sum = ((sum as i64) * (n as i64) % (p as i64)) as i32;
            }
            ans.push(sum);
        }
        ans
    }
}
// @lc code=end
