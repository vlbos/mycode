// 694\. Number of Distinct Islands
// ================================

// Given a non-empty 2D array `grid` of 0's and 1's, an **island** is a group of `1`'s (representing land) connected 4-directionally (horizontal or vertical.)
// You may assume all four edges of the grid are surrounded by water.

// Count the number of **distinct** islands.
// An island is considered to be the same as another if and only if one island can be translated (and not rotated or reflected) to equal the other.

// **Example 1:**

// 11000
// 11000
// 00011
// 00011

// Given the above grid map, return `1`.

// **Example 2:**

// 11011
// 10000
// 00001
// 11011

// Given the above grid map, return `3`.

// Notice that:

// 11
// 1

// and

//  1
// 11

// are considered different island shapes, because we do not consider reflection / rotation.

// **Note:** The length of each dimension in the given `grid` does not exceed 50.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Lyft](https://leetcode.ca/tags/#Lyft) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
// use std::collections::{HashMap, HashSet};

// pub  struct UnionFind {
//     sz: Vec<usize>,
//     id: Vec<usize>,
//     count: usize,
// }

// impl UnionFind {
//     pub fn new(size: usize) -> Self {
//         Self {
//             sz: vec![0; size],
//             id: (0..size).collect(),
//             count: size,
//         }
//     }

//     pub fn find(&self, mut p: usize) -> usize {
//         while self.id[p] != p {
//             p = self.id[p];
//         }
//         p
//     }

//     pub fn connected(&self, p: usize, q: usize) -> bool {
//         let pid = self.find(p);
//         let qid = self.find(q);
//         pid == qid
//     }

//     pub fn union(&mut self, p: usize, q: usize) {
//         let pid = self.find(p);
//         let qid = self.find(q);
//         if pid == qid {
//             return;
//         }
//         if self.sz[pid] > self.sz[qid] {
//             self.id[qid] = self.id[pid];
//         } else {
//             self.id[pid] = self.id[qid];
//         }
//     }
// }

impl Solution {
    pub fn num_distinct_islands(grid: Vec<Vec<i32>>) -> i32 {
        // let rows = grid.len();
        // let cols = if rows == 0 { 0 } else { grid[0].len() };
        // if rows * cols == 0 {
        //     return 0;
        // }
        // let mut uf = UnionFind::new(rows * cols);
        // for i in 0..rows {
        //     for j in 0..cols {
        //         if grid[i][j] == 1 {
        //             let k = i * cols + j;
        //             if i > 0 && grid[i - 1][j] == 1 {
        //                 let k_up = k - cols;
        //                 uf.union(k, k_up);
        //             }
        //             if j > 0 && grid[i][j - 1] == 1 {
        //                 let k_left = k - 1;
        //                 uf.union(k, k_left);
        //             }
        //         }
        //     }
        // }
        // let mut islands = HashMap::<usize, Vec<i32>>::new();
        // for i in 0..rows * cols {
        //     let r = i / cols;
        //     let c = i % cols;
        //     if grid[r][c] == 1 {
        //         let id = uf.find(i);
        //         let relative = (i as i32) - (id as i32);
        //         islands
        //             .entry(id)
        //             .and_modify(|v| v.push(relative))
        //             .or_insert_with(|| vec![relative]);
        //     }
        // }
        // islands
        //     .into_iter()
        //     .map(|(_, v)| v)
        //     .collect::<HashSet<_>>()
        //     .len() as i32
        use std::collections::HashSet;
        fn dfs(
            i: usize,
            j: usize,
            grid: &Vec<Vec<i32>>,
            visited: &mut HashSet<(usize, usize)>,
            island: &mut Vec<Vec<i32>>,
        ) {
            visited.insert((i, j));
            let dirs = [0, 1, 0, -1, 0];
            let (m, n) = (grid.len() as i32, grid[0].len() as i32);
            for (k, &d) in dirs[1..].iter().enumerate() {
                let (x, y) = (i as i32 + dirs[k], j as i32 + d);
                if x < 0 || x >= m || y < 0 || y >= n {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if grid[x][y] == 1 && !visited.contains(&(x, y)) {
                    island.push(vec![dirs[k], d]);
                    dfs(x, y, grid, visited, island);
                }
            }
        }
        let mut ans = 0;
        let mut visited = HashSet::new();
        let mut islands = HashSet::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v == 1 && !visited.contains(&(i, j)) {
                    let mut island = Vec::new();
                    dfs(i, j, &grid, &mut visited, &mut island);
                    if !islands.contains(&island) {
                        ans += 1;
                        islands.insert(island);
                    }
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
    use crate::lc_matrix;

    #[test]
    fn test_num_distinct_islands_1() {
        let grid = lc_matrix![
            [1, 1, 0, 0, 0],
            [1, 1, 0, 0, 0],
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1]
        ];
        assert_eq!(Solution::num_distinct_islands(grid), 1);
    }

    #[test]
    fn test_num_distinct_islands_2() {
        let grid = lc_matrix![
            [1, 1, 0, 1, 1],
            [1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1],
            [1, 1, 0, 1, 1]
        ];
        assert_eq!(Solution::num_distinct_islands(grid), 3);
    }
}
