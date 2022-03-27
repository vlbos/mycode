/*
 * @lc app=leetcode id=1947 lang=rust
 *
 * [1947] Maximum Compatibility Score Sum
 */

// @lc code=start
impl Solution {
    pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (students.len(), students[0].len());
        let mut g = vec![vec![0; m]; m];
        for i in 0..m {
            for j in 0..m {
                for k in 0..n {
                    if students[i][k] == mentors[j][k] {
                        g[i][j] += 1;
                    }
                }
            }
        }
        let m1 = 1 << m;
        let mut f = vec![0; m1];
        for mask in 1..m1 {
            let c = mask.count_ones() as usize;
            for i in 0..m {
                if mask & (1 << i) > 0 {
                    f[mask] = f[mask].max(f[mask ^ (1 << i)] + g[c - 1][i]);
                }
            }
        }
        f[m1 - 1]
    }
}
// @lc code=end
