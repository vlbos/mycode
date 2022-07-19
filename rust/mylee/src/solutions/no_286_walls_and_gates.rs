// 286\. Walls and Gates
// =====================

// You are given a _m x n_ 2D grid initialized with these three possible values.

// 1.  `-1` \- A wall or an obstacle.
// 2.  `0` \- A gate.
// 3.  `INF` \- Infinity means an empty room. We use the value `231 - 1 = 2147483647` to represent `INF` as you may assume that the distance to a gate is less than `2147483647`.

// Fill each empty room with the distance to its _nearest_ gate. If it is impossible to reach a gate, it should be filled with `INF`.

// **Example:**

// Given the 2D grid:

// INF  -1  0  INF
// INF INF INF  -1
// INF  -1 INF  -1
//   0  -1 INF INF

// After running your function, the 2D grid should be:

//   3  -1   0   1
//   2   2   1  -1
//   1  -1   2  -1
//   0  -1   3   4

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [ByteDance](https://leetcode.ca/tags/#ByteDance) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Uber](https://leetcode.ca/tags/#Uber)
// @lc code=start
// use std::collections::VecDeque;
// use std::mem::swap;

// const MOVEMENTS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const INF: i32 = 2147483647;

impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        // if rooms.is_empty() {
        //     return;
        // }
        // if rooms[0].is_empty() {
        //     return;
        // }
        // let rows = rooms.len();
        // let cols = rooms[0].len();
        // for r in 0..rows {
        //     for c in 0..cols {
        //         Solution::broadcast_update(rooms, r, c, rows, cols);
        //     }
        // }
        if rooms.is_empty() || rooms[0].is_empty() {
            return;
        }
        let (m, n) = (rooms.len(), rooms[0].len());
        let mut visited = vec![vec![false; n]; m];
        fn dfs(i: i32, j: i32, d: i32, rooms: &mut Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) {
            let (m, n) = (rooms.len() as i32, rooms[0].len() as i32);
            if i < 0 || i >= m || j < 0 || j >= n {
                return;
            }
            let (i, j) = (i as usize, j as usize);
            if visited[i][j] || rooms[i][j] == -1 || rooms[i][j] < d {
                return;
            }
            visited[i][j] = true;
            rooms[i][j] = d;
            let dirs = [0, 1, 0, -1, 0];
            for (k, &v) in dirs[1..].iter().enumerate() {
                dfs(i as i32 + dirs[k], j as i32 + v, d + 1, rooms, visited);
            }
            visited[i][j] = false;
        }
        for i in 0..m {
            for j in 0..n {
                if rooms[i][j] == 0 {
                    dfs(i as i32, j as i32, 0, rooms, &mut visited);
                }
            }
        }
    }

    // // the value of each grid tells us if it has been visited
    // fn broadcast_update(rooms: &mut Vec<Vec<i32>>, r: usize, c: usize, rows: usize, cols: usize) {
    //     if rooms[r][c] != 0 {
    //         return;
    //     }
    //     let rows = rows as isize;
    //     let cols = cols as isize;
    //     let mut curr = VecDeque::<(usize, usize)>::new();
    //     let mut curr_val = 0;
    //     let mut next = VecDeque::<(usize, usize)>::new();
    //     curr.push_back((r, c));
    //     while !curr.is_empty() {
    //         while let Some((r, c)) = curr.pop_front() {
    //             let val = rooms[r][c];
    //             if val == -1 || rooms[r][c] < curr_val {
    //                 continue;
    //             }
    //             rooms[r][c] = curr_val;
    //             for (dr, dc) in MOVEMENTS.iter() {
    //                 let nr = dr + (r as isize);
    //                 let nc = dc + (c as isize);
    //                 if nr >= 0 && nc >= 0 && nr < rows && nc < cols {
    //                     next.push_back((nr as usize, nc as usize));
    //                 }
    //             }
    //         }
    //         swap(&mut curr, &mut next);
    //         curr_val += 1;
    //     }
    // }
}
// @lc code=end
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walls_and_gates() {
        let mut src: Vec<Vec<i32>> = vec![
            vec![INF, -1, 0, INF],
            vec![INF, INF, INF, -1],
            vec![INF, -1, INF, -1],
            vec![0, -1, INF, INF],
        ];
        let target: Vec<Vec<i32>> = vec![
            vec![3, -1, 0, 1],
            vec![2, 2, 1, -1],
            vec![1, -1, 2, -1],
            vec![0, -1, 3, 4],
        ];
        Solution::walls_and_gates(&mut src);
        assert_eq!(src, target);
    }
}
