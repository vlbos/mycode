// 1102\. Path With Maximum Minimum Value
// ======================================

// Given a matrix of integers `A` with R rows and C columns, find the **maximum** score of a path starting at `[0,0]` and ending at `[R-1,C-1]`.

// The _score_ of a path is the **minimum** value in that path.  For example, the value of the path 8 →  4 →  5 →  9 is 4.

// A _path_ moves some number of times from one visited cell to any neighbouring unvisited cell in one of the 4 cardinal directions (north, east, west, south).

// **Example 1:**

// **![](https://assets.leetcode.com/uploads/2019/04/23/1313_ex1.JPG)**

// **Input:** \[\[5,4,5\],\[1,2,6\],\[7,4,6\]\]
// **Output:** 4
// **Explanation:**
// The path with the maximum score is highlighted in yellow.

// **Example 2:**

// **![](https://assets.leetcode.com/uploads/2019/04/23/1313_ex2.JPG)**

// **Input:** \[\[2,2,1,2,2,2\],\[1,2,2,2,1,2\]\]
// **Output: 2**

// **Example 3:**

// **![](https://assets.leetcode.com/uploads/2019/04/23/1313_ex3.JPG)**

// **Input:** \[\[3,4,6,3,4\],\[0,2,1,1,7\],\[8,8,3,2,7\],\[3,2,4,9,8\],\[4,1,2,0,0\],\[4,6,5,4,3\]\]
// **Output: 3**

// **Note:**

// 1.  `1 <= R, C <= 100`
// 2.  `0 <= A[i][j] <= 10^9`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn maximum_minimum_path(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; n]; m];
        let mut q = std::collections::BinaryHeap::from([(grid[0][0], 0, 0)]);
        let dirs = [0, 1, 0, -1, 0];
        while let Some((c, i, j)) = q.pop() {
            if i == m - 1 && j == n - 1 {
                return c;
            }
            visited[i][j] = true;
            for d in dirs.windows(2) {
                let (x, y) = (i as i32 + d[0], j as i32 + d[1]);
                if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if !visited[x][y] {
                    q.push((grid[x][y].min(c), x, y));
                    visited[x][y] = true;
                }
            }
        }
        -1
    }
}

// impl Solution {
//     pub fn maximum_minimum_path(grid: Vec<Vec<i32>>) -> i32 {
//  let n = grid.len();
//         let m = grid[0].len();
//         let mut vis = vec![vec![false;m];n];
//         let dirs = [(1,0),(-1,0),(0,1),(0,-1)];
//         let mut queue = std::collections::BinaryHeap::new();
//         vis[0][0] = true;
//         queue.push((grid[0][0], 0, 0));
//         let mut ans = i32::MAX;

//         while let Some((v, i, j)) = queue.pop() {
//             ans = ans.min(v);

//             if i == n - 1 && j == m - 1 {
//                 break;
//             }

//             for k in 0..4 {
//                 let x = (i as i32 + dirs[k].0) as usize;
//                 let y = (j as i32 + dirs[k].1) as usize;

//                 if x < n && y < m && !vis[x][y] {
//                     queue.push((grid[x][y], x, y));
//                     vis[x][y] = true;
//                 }
//             }
//         }

//         ans
//     }
// }
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_maximum_minimum_path_1() {
        assert_eq!(
            4,
            Solution::maximum_minimum_path(vec![vec![5, 4, 5], vec![1, 2, 6], vec![7, 4, 6]])
        );
    }

    #[test]
    pub fn test_maximum_minimum_path_2() {
        assert_eq!(
            2,
            Solution::maximum_minimum_path(vec![vec![2, 2, 1, 2, 2, 2], vec![1, 2, 2, 2, 1, 2]])
        );
    }

    #[test]
    pub fn test_maximum_minimum_path_3() {
        assert_eq!(
            3,
            Solution::maximum_minimum_path(vec![
                vec![3, 4, 6, 3, 4],
                vec![0, 2, 1, 1, 7],
                vec![8, 8, 3, 2, 7],
                vec![3, 2, 4, 9, 8],
                vec![4, 1, 2, 0, 0],
                vec![4, 6, 5, 4, 3]
            ])
        );
    }
}
