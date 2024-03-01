// 505\. The Maze II
// =================

// There is a **ball** in a maze with empty spaces and walls. The ball can go through empty spaces by rolling **up**, **down**, **left** or **right**,
// but it won't stop rolling until hitting a wall. When the ball stops, it could choose the next direction.

// Given the ball's **start position**, the **destination** and the **maze**, find the shortest distance for the ball to stop at the destination.
// The distance is defined by the number of **empty spaces** traveled by the ball from the start position (excluded) to the destination (included).
// If the ball cannot stop at the destination, return -1.

// The maze is represented by a binary 2D array. 1 means the wall and 0 means the empty space. You may assume that the borders of the maze are all walls.
// The start and destination coordinates are represented by row and column indexes.

// **Example 1:**

// **Input 1:** a maze represented by a 2D array

// 0 0 1 0 0
// 0 0 0 0 0
// 0 0 0 1 0
// 1 1 0 1 1
// 0 0 0 0 0

// **Input 2:** start coordinate (rowStart, colStart) = (0, 4)
// **Input 3:** destination coordinate (rowDest, colDest) = (4, 4)

// **Output:** 12

// **Explanation:** One shortest way is : left -> down -> left -> down -> right -> down -> right.
//              The total distance is 1 + 1 + 3 + 1 + 2 + 2 + 2 = 12.
// ![](https://assets.leetcode.com/uploads/2018/10/12/maze_1_example_1.png)

// **Example 2:**

// **Input 1:** a maze represented by a 2D array

// 0 0 1 0 0
// 0 0 0 0 0
// 0 0 0 1 0
// 1 1 0 1 1
// 0 0 0 0 0

// **Input 2:** start coordinate (rowStart, colStart) = (0, 4)
// **Input 3:** destination coordinate (rowDest, colDest) = (3, 2)

// **Output:** -1

// **Explanation:** There is no way for the ball to stop at the destination.
// ![](https://assets.leetcode.com/uploads/2018/10/13/maze_1_example_2.png)

// **Note:**

// 1.  There is only one ball and one destination in the maze.
// 2.  Both the ball and the destination exist on an empty space, and they will not be at the same position initially.
// 3.  The given maze does not contain border (like the red rectangle in the example pictures), but you could assume the border of the maze are all walls.
// 4.  The maze contains at least 2 empty spaces, and both the width and height of the maze won't exceed 100.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Oracle](https://leetcode.ca/tags/#Oracle) [Snapchat](https://leetcode.ca/tags/#Snapchat)

// @lc code=start
// use std::collections::{HashMap, VecDeque};

// type Pos = (isize, isize);
// const MOVEMENTS: &[Pos; 4] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

impl Solution {
    pub fn shortest_distance(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> i32 {
        // let rows = maze.len() as isize;
        // let cols = if rows == 0 { 0 } else { maze[0].len() as isize };
        // if rows == 0 || cols == 0 {
        //     return -1;
        // };
        // let start = (start[0] as isize, start[1] as isize);
        // let dest = (destination[0] as isize, destination[1] as isize);
        // {
        //     let (y, x) = start;
        //     if x < 0 || y < 0 || y >= rows || x >= cols || maze[y as usize][x as usize] != 0 {
        //         return -1;
        //     }
        // }
        // if start == dest {
        //     return 0;
        // };
        // let mut queue = VecDeque::<Pos>::new();
        // let mut memo = HashMap::<Pos, usize>::new();
        // queue.push_back(start);
        // memo.insert(start, 0);
        // let mut min_cost = usize::max_value();
        // while let Some(pos) = queue.pop_front() {
        //     let (y, x) = pos;
        //     let cost = memo[&pos];
        //     'movement: for &(dy, dx) in MOVEMENTS {
        //         let mut cy = y;
        //         let mut cx = x;
        //         let mut ccost = cost;
        //         loop {
        //             let ny = cy + dy;
        //             let nx = cx + dx;
        //             let cpos = (cy, cx);
        //             let next_invalid = ny < 0
        //                 || nx < 0
        //                 || ny >= rows
        //                 || nx >= cols
        //                 || maze[ny as usize][nx as usize] != 0;
        //             if next_invalid {
        //                 if !memo.contains_key(&cpos) || memo[&cpos] > ccost {
        //                     memo.insert(cpos, ccost);
        //                     if cpos == dest {
        //                         min_cost = usize::min(min_cost, ccost);
        //                     } else {
        //                         queue.push_back(cpos);
        //                     }
        //                 }
        //                 continue 'movement;
        //             }
        //             cy = ny;
        //             cx = nx;
        //             ccost += 1;
        //             if ccost >= min_cost {
        //                 continue 'movement;
        //             }
        //         }
        //     }
        // }
        // if min_cost == usize::max_value() {
        //     -1
        // } else {
        //     min_cost as i32
        // }
        let (m, n) = (maze.len(), maze[0].len());
        let mut dists = vec![vec![i32::MAX; n]; m];
        dists[start[0] as usize][start[1] as usize] = 0;
        pub fn dfs(
            maze: &mut Vec<Vec<i32>>,
            start: &Vec<i32>,
            destination: &Vec<i32>,
            dists: &mut Vec<Vec<i32>>,
        ) {
            let (m, n) = (maze.len() as i32, maze[0].len() as i32);
            let dirs = [0, 1, 0, -1, 0];
            for (i, &d) in dirs[1..].iter().enumerate() {
                let (mut x, mut y) = (start[0] + dirs[i], start[1] + d);
                let mut dist = 0;
                while x >= 0 && x < m && y >= 0 && y < n && maze[x as usize][y as usize] == 0 {
                    x += dirs[i];
                    y += d;
                    dist += 1;
                }
                x -= dirs[i];
                y -= d;
                if dists[x as usize][y as usize]
                    > dist + dists[start[0] as usize][start[1] as usize]
                {
                    dists[x as usize][y as usize] =
                        dist + dists[start[0] as usize][start[1] as usize];
                    dfs(maze, &vec![x, y], destination, dists);
                }
            }
        }
        let mut maze = maze;
        dfs(&mut maze, &start, &destination, &mut dists);
        if dists[destination[0] as usize][destination[1] as usize] != i32::MAX {
            dists[destination[0] as usize][destination[1] as usize]
        } else {
            -1
        }
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_has_path_1() {
        let maze = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(
            Solution::shortest_distance(maze, vec![0, 4], vec![4, 4]),
            12
        );
    }

    #[test]
    pub fn test_has_path_2() {
        let maze = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(
            Solution::shortest_distance(maze, vec![0, 4], vec![3, 2]),
            -1
        );
    }
}
