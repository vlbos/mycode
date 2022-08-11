// 711\..
// ===================================

// Given a non-empty 2D array `grid` of 0's and 1's, an **island** is a group of `1`'s (representing land) connected 4-directionally (horizontal or vertical.)
// You may assume all four edges of the grid are surrounded by water.

// Count the number of **distinct** islands. An island is considered to be the same as another if they have the same shape,
//  or have the same shape after **rotation** (90, 180, or 270 degrees only) or **reflection** (left/right direction or up/down direction).

// **Example 1:**

// 11000
// 10000
// 00001
// 00011

// Given the above grid map, return `1`.

// Notice that:

// 11
// 1

// and

//  1
// 11

// are considered **same** island shapes. Because if we make a 180 degrees clockwise rotation on the first island, then two islands will have the same shapes.

// **Example 2:**

// 11100
// 10001
// 01001
// 01110

// Given the above grid map, return `2`.

// Here are the two distinct islands:

// 111
// 1

// and

// 1
// 1

// Notice that:

// 111
// 1

// and

// 1
// 111

// are considered **same** island shapes. Because if we flip the first array in the up/down direction, then they have the same shapes.

// **Note:** The length of each dimension in the given `grid` does not exceed 50.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
// use std::collections::{HashMap, HashSet};

// const TRANS_MATS: &[[[i32; 2]; 2]; 8] = &[
//     [[1, 0], [0, 1]],
//     [[1, 0], [0, -1]],
//     [[-1, 0], [0, 1]],
//     [[-1, 0], [0, -1]],
//     [[0, 1], [-1, 0]],
//     [[0, 1], [1, 0]],
//     [[0, -1], [1, 0]],
//     [[0, -1], [-1, 0]],
// ];

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
    pub fn num_distinct_islands2(grid: Vec<Vec<i32>>) -> i32 {
        // let rows = grid.len();
        // let cols = if rows == 0 { 0 } else { grid[0].len() };
        // let size = rows * cols;
        // if size == 0 {
        //     return 0;
        // }
        // let mut uf = UnionFind::new(size);
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
        // let mut islands = HashMap::<usize, Vec<(usize, usize)>>::new();
        // for i in 0..size {
        //     let x = i % cols;
        //     let y = i / cols;
        //     if grid[y][x] == 1 {
        //         let id = uf.find(i);
        //         islands
        //             .entry(id)
        //             .and_modify(|v| v.push((x, y)))
        //             .or_insert_with(|| vec![(x, y)]);
        //     }
        // }
        // let mut unique = HashSet::<Vec<i32>>::new();
        // 'outer: for (_, points) in &islands {
        //     let mut ident = vec![];
        //     for tm in TRANS_MATS {
        //         let mut trans_points = points
        //             .iter()
        //             .map(|source| Solution::trans(source.0 as i32, source.1 as i32, tm))
        //             .collect::<Vec<_>>();
        //         trans_points.sort();
        //         let min = trans_points[0];
        //         ident = trans_points
        //             .into_iter()
        //             .map(|(x, y)| (y - min.1) * (cols as i32) + (x - min.0))
        //             .collect();
        //         if unique.contains(&ident) {
        //             continue 'outer;
        //         }
        //     }
        //     unique.insert(ident);
        // }
        // unique.len() as i32
        use std::collections::HashSet;
        fn dfs(
            i: usize,
            j: usize,
            grid: &Vec<Vec<i32>>,
            visited: &mut HashSet<(usize, usize)>,
            island: &mut Vec<Vec<i32>>,
        ) {
            visited.insert((i, j));
            island.push(vec![i as i32, j as i32]);
            let dirs = [0, 1, 0, -1, 0];
            let (m, n) = (grid.len() as i32, grid[0].len() as i32);
            for (k, &d) in dirs[1..].iter().enumerate() {
                let (x, y) = (i as i32 + dirs[k], j as i32 + d);
                if x < 0 || x >= m || y < 0 || y >= n {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if grid[x][y] == 1 && !visited.contains(&(x, y)) {
                    dfs(x, y, grid, visited, island);
                }
            }
        }
        let normalize = |island: Vec<Vec<i32>>| {
            let mut shapes: Vec<Vec<Vec<i32>>> = Vec::new();
            for i in [-1, 1] {
                for j in [-1, 1] {
                    shapes.push(island.iter().map(|a| vec![a[0] * i, a[1] * j]).collect());
                    shapes.push(island.iter().map(|a| vec![a[1] * i, a[0] * j]).collect());
                }
            }

            for i in 0..shapes.len() {
                shapes[i].sort();
                for j in (0..shapes[i].len()).rev() {
                    shapes[i][j][0] -= shapes[i][0][0];
                    shapes[i][j][1] -= shapes[i][0][1];
                }
            }
            shapes.sort();
            shapes[0].clone()
        };
        let mut ans = 0;
        let mut visited = HashSet::new();
        let mut islands = HashSet::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v == 1 && !visited.contains(&(i, j)) {
                    let mut island = Vec::new();
                    dfs(i, j, &grid, &mut visited, &mut island);
                    let island = normalize(island);
                    if !islands.contains(&island) {
                        ans += 1;
                        islands.insert(island);
                    }
                }
            }
        }
        ans
    }

    // #[inline]
    // fn trans(cx: i32, cy: i32, trans_mat: &[[i32; 2]; 2]) -> (i32, i32) {
    //     (
    //         cx * trans_mat[0][0] + cy * trans_mat[1][0],
    //         cx * trans_mat[0][1] + cy * trans_mat[1][1],
    //     )
    // }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;

    #[test]
    fn test_num_distinct_islands2_1() {
        let grid = lc_matrix![
            [1, 1, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 1, 1]
        ];
        assert_eq!(Solution::num_distinct_islands2(grid), 1);
    }

    #[test]
    fn test_num_distinct_islands2_2() {
        let grid = lc_matrix![
            [1, 1, 1, 0, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 0, 1],
            [0, 1, 1, 1, 0]
        ];
        assert_eq!(Solution::num_distinct_islands2(grid), 2);
    }
}
