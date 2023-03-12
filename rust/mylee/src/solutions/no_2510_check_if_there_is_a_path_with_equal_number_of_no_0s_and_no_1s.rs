// # [2510. Check if There is a Path With Equal Number of 0's And 1's](https://leetcode.com/problems/check-if-there-is-a-path-with-equal-number-of-0s-and-1s)
// ## Description

//  You are given a  0-indexed   m x n   binary  matrix  grid . You can move from a cell  (row, col)  to any of the cells  (row + 1, col)  or  (row, col + 1) .

//  Return  true   if there is a path from   (0, 0)   to   (m - 1, n - 1)   that visits an  equal  number of   0  &#39;s and   1  &#39;s . Otherwise return  false .

//  Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2500-2599/2510.Check%20if%20There%20is%20a%20Path%20With%20Equal%20Number%20of%200%27s%20And%201%27s/images/yetgriddrawio-4.png" />

//  Input:  grid = [[0,1,0,0],[0,1,0,0],[1,0,1,0]]
//  Output:  true
//  Explanation:  The path colored in blue in the above diagram is a valid path because we have 3 cells with a value of 1 and 3 with a value of 0. Since there is a valid path, we return true.

//  Example 2:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2500-2599/2510.Check%20if%20There%20is%20a%20Path%20With%20Equal%20Number%20of%200%27s%20And%201%27s/images/yetgrid2drawio-1.png" style="width: 151px; height: 151px;" />

//  Input:  grid = [[1,1,0],[0,0,1],[1,0,0]]
//  Output:  false
//  Explanation:  There is no path in this grid with an equal number of 0&#39;s and 1&#39;s.

//   Constraints:

// 	  m == grid.length
// 	  n == grid[i].length
// 	  2 <= m, n <= 100
// 	  grid[i][j]  is either  0  or  1 .
//  bool is_there_a_path(vector<vector<int>>& grid) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_there_a_path(grid: Vec<Vec<i32>>) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_is_there_a_path_1() {
        assert!(Solution::is_there_a_path(vec![
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0]
        ]));
    }
    #[test]
    pub fn test_is_there_a_path_2() {
        assert!(!Solution::is_there_a_path(vec![
            vec![1, 1, 0],
            vec![0, 0, 1],
            vec![1, 0, 0]
        ]));
    }
}
