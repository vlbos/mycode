// # [2371. Minimize Maximum Value in a Grid](https://leetcode.com/problems/minimize-maximum-value-in-a-grid)

// ## Description

// You are given an m x n integer matrix grid containing distinct positive integers.

// You have to replace each integer in the matrix with a positive integer satisfying the following conditions:

//
// 	The relative order of every two elements that are in the same row or column should stay the same after the replacements.
// 	The maximum number in the matrix after the replacements should be as small as possible.
//

// The relative order stays the same if for all pairs of elements in the original matrix such that grid[r1][c1] &gt; grid[r2][c2] where either r1 == r2 or c1 == c2, then it must be true that grid[r1][c1] &gt; grid[r2][c2] after the replacements.

// For example, if grid = [[2, 4, 5], [7, 3, 9]] then a good replacement could be either grid = [[1, 2, 3], [2, 1, 4]] or grid = [[1, 2, 3], [3, 1, 4]].

// Return the resulting matrix. If there are multiple answers, return any of them.

// Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2300-2399/2371.Minimize%20Maximum%20Value%20in%20a%20Grid/images/grid2drawio.png" style="width: 371px; height: 121px;" />
//
// Input: grid = [[3,1],[2,5]]
// Output: [[2,1],[1,2]]
// Explanation: The above diagram shows a valid replacement.
// The maximum number in the matrix is 2. It can be shown that no smaller value can be obtained.
//

// Example 2:

//
// Input: grid = [[10]]
// Output: [[1]]
// Explanation: We replace the only number in the matrix with 1.
//

// Constraints:

//
// 	m == grid.length
// 	n == grid[i].length
// 	1 <= m, n <= 1000
// 	1 <= m * n <= 105
// 	1 <= grid[i][j] <= 109
// 	grid consists of distinct integers.
//

// vector<vector<int>> minScore(vector<vector<int>>& grid) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_word_1() {
        assert_eq!(
            "kiran".to_string(),
            Solution::longest_word(
                ["k", "ki", "kir", "kira", "kiran"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_2() {
        assert_eq!(
            "apple".to_string(),
            Solution::longest_word(
                ["a", "banana", "app", "appl", "ap", "apply", "apple"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_3() {
        assert_eq!(
            String::new(),
            Solution::longest_word(
                ["abc", "bc", "ab", "qwe"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
}
