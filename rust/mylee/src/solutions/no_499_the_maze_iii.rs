// 499\. The Maze III
// ==================

// There is a **ball** in a maze with empty spaces and walls. The ball can go through empty spaces by rolling **up** (u), **down** (d), **left** (l) or **right** (r),
// but it won't stop rolling until hitting a wall. When the ball stops, it could choose the next direction. There is also a **hole** in this maze.
// The ball will drop into the hole if it rolls on to the hole.

// Given the **ball position**, the **hole position** and the **maze**, find out how the ball could drop into the hole by moving the **shortest distance**.
// The distance is defined by the number of **empty spaces** traveled by the ball from the start position (excluded) to the hole (included).
// Output the moving **directions** by using 'u', 'd', 'l' and 'r'. Since there could be several different shortest ways,
// you should output the **lexicographically smallest** way. If the ball cannot reach the hole, output "impossible".

// The maze is represented by a binary 2D array. 1 means the wall and 0 means the empty space. You may assume that the borders of the maze are all walls.
// The ball and the hole coordinates are represented by row and column indexes.

// **Example 1:**

// **Input 1:** a maze represented by a 2D array

// 0 0 0 0 0
// 1 1 0 0 1
// 0 0 0 0 0
// 0 1 0 0 1
// 0 1 0 0 0

// **Input 2:** ball coordinate (rowBall, colBall) = (4, 3)
// **Input 3:** hole coordinate (rowHole, colHole) = (0, 1)

// **Output:** "lul"

// **Explanation:** There are two shortest ways for the ball to drop into the hole.
// The first way is left -> up -> left, represented by "lul".
// The second way is up -> left, represented by 'ul'.
// Both ways have shortest distance 6, but the first way is lexicographically smaller because 'l' < 'u'. So the output is "lul".
// ![](https://assets.leetcode.com/uploads/2018/10/13/maze_2_example_1.png)

// **Example 2:**

// **Input 1:** a maze represented by a 2D array

// 0 0 0 0 0
// 1 1 0 0 1
// 0 0 0 0 0
// 0 1 0 0 1
// 0 1 0 0 0

// **Input 2:** ball coordinate (rowBall, colBall) = (4, 3)
// **Input 3:** hole coordinate (rowHole, colHole) = (3, 0)

// **Output:** "impossible"

// **Explanation:** The ball cannot reach the hole.
// ![](https://assets.leetcode.com/uploads/2018/10/13/maze_2_example_2.png)

// **Note:**

// 1.  There is only one ball and one hole in the maze.
// 2.  Both the ball and hole exist on an empty space, and they will not be at the same position initially.
// 3.  The given maze does not contain border (like the red rectangle in the example pictures), but you could assume the border of the maze are all walls.
// 4.  The maze contains at least 2 empty spaces, and the width and the height of the maze won't exceed 30.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
// use std::collections::{HashMap, VecDeque};

// type Pos = (isize, isize);
// const MOVEMENTS: &[(isize, isize, char); 4] =
//     &[(-1, 0, 'u'), (0, 1, 'r'), (1, 0, 'd'), (0, -1, 'l')];

