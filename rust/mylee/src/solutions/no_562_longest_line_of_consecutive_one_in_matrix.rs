// 562\. Longest Line of Consecutive One in Matrix
// ===============================================

// Given a 01 matrix **M**, find the longest line of consecutive one in the matrix. The line could be horizontal, vertical, diagonal or anti-diagonal.

// **Example:**

// **Input:**
// \[\[0,1,1,0\],
//  \[0,1,1,0\],
//  \[0,0,0,1\]\]
// **Output:** 3

// **Hint:** The number of elements in the given matrix will not exceed 10,000.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
impl Solution {
    pub fn longest_line(m: Vec<Vec<i32>>) -> i32 {
        // let rows = m.len();
        // let cols = if rows == 0 { 0 } else { m[0].len() };
        // let mut dp_vertical = vec![vec![0; cols]; 2];
        // let mut dp_main_cross = vec![vec![0; cols]; 2];
        // let mut dp_sub_cross = vec![vec![0; cols]; 2];
        // let mut max_line = 0;
        // for i in 0..rows {
        //     let mut dp_horizontal = vec![0; 2];
        //     for j in 0..cols {
        //         let is_one = m[i][j] == 1;
        //         let mod_i = i % 2;
        //         let mod_j = j % 2;
        //         let mod_prev_i = (i + 1) % 2;
        //         let mod_prev_j = (j + 1) % 2;
        //         dp_vertical[mod_i][j] = if is_one {
        //             1 + dp_vertical[mod_prev_i][j]
        //         } else {
        //             0
        //         };
        //         dp_horizontal[mod_j] = if is_one {
        //             1 + dp_horizontal[mod_prev_j]
        //         } else {
        //             0
        //         };
        //         dp_main_cross[mod_i][j] = if is_one {
        //             1 + if j > 0 {
        //                 dp_main_cross[mod_prev_i][j - 1]
        //             } else {
        //                 0
        //             }
        //         } else {
        //             0
        //         };
        //         dp_sub_cross[mod_i][j] = if is_one {
        //             1 + if (j + 1) < cols {
        //                 dp_sub_cross[mod_prev_i][j + 1]
        //             } else {
        //                 0
        //             }
        //         } else {
        //             0
        //         };
        //         max_line = i32::max(
        //             max_line,
        //             i32::max(
        //                 i32::max(dp_vertical[mod_i][j], dp_horizontal[mod_j]),
        //                 i32::max(dp_main_cross[mod_i][j], dp_sub_cross[mod_i][j]),
        //             ),
        //         )
        //     }
        // }
        // max_line
        let mut ans = 0;
        let dirs = [0, 1, 0, -1, 0];

        for (i, row) in m.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v == 0 {
                    continue;
                }
                for (dx, dy) in [(1, 0), (0, 1), (-1, -1), (-1, 1)] {
                    let mut cnt = 0;
                    let (mut x, mut y) = (i as i32, j as i32);
                    while x >= 0
                        && x < m.len() as i32
                        && y >= 0
                        && y < row.len() as i32
                        && m[x as usize][y as usize] == 1
                    {
                        x += dx;
                        y += dy;
                        cnt += 1;
                    }
                    ans = ans.max(cnt);
                }
            }
        }
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub  struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_line_1() {
        assert_eq!(
            Solution::longest_line(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 1]]),
            3
        );
    }

    #[test]
    fn test_longest_line_2() {
        assert_eq!(
            Solution::longest_line(vec![
                vec![0, 1, 0, 1, 1],
                vec![1, 1, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
                vec![1, 0, 1, 1, 1],
                vec![1, 0, 0, 0, 1]
            ]),
            3
        )
    }
}
