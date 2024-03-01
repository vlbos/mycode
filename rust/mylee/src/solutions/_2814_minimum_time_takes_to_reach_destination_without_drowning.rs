// # [2814. Minimum Time Takes to Reach Destination Without Drowning](https://leetcode.com/problems/minimum-time-takes-to-reach-destination-without-drowning)

// ## Description

// You are given an n * m 0-indexed grid of string land. Right now, you are standing at the cell that contains "S",
// and you want to get to the cell containing "D". There are three other types of cells in this land:

//
// 	".": These cells are empty.
// 	"X": These cells are stone.
// 	"*": These cells are flooded.
//

// At each second, you can move to a cell that shares a side with your current cell (if it exists).
// Also, at each second, every empty cell that shares a side with a flooded cell becomes flooded as well.
// There are two problems ahead of your journey:

//
// 	You can't step on stone cells.
// 	You can't step on flooded cells since you will drown (also, you can't step on a cell that will be flooded at the same time as you step on it).
//

// Return the minimum time it takes you to reach the destination in seconds, or -1 if it is impossible.

// Note that the destination will never be flooded.

//
// ### Example 1:

//
// Input: land = [["D",".","*"],[".",".","."],[".","S","."]]
// Output: 3
// Explanation: The picture below shows the simulation of the land second by second.
// The blue cells are flooded, and the gray cells are stone.
// Picture (0) shows the initial state and picture (3) shows the final state when we reach destination.
// As you see, it takes us 3 second to reach destination and the answer would be 3.
// It can be shown that 3 is the minimum time needed to reach from S to D.
//

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2800-2899/2814.Minimum%20Time%20Takes%20to%20Reach%20Destination%20Without%20Drowning/images/ex1.png" style="padding: 5px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 600px; height: 111px;" />

// ### Example 2:

//
// Input: land = [["D","X","*"],[".",".","."],[".",".","S"]]
// Output: -1
// Explanation: The picture below shows the simulation of the land second by second.
// The blue cells are flooded, and the gray cells are stone.
// Picture (0) shows the initial state. As you see, no matter which paths we choose, we will drown at the 3rd second.
// Also the minimum path takes us 4 seconds to reach from S to D.
// So the answer would be -1.
//

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2800-2899/2814.Minimum%20Time%20Takes%20to%20Reach%20Destination%20Without%20Drowning/images/ex2-2.png" style="padding: 7px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 600px; height: 107px;" />

// ### Example 3:

//
// Input: land = [["D",".",".",".","*","."],[".","X",".","X",".","."],[".",".",".",".","S","."]]
// Output: 6
// Explanation: It can be shown that we can reach destination in 6 seconds.
// Also it can be shown that 6 is the minimum seconds one need to reach from S to D.
//

//
// Constraints:

//
// 	2  <= n, m  <= 100
// 	land consists only of "S", "D", ".", "*" and "X".
// 	Exactly one of the cells is equal to "S".
// 	Exactly one of the cells is equal to "D".
//

// ## Solutions

// ```cpp
// class Solution {
// public:
//     int minimum_seconds(vector<vector<string>>& land) {

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn minimum_seconds(land: Vec<Vec<String>>) -> i32 {
        let (m, n) = (land.len(), land[0].len());
        let mut g = vec![vec![i32::MAX / 2; n]; m];
        let mut vis = vec![vec![false; n]; m];
        let mut q = std::collections::VecDeque::new();
        let (mut si, mut sj) = (0, 0);
        for (i, row) in land.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if c.as_str() == "*" {
                    q.push_back((i, j));
                } else if c.as_str() == "S" {
                    si = i;
                    sj = j;
                }
            }
        }
        let mut t = 0;
        while !q.is_empty() {
            let len = q.len();
            for _ in 0..len {
                let (i, j) = q.pop_front().unwrap();
                g[i][j] = t;
                for d in [-1, 0, 1, 0, -1].windows(2) {
                    let (x, y) = (i as i32 + d[0], j as i32 + d[1]);
                    if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                        continue;
                    }
                    let (x, y) = (x as usize, y as usize);
                    if !vis[x][y] && (land[x][y].as_str() == "." || land[x][y].as_str() == "S") {
                        vis[x][y] = true;
                        q.push_back((x, y));
                    }
                }
            }
            t += 1;
        }
        let mut vis = vec![vec![false; n]; m];
        vis[si][sj] = true;
        let mut q = std::collections::VecDeque::from([(si, sj)]);
        let mut t = 0;
        while !q.is_empty() {
            let len = q.len();
            for _ in 0..len {
                let (i, j) = q.pop_front().unwrap();
                if land[i][j].as_str() == "D" {
                    return t;
                }
                for d in [-1, 0, 1, 0, -1].windows(2) {
                    let (x, y) = (i as i32 + d[0], j as i32 + d[1]);
                    if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                        continue;
                    }
                    let (x, y) = (x as usize, y as usize);
                    if g[x][y] > t + 1
                        && !vis[x][y]
                        && (land[x][y].as_str() == "." || land[x][y].as_str() == "D")
                    {
                        vis[x][y] = true;
                        q.push_back((x, y));
                    }
                }
            }
            t += 1;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix_s;
    #[test]
    pub fn test_minimum_seconds_1() {
        assert_eq!(
            3,
            Solution::minimum_seconds(lc_matrix_s![
                ["D", ".", "*"],
                [".", ".", "."],
                [".", "S", "."]
            ])
        );
    }
    #[test]
    pub fn test_minimum_seconds_2() {
        assert_eq!(
            -1,
            Solution::minimum_seconds(lc_matrix_s![
                ["D", "X", "*"],
                [".", ".", "."],
                [".", ".", "S"]
            ])
        );
    }
    #[test]
    pub fn test_minimum_seconds_3() {
        assert_eq!(
            6,
            Solution::minimum_seconds(lc_matrix_s![
                ["D", ".", ".", ".", "*", "."],
                [".", "X", ".", "X", ".", "."],
                [".", ".", ".", ".", "S", "."]
            ])
        );
    }
}
