// 317\. Shortest Distance from All Buildings
// ==========================================

// You want to build a house on an _empty_ land which reaches all buildings in the shortest amount of distance.
// You can only move up, down, left and right. You are given a 2D grid of values **0**, **1** or **2**, where:

// *   Each **0** marks an empty land which you can pass by freely.
// *   Each **1** marks a building which you cannot pass through.
// *   Each **2** marks an obstacle which you cannot pass through.

// **Example:**

// **Input:** \[\[1,0,2,0,1\],\[0,0,0,0,0\],\[0,0,1,0,0\]\]

// 1 - 0 - 2 - 0 - 1
// |   |   |   |   |
// 0 - 0 - 0 - 0 - 0
// |   |   |   |   |
// 0 - 0 - 1 - 0 - 0

// **Output:** 7

// **Explanation:** Given three buildings at `(0,0)`, `(0,4)`, `(2,2)`, and an obstacle at `(0,2),
//              t`he point `(1,2)` is an ideal empty land to build a house, as the total
//              travel distance of 3+3+1=7 is minimal. So return 7.

// **Note:**
// There will be at least one building. If it is not possible to build such house according to the above rules, return -1.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [ByteDance](https://leetcode.ca/tags/#ByteDance) [Facebook](https://leetcode.ca/tags/#Facebook) [Goldman Sachs](https://leetcode.ca/tags/#Goldman%20Sachs) [Google](https://leetcode.ca/tags/#Google) [Mathworks](https://leetcode.ca/tags/#Mathworks) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Snapchat](https://leetcode.ca/tags/#Snapchat) [Splunk](https://leetcode.ca/tags/#Splunk) [Uber](https://leetcode.ca/tags/#Uber) [Zenefits](https://leetcode.ca/tags/#Zenefits)

// @lc code=start

// const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    pub fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
        // let rows = grid.len();
        // let cols = if rows == 0 { 0 } else { grid[0].len() };
        // if rows == 0 || cols == 0 {
        //     return -1;
        // }
        // let mut accessible = vec![vec![0; cols]; rows];
        // let mut cost = vec![vec![0; cols]; rows];

        // let mut buildings = 0;
        // for i in 0..rows {
        //     for j in 0..cols {
        //         if grid[i][j] == 1 {
        //             let mut deque = VecDeque::<(isize, isize, i32)>::new();
        //             deque.push_back((i as isize, j as isize, 0));
        //             accessible[i][j] = buildings;
        //             while let Some((x, y, c)) = deque.pop_front() {
        //                 let acc = accessible[x as usize][y as usize];
        //                 accessible[x as usize][y as usize] = if acc == buildings {
        //                     for (dx, dy) in &DELTAS {
        //                         let nx = x + dx;
        //                         let ny = y + dy;
        //                         if nx < 0
        //                             || ny < 0
        //                             || nx >= rows as isize
        //                             || ny >= cols as isize
        //                             || grid[nx as usize][ny as usize] != 0
        //                         {
        //                             continue;
        //                         }
        //                         deque.push_back((nx, ny, c + 1));
        //                     }
        //                     cost[x as usize][y as usize] += c;
        //                     buildings + 1
        //                 } else if acc > buildings {
        //                     acc
        //                 } else {
        //                     -1
        //                 }
        //             }
        //             buildings += 1;
        //         }
        //     }
        // }
        // let mut md = i32::max_value();
        // for i in 0..rows {
        //     for j in 0..cols {
        //         md = if accessible[i][j] == buildings && grid[i][j] == 0 {
        //             i32::min(cost[i][j], md)
        //         } else {
        //             md
        //         };
        //     }
        // }
        // if md == i32::max_value() {
        //     -1
        // } else {
        //     md
        // }
        let (m, n) = (grid.len(), grid[0].len());
        let mut record1 = vec![vec![0; n]; m];
        let mut record2 = vec![vec![0; n]; m];
        let mut num1 = 0;
        let dirs = [0, 1, 0, -1, 0];
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 1 {
                    continue;
                }
                num1 += 1;
                let mut q = std::collections::VecDeque::from([(i, j)]);
                let mut visited = vec![vec![false; n]; m];
                visited[i][j] = true;
                let mut distance = 0;
                while !q.is_empty() {
                    let len = q.len();
                    for _ in 0..len {
                        let (x, y) = q.pop_front().unwrap();
                        record2[x][y] += distance;
                        record1[x][y] += 1;
                        for (k, d) in dirs[1..].iter().enumerate() {
                            let (nx, ny) = (x as i32 + dirs[k], y as i32 + d);
                            if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                                continue;
                            }
                            let (nx, ny) = (nx as usize, ny as usize);
                            if !visited[nx][ny] && grid[nx][ny] == 0 {
                                q.push_back((nx, ny));
                                visited[nx][ny] = true;
                            }
                        }
                    }
                    distance += 1;
                }
            }
        }
        let mut ans = i32::MAX;
        for (i, r) in grid.iter().enumerate() {
            for (j, &v) in r.iter().enumerate() {
                if v == 0 && record1[i][j] == num1 && record2[i][j] < ans {
                    ans = record2[i][j];
                }
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
// @lc code=end

#[allow(dead_code)]
pub  struct  Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shortest_distance() {
        let grid = vec![
            vec![1, 0, 2, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
        ];
        assert_eq!(Solution::shortest_distance(grid), 7);
    }
}
