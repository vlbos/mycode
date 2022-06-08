/*
 * @lc app=leetcode id=1072 lang=rust
 *
 * [1072] Flip Columns For Maximum Number of Equal Rows
 */

// @lc code=start
impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let ans = matrix
            .iter()
            .map(|x| {
                x.iter()
                    .map(|&y| {
                        if x[0] == 0 {
                            (y ^ 1).to_string()
                        } else {
                            y.to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .concat()
            })
            .collect::<Vec<String>>()
            .iter()
            .cloned()
            .fold(std::collections::HashMap::new(), |mut m, z| {
                *m.entry(z).or_insert(0) += 1;
                m
            });

        *ans.iter().map(|x| x.1).max().unwrap()
    }
}
// @lc code=end
