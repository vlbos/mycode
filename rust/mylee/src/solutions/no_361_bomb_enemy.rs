// 361\. Bomb Enemy
// ================

// Given a 2D grid, each cell is either a wall `'W'`, an enemy `'E'` or empty `'0'` (the number zero),
//  return the maximum enemies you can kill using one bomb.
// The bomb kills all the enemies in the same row and column from the planted point until it hits the wall
// since the wall is too strong to be destroyed.
// **Note:** You can only put the bomb at an empty cell.

// **Example:**

// **Input:** \[\["0","E","0","0"\],\["E","0","W","E"\],\["0","E","0","0"\]\]
// **Output:** 3
// **Explanation:** For the given grid,

// 0 E 0 0
// E 0 W E
// 0 E 0 0

// Placing a bomb at (1,1) kills 3 enemies.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
// const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn   max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {
        // let rows = grid.len();
        // let cols = if rows == 0 { 0 } else { grid[0].len() };
        // let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0usize; cols + 2]; rows + 2]; 4];
        // for k in 0..4 {
        //     let (di, dj) = DIRECTIONS[k];
        //     let is: Vec<usize> = if di > 0 {
        //         (0..rows).rev().collect()
        //     } else {
        //         (0..rows).collect()
        //     };
        //     let js: Vec<usize> = if dj > 0 {
        //         (0..cols).rev().collect()
        //     } else {
        //         (0..cols).collect()
        //     };
        //     for &i in &is {
        //         for &j in &js {
        //             let ti = i + 1;
        //             let tj = j + 1;
        //             let ni = (ti as isize + di) as usize;
        //             let nj = (tj as isize + dj) as usize;
        //             dp[k][ti][tj] = match grid[i][j] {
        //                 'W' => 0,
        //                 'E' => dp[k][ni][nj] + 1,
        //                 _ => dp[k][ni][nj],
        //             }
        //         }
        //     }
        // }
        // let mut max = 0;
        // for i in 0..rows {
        //     for j in 0..cols {
        //         if grid[i][j] == '0' {
        //             let ti = i + 1;
        //             let tj = j + 1;
        //             max = i32::max(
        //                 max,
        //                 (dp[0][ti][tj] + dp[1][ti][tj] + dp[2][ti][tj] + dp[3][ti][tj]) as i32,
        //             );
        //         }
        //     }
        // }
        // max
        let (m, n) = (grid.len(), grid[0].len());
        let mut column = vec![0; n];
        let mut ans = 0;
        let mut row = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 'W' {
                    continue;
                }
                if j == 0 || grid[i][j - 1] == 'W' {
                    row = 0;
                    for k in j..n {
                        if grid[i][k] == 'W' {
                            break;
                        }
                        if grid[i][k] == 'E' {
                            row += 1;
                        }
                    }
                }
                if i == 0 || grid[i - 1][j] == 'W' {
                    column[j] = 0;
                    for k in i..m {
                        if grid[k][j] == 'W' {
                            break;
                        }
                        if grid[k][j] == 'E' {
                            column[j] += 1;
                        }
                    }
                }
                if grid[i][j] == '0' {
                    ans = ans.max(row + column[j]);
                }
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

    #[test]
   pub fn  test_bomb_enemy() {
        let matrix = vec![
            vec!['0', 'E', '0', '0'],
            vec!['E', '0', 'W', 'E'],
            vec!['0', 'E', '0', '0'],
        ];
        assert_eq!(Solution::max_killed_enemies(matrix), 3);
    }
}
