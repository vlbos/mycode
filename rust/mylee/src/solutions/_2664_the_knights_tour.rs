// # [2664. The Knight’s Tour](https://leetcode.com/problems/the-knights-tour)

// ## Description

//  Given two positive integers  m  and  n  which are the height and width of a  0-indexed  2D-array  board ,
// a pair of positive integers  (r, c)  which is the starting position of the knight on the board.

//  Your task is to find an order of movements for the knight,
// in a manner that every cell of the  board  gets visited  exactly  once (the starting cell is considered visited and you  shouldn't  visit it again).

//  Return  the array   board   in which the cells'values show the order of visiting the cell starting from 0 (the initial place of the knight).

//  Note that a  knight  can  move  from cell  (r1, c1)  to cell  (r2, c2)
// if  0  <= r2  <= m - 1  and  0  <= c2  <= n - 1  and  min(abs(r1 - r2),
// abs(c1 - c2)) = 1  and  max(abs(r1 - r2), abs(c1 - c2)) = 2 .

//   Example 1:

//  Input:  m = 1, n = 1, r = 0, c = 0
//  Output:  [[0]]
//  Explanation:  There is only 1 cell and the knight is initially on it so there is only a 0 inside the 1x1 grid.

//   Example 2:

//  Input:  m = 3, n = 4, r = 0, c = 0
//  Output:  [[0,3,6,9],[11,8,1,4],[2,5,10,7]]
//  Explanation:  By the following order of movements we can visit the entire board.
// (0,0)->(1,2)->(2,0)->(0,1)->(1,3)->(2,1)->(0,2)->(2,3)->(1,1)->(0,3)->(2,2)->(1,0)

//   Constraints:

// 	  1  <= m, n  <= 5
// 	  0  <= r  <= m - 1
// 	  0  <= c  <= n - 1
// 	 The inputs will be generated such that there exists at least one possible order of movements with the given condition

// ## Solutions

// ### **C++**

// ```cpp
// class Solution {
// public:
//     vector<vector > tour_of_knight(int m, int n, int r, int c) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn tour_of_knight(m: i32, n: i32, r: i32, c: i32) -> Vec<Vec<i32>> {
        let mut g = vec![vec![-1; n as usize]; m as usize];
        g[r as usize][c as usize] = 0;
        fn dfs(i: i32, j: i32, m: i32, n: i32, ok: &mut bool, g: &mut Vec<Vec<i32>>) {
            if g[i as usize][j as usize] == m * n - 1 {
                *ok = true;
                return;
            }
            for w in [-2, -1, 2, 1, -2, 1, 2, -1, -2].windows(2) {
                let (x, y) = (i + w[0], j + w[1]);
                if 0 <= x && x < m && 0 <= y && y < n && g[x as usize][y as usize] == -1 {
                    g[x as usize][y as usize] = g[i as usize][j as usize] + 1;
                    dfs(x, y, m, n, ok, g);
                    if *ok {
                        return;
                    }
                    g[x as usize][y as usize] = -1;
                }
            }
        }
        let mut ok = false;
        dfs(r, c, m, n, &mut ok, &mut g);
        g
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_tour_of_knight_1() {
        assert_eq!(vec![vec![0]], Solution::tour_of_knight(1, 1, 0, 0));
    }
    #[test]
    pub fn test_tour_of_knight_2() {
        assert_eq!(
            vec![vec![0, 3, 6, 9], vec![11, 8, 1, 4], vec![2, 5, 10, 7]],
            Solution::tour_of_knight(3, 4, 0, 0)
        );
    }
}
