// # [2174. Remove All Ones With Row and Column Flips II](https://leetcode.com/problems/remove-all-ones-with-row-and-column-flips-ii)

// ## Description

// You are given a 0-indexed m x n binary matrix grid.

// In one operation, you can choose any i and j that meet the following conditions:

//
// 	0 <= i < m
// 	0 <= j < n
// 	grid[i][j] == 1
//

// and change the values of all cells in row i and column j to zero.

// Return the minimum number of operations needed to remove all 1's from grid.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2174.Remove%20All%20Ones%20With%20Row%20and%20Column%20Flips%20II/images/image-20220213162716-1.png" style="width: 709px; height: 200px;" />
//
// Input: grid = [[1,1,1],[1,1,1],[0,1,0]]
// Output: 2
// Explanation:
// In the first operation, change all cell values of row 1 and column 1 to zero.
// In the second operation, change all cell values of row 0 and column 0 to zero.
//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2174.Remove%20All%20Ones%20With%20Row%20and%20Column%20Flips%20II/images/image-20220213162737-2.png" style="width: 734px; height: 200px;" />
//
// Input: grid = [[0,1,0],[1,0,1],[0,1,0]]
// Output: 2
// Explanation:
// In the first operation, change all cell values of row 1 and column 0 to zero.
// In the second operation, change all cell values of row 2 and column 1 to zero.
// Note that we cannot perform an operation using row 1 and column 1 because grid[1][1] != 1.
//

// Example 3:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2174.Remove%20All%20Ones%20With%20Row%20and%20Column%20Flips%20II/images/image-20220213162752-3.png" style="width: 156px; height: 150px;" />
//
// Input: grid = [[0,0],[0,0]]
// Output: 0
// Explanation:
// There are no 1's to remove so return 0.
//

// Constraints:

//
// 	m == grid.length
// 	n == grid[i].length
// 	1 <= m, n <= 15
// 	1 <= m * n <= 15
// 	grid[i][j] is either 0 or 1.
//

// int removeOnes(vector<vector<int>>& grid) {

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