impl Solution {
    pub fn find_shortest_way(maze: Vec<Vec<i32>>, ball: Vec<i32>, hole: Vec<i32>) -> String {
        // let impossible = String::from("impossible");
        // let rows = maze.len() as isize;
        // let cols = if rows == 0 { 0 } else { maze[0].len() as isize };
        // if rows == 0 || cols == 0 {
        //     return impossible.clone();
        // };
        // let start = (ball[0] as isize, ball[1] as isize);
        // let dest = (hole[0] as isize, hole[1] as isize);
        // {
        //     let (y, x) = start;
        //     if x < 0 || y < 0 || y >= rows || x >= cols || maze[y as usize][x as usize] != 0 {
        //         return impossible.clone();
        //     }
        // }
        // if start == dest {
        //     return String::new();
        // };
        // let mut queue = VecDeque::<Pos>::new();
        // let mut memo = HashMap::<Pos, (usize, String)>::new();
        // queue.push_back(start);
        // memo.insert(start, (0, String::new()));
        // let mut min_cost = (usize::max_value(), String::from("z"));
        // while let Some(pos) = queue.pop_front() {
        //     let (y, x) = pos;
        //     'movement: for &(dy, dx, d) in MOVEMENTS {
        //         let mut cy = y;
        //         let mut cx = x;
        //         let mut ccost = memo[&pos].clone();
        //         ccost.1.push(d);
        //         loop {
        //             let ny = cy + dy;
        //             let nx = cx + dx;
        //             let cpos = (cy, cx);
        //             let npos = (ny, nx);
        //             if npos == dest {
        //                 min_cost = if ccost < min_cost {
        //                     ccost.clone()
        //                 } else {
        //                     min_cost
        //                 };
        //                 continue 'movement;
        //             }
        //             let next_invalid = ny < 0
        //                 || nx < 0
        //                 || ny >= rows
        //                 || nx >= cols
        //                 || maze[ny as usize][nx as usize] != 0;
        //             if next_invalid {
        //                 if !memo.contains_key(&cpos) || ccost < memo[&cpos] {
        //                     queue.push_back(cpos);
        //                     memo.insert(cpos, ccost);
        //                 }
        //                 continue 'movement;
        //             }
        //             cy = ny;
        //             cx = nx;
        //             ccost.0 += 1;
        //             if min_cost < ccost {
        //                 continue 'movement;
        //             }
        //         }
        //     }
        // }
        // if min_cost.0 == usize::max_value() {
        //     impossible
        // } else {
        //     min_cost.1
        // }
        let (m, n) = (maze.len(), maze[0].len());
        let mut dists = vec![vec![i32::MAX; n]; m];
        dists[ball[0] as usize][ball[1] as usize] = 0;
        use std::collections::HashMap;
        let mut u = HashMap::new();
        fn dfs(
            maze: &mut Vec<Vec<i32>>,
            ball: &Vec<i32>,
            hole: &Vec<i32>,
            dists: &mut Vec<Vec<i32>>,
            u: &mut HashMap<Vec<i32>, String>,
        ) {
            if ball == hole {
                return;
            }
            let (m, n) = (maze.len() as i32, maze[0].len() as i32);
            let dirs = [0, 1, 0, -1, 0];
            let way = "rdlu".as_bytes();
            for (i, &d) in dirs[1..].iter().enumerate() {
                let (mut x, mut y) = (ball[0], ball[1]);
                let mut dist = dists[x as usize][y as usize];
                let mut path = u.get(&vec![x, y]).unwrap_or(&String::new()).clone();
                while x >= 0
                    && x < m
                    && y >= 0
                    && y < n
                    && maze[x as usize][y as usize] == 0
                    && (x != hole[0] || y != hole[1])
                {
                    x += dirs[i];
                    y += d;
                    dist += 1;
                }
                if x != hole[0] || y != hole[1] {
                    x -= dirs[i];
                    y -= d;
                    dist -= 1;
                }
                path.push(way[i] as char);
                if dists[x as usize][y as usize] > dist {
                    dists[x as usize][y as usize] = dist;
                    u.insert(vec![x, y], path);
                    dfs(maze, &vec![x, y], hole, dists, u);
                } else if dists[x as usize][y as usize] == dist
                    && &path < u.get(&vec![x, y]).unwrap_or(&String::new())
                {
                    u.insert(vec![x, y], path);
                    dfs(maze, &vec![x, y], hole, dists, u);
                }
            }
        }
        let mut maze = maze;
        dfs(&mut maze, &ball, &hole, &mut dists, &mut u);
        if let Some(s) = u.get(&hole) {
            s.clone()
        } else {
            "impossible".to_string()
        }
    }
}
// @lc code=end

#[allow(dead_code)]
pub  struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_path_1() {
        let maze = vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1],
            vec![0, 1, 0, 0, 0],
        ];
        assert_eq!(
            Solution::find_shortest_way(maze, vec![4, 3], vec![0, 1]),
            String::from("lul")
        );
    }

    #[test]
    fn test_has_path_2() {
        let maze = vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1],
            vec![0, 1, 0, 0, 0],
        ];
        assert_eq!(
            Solution::find_shortest_way(maze, vec![4, 3], vec![3, 0]),
            String::from("impossible")
        );
    }

    #[test]
    fn test_has_path_3() {
        let maze = vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1],
        ];
        assert_eq!(
            Solution::find_shortest_way(maze, vec![0, 4], vec![3, 5]),
            String::from("dldr")
        );
    }
}
