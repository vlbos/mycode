/*
 * @lc app=leetcode id=363 lang=rust
 *
 * [363] Max Sum of Rectangle No Larger Than K
 */

// @lc code=start
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = i32::MIN;
        let (m, n) = (matrix.len(), matrix[0].len());
        for i in 0..m {
            let mut sum = vec![0; n];
            for j in i..m {
                for c in 0..n {
                    sum[c] += matrix[j][c];
                }
                let mut sum_set = std::collections::BTreeSet::from([0]);
                let mut s = 0;
                for &v in &sum {
                    s += v;
                    if let Some(lb)  = sum_set.range(s - k..).next() {
                        ans = ans.max(s - lb);
                    }
                    sum_set.insert(s);
                }
            }
        }
        ans
    }
}
// @lc code=end
