// # [2912. Number of Ways to Reach Destination in the Grid](https://leetcode.com/problems/number-of-ways-to-reach-destination-in-the-grid)

// ## Description

// You are given two integers n and m which represent the size of a 1-indexed grid. You are also given an integer k, a 1-indexed integer array source and a 1-indexed integer array dest, where source and dest are in the form [x, y] representing a cell on the given grid.

// You can move through the grid in the following way:

//
// 	You can go from cell [x1, y1] to cell [x2, y2] if either x1 == x2 or y1 == y2.
// 	Note that you can 't move to the cell you are already in e.g. x1 == x2 and y1 == y2.
//

// Return the number of ways you can reach dest from source by moving through the grid exactly k times.

// Since the answer may be very large, return it modulo 109 + 7.

//
// Example 1:

//
// Input: n = 3, m = 2, k = 2, source = [1,1], dest = [2,2]
// Output: 2
// Explanation: There are 2 possible sequences of reaching [2,2] from [1,1]:
// - [1,1] -> [1,2] -> [2,2]
// - [1,1] -> [2,1] -> [2,2]
//

// Example 2:

//
// Input: n = 3, m = 4, k = 3, source = [1,2], dest = [2,3]
// Output: 9
// Explanation: There are 9 possible sequences of reaching [2,3] from [1,2]:
// - [1,2] -> [1,1] -> [1,3] -> [2,3]
// - [1,2] -> [1,1] -> [2,1] -> [2,3]
// - [1,2] -> [1,3] -> [3,3] -> [2,3]
// - [1,2] -> [1,4] -> [1,3] -> [2,3]
// - [1,2] -> [1,4] -> [2,4] -> [2,3]
// - [1,2] -> [2,2] -> [2,1] -> [2,3]
// - [1,2] -> [2,2] -> [2,4] -> [2,3]
// - [1,2] -> [3,2] -> [2,2] -> [2,3]
// - [1,2] -> [3,2] -> [3,3] -> [2,3]
//

//
// Constraints:

//
// 	2  <= n, m  <= 109
// 	1  <= k  <= 105
// 	source.length == dest.length == 2
// 	1  <= source[1], dest[1]  <= n
// 	1  <= source[2], dest[2]  <= m
//

//     int number_of_ways(int n, int m, int k, vector<int>& source, vector<int>& dest) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn number_of_ways(n: i32, m: i32, k: i32, source: Vec<i32>, dest: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_number_of_ways_1() {
        assert_eq!(2, Solution::number_of_ways(3, 2, 2, vec![1, 1], vec![2, 2]));
    }
    #[test]
    pub fn test_number_of_ways_2() {
        assert_eq!(9, Solution::number_of_ways(3, 4, 3, vec![1, 2], vec![2, 3]));
    }
}
