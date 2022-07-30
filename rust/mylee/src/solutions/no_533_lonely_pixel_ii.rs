// 533\. Lonely Pixel II
// =====================

// Given a picture consisting of black and white pixels, and a positive integer N,
// find the number of black pixels located at some specific row **R** and column **C** that align with all the following rules:

// 1.  Row R and column C both contain exactly N black pixels.
// 2.  For all rows that have a black pixel at column C, they should be exactly the same as row R

// The picture is represented by a 2D char array consisting of 'B' and 'W', which means black and white pixels respectively.

// **Example:**

// **Input:**
// \[\['W', 'B', 'W', 'B', 'B', 'W'\],
//  \['W', 'B', 'W', 'B', 'B', 'W'\],
//  \['W', 'B', 'W', 'B', 'B', 'W'\],
//  \['W', 'W', 'B', 'W', 'B', 'W'\]\]

// N = 3
// **Output:** 6
// **Explanation:** All the bold 'B' are the black pixels we need (all 'B's at column 1 and 3).
//         0    1    2    3    4    5         column index
// 0    \[\['W', **'B'**, 'W', **'B'**, 'B', 'W'\],
// 1     \['W', **'B'**, 'W', **'B'**, 'B', 'W'\],
// 2     \['W', **'B'**, 'W', **'B'**, 'B', 'W'\],
// 3     \['W', 'W', 'B', 'W', 'B', 'W'\]\]
// row index

// Take 'B' at row R = 0 and column C = 1 as an example:
// Rule 1, row R = 0 and column C = 1 both have exactly N = 3 black pixels.
// Rule 2, the rows have black pixel at column C = 1 are row 0, row 1 and row 2. They are exactly the same as row R = 0.

// **Note:**

// 1.  The range of width and height of the input 2D array is \[1,200\].

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
// use std::collections::HashMap;

impl Solution {
    pub fn find_black_pixel(picture: Vec<Vec<char>>, n: i32) -> i32 {
        // let rows = picture.len();
        // let cols = if rows == 0 { 0 } else { picture[0].len() };
        // if rows == 0 || cols == 0 {
        //     return 0;
        // };
        // let mut row_memo = HashMap::<Vec<char>, usize>::new();
        // let mut row_b = vec![0; rows];
        // let mut col_b = vec![0; cols];
        // for i in 0..rows {
        //     for j in 0..cols {
        //         if picture[i][j] == 'B' {
        //             col_b[j] += 1;
        //             row_b[i] += 1;
        //         }
        //     }
        //     if row_b[i] == n {
        //         row_memo
        //             .entry(picture[i].clone())
        //             .and_modify(|v| *v += 1)
        //             .or_insert_with(|| 1);
        //     }
        // }
        // let mut sum = 0;
        // for (k, v) in row_memo.into_iter() {
        //     if v == (n as usize) {
        //         for j in 0..cols {
        //             if k[j] == 'B' && col_b[j] == n {
        //                 sum += n;
        //             }
        //         }
        //     }
        // }
        // sum
        let nn = n;
        let (m, n) = (picture.len(), picture[0].len());
        let (mut rows, mut cols) = (vec![0; m], vec![0; n]);
        picture.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &v)| {
                if v == 'B' {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            })
        });
        let mut invalid_col: Vec<bool> = cols.iter().map(|x| *x != nn).collect();
        for j in 0..n {
            let mut same_row = String::new();
            for i in 0..m {
                if picture[i][j] == 'W' {
                    continue;
                }
                if rows[i] != nn {
                    invalid_col[j] = true;
                    break;
                }
                if same_row.is_empty() {
                    same_row = picture[i].iter().cloned().collect();
                    continue;
                }
                if same_row != picture[i].iter().cloned().collect::<String>() {
                    invalid_col[j] = true;
                    break;
                }
            }
        }
        invalid_col.iter().filter(|x| !*x).count() as i32 * nn
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_black_pixel_1() {
        let p = vec![
            vec!['W', 'B', 'W', 'B', 'B', 'W'],
            vec!['W', 'B', 'W', 'B', 'B', 'W'],
            vec!['W', 'B', 'W', 'B', 'B', 'W'],
            vec!['W', 'W', 'B', 'W', 'B', 'W'],
        ];
        assert_eq!(Solution::find_black_pixel(p, 3), 6);
    }
}
