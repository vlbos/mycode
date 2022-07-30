// 311\. Sparse Matrix Multiplication
// ==================================

// Given two [sparse matrices](https://en.wikipedia.org/wiki/Sparse_matrix) **A** and **B**, return the result of **AB**.

// You may assume that **A**'s column number is equal to **B**'s row number.

// **Example:**

// **Input:** **A** = \[
//   \[ 1, 0, 0\],
//   \[-1, 0, 3\]
// \]

// **B** = \[
//   \[ 7, 0, 0 \],
//   \[ 0, 0, 0 \],
//   \[ 0, 0, 1 \]
// \]

// **Output:**

//      |  1 0 0 |   | 7 0 0 |   |  7 0 0 |
// **AB** = | -1 0 3 | x | 0 0 0 | = | -7 0 3 |
//                   | 0 0 1 |

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Facebook](https://leetcode.ca/tags/#Facebook) [Goldman Sachs](https://leetcode.ca/tags/#Goldman%20Sachs) [Google](https://leetcode.ca/tags/#Google) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Snapchat](https://leetcode.ca/tags/#Snapchat)
// @lc code=start
// use std::collections::hash_map::HashMap;

// enum MatrixAxis {
//     ROW,
//     COL,
// }

impl Solution {
    pub fn multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // let a_rows = a.len();
        // let multi_lines = b.len();
        // let b_cols = if multi_lines == 0 { 0 } else { b[0].len() };
        // let a_col_numbers = Solution::build_axis_numbers_from_matrix(a, MatrixAxis::COL);
        // println!("{:?}", a_col_numbers);
        // let b_row_numbers = Solution::build_axis_numbers_from_matrix(b, MatrixAxis::ROW);
        // if a_rows == 0 || multi_lines == 0 || b_cols == 0 {
        //     return vec![];
        // }
        // let mut res = vec![vec![0; b_cols]; a_rows];
        // for k in 0..multi_lines {
        //     if !a_col_numbers.contains_key(&k) || !b_row_numbers.contains_key(&k) {
        //         continue;
        //     }
        //     for (&i, &av) in &a_col_numbers[&k] {
        //         for (&j, &bv) in &b_row_numbers[&k] {
        //             res[i][j] += av * bv;
        //         }
        //     }
        // }
        // res
        let (m, n) = (a.len(), a[0].len());
        let nb = b[0].len();
        let mut ans = vec![vec![0; nb]; m];
        for i in 0..m {
            for k in 0..n {
                if a[i][k] == 0 {
                    continue;
                }
                for j in 0..nb {
                    if b[k][j] != 0 {
                        ans[i][j] += a[i][k] * b[k][j];
                    }
                }
            }
        }
        ans
    }

    // fn build_axis_numbers_from_matrix(
    //     matrix: Vec<Vec<i32>>,
    //     axis: MatrixAxis,
    // ) -> HashMap<usize, HashMap<usize, i32>> {
    //     match axis {
    //         MatrixAxis::ROW => matrix
    //             .into_iter()
    //             .enumerate()
    //             .map(|(i, r)| {
    //                 (
    //                     i,
    //                     r.into_iter()
    //                         .enumerate()
    //                         .filter(|(_, v)| *v != 0)
    //                         .collect::<HashMap<usize, i32>>(),
    //                 )
    //             })
    //             .filter(|(_, v)| !v.is_empty())
    //             .collect::<HashMap<usize, HashMap<usize, i32>>>(),
    //         MatrixAxis::COL => {
    //             let rows = matrix.len();
    //             let cols = if rows == 0 { 0 } else { matrix[0].len() };
    //             (0..cols)
    //                 .into_iter()
    //                 .map(|j| {
    //                     (
    //                         j,
    //                         (0..rows)
    //                             .into_iter()
    //                             .map(|i| matrix[i][j])
    //                             .enumerate()
    //                             .filter(|(_, v)| *v != 0)
    //                             .collect::<HashMap<usize, i32>>(),
    //                     )
    //                 })
    //                 .filter(|(_, v)| !v.is_empty())
    //                 .collect::<HashMap<usize, HashMap<usize, i32>>>()
    //         }
    //     }
    // }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = vec![vec![1, 0, 0], vec![-1, 0, 3]];
        let b = vec![vec![7, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];
        assert_eq!(
            Solution::multiply(a, b),
            vec![vec![7, 0, 0], vec![-7, 0, 3]]
        );
    }
}
