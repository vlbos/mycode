// 750\. Number Of Corner Rectangles
// =================================

// Given a grid where each entry is only 0 or 1, find the number of corner rectangles.

// A _corner rectangle_ is 4 distinct 1s on the grid that form an axis-aligned rectangle.
// Note that only the corners need to have the value 1. Also, all four 1s used must be distinct.

// **Example 1:**

// **Input:** grid =
// \[\[1, 0, 0, 1, 0\],
//  \[0, 0, 1, 0, 1\],
//  \[0, 0, 0, 1, 0\],
//  \[1, 0, 1, 0, 1\]\]
// **Output:** 1
// **Explanation:** There is only one corner rectangle, with corners grid\[1\]\[2\], grid\[1\]\[4\], grid\[3\]\[2\], grid\[3\]\[4\].

// **Example 2:**

// **Input:** grid =
// \[\[1, 1, 1\],
//  \[1, 1, 1\],
//  \[1, 1, 1\]\]
// **Output:** 9
// **Explanation:** There are four 2x2 rectangles, four 2x3 and 3x2 rectangles, and one 3x3 rectangle.

// **Example 3:**

// **Input:** grid =
// \[\[1, 1, 1, 1\]\]
// **Output:** 0
// **Explanation:** Rectangles must have four distinct corners.

// **Note:**

// 1.  The number of rows and columns of `grid` will each be in the range `[1, 200]`.
// 2.  Each `grid[i][j]` will be either `0` or `1`.
// 3.  The number of `1`s in the grid will be at most `6000`.

// @lc code=start
// use std::collections::HashMap;

impl Solution {
    pub fn count_corner_rectangles(grid: Vec<Vec<i32>>) -> i32 {
        // let rows = grid.len();
        // let cols = if rows == 0 { 0 } else { grid[0].len() };
        // if rows * cols == 0 {
        //     return 0;
        // }
        // let mut dict = HashMap::<(usize, usize), usize>::new();
        // for i in 0..rows {
        //     let mut exists = grid[i]
        //         .iter()
        //         .enumerate()
        //         .filter(|(_, &v)| v != 0)
        //         .map(|(j, _)| j)
        //         .collect::<Vec<_>>();
        //     let e_len = exists.len();
        //     if e_len <= 1 {
        //         continue;
        //     }
        //     exists.sort();
        //     for i in 0..e_len {
        //         for j in (i + 1)..e_len {
        //             dict.entry((exists[i], exists[j]))
        //                 .and_modify(|v| *v += 1)
        //                 .or_insert(1);
        //         }
        //     }
        // }
        // dict.into_iter()
        //     .map(|(_, v)| v * (v - 1) / 2)
        //     .fold(0usize, |acc, curr| acc + curr) as i32
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = 0;
        for i in 0..m {
            for j in i + 1..m {
                let mut cnt = 0;
                for k in 0..n {
                    if grid[i][k] == 1 && grid[j][k] == 1 {
                        cnt += 1;
                    }
                }
                ans += cnt * (cnt - 1) / 2;
            }
        }
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;

    #[test]
    fn test_count_corner_rectangles_1() {
        let grid = lc_matrix![
            [1, 0, 0, 1, 0],
            [0, 0, 1, 0, 1],
            [0, 0, 0, 1, 0],
            [1, 0, 1, 0, 1]
        ];
        assert_eq!(Solution::count_corner_rectangles(grid), 1);
    }

    #[test]
    fn test_count_corner_rectangles_2() {
        let grid = lc_matrix![[1, 1, 1], [1, 1, 1], [1, 1, 1]];
        assert_eq!(Solution::count_corner_rectangles(grid), 9);
    }

    #[test]
    fn test_count_corner_rectangles_3() {
        let grid = lc_matrix![[1, 1, 1, 1]];
        assert_eq!(Solution::count_corner_rectangles(grid), 0);
    }
}
