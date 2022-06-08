/*
 * @lc app=leetcode id=1074 lang=rust
 *
 * [1074] Number of Submatrices That Sum to Target
 */

// @lc code=start
impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let mut ans = 0;
        let (m, n) = (matrix.len(), matrix[0].len());
        let subarray_sum = |nums: &Vec<i32>| -> i32 {
            let mut m = std::collections::HashMap::new();
            m.insert(0, 1);
            let mut ans = 0;
            let mut pre = 0;
            for &num in nums {
                pre += num;
                if let Some(&cnt) = m.get(&(pre - target)) {
                    ans += cnt;
                }
                m.entry(pre).and_modify(|x| *x += 1).or_insert(1);
            }
            ans
        };
        for i in 0..m {
            let mut sum = vec![0; n];
            for j in i..m {
                for c in 0..n {
                    sum[c] += matrix[j][c];
                }
                ans += subarray_sum(&sum);
            }
        }
        ans
    }
}
// @lc code=end
