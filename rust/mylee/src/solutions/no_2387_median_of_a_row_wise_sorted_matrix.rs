// # [2387. Median of a Row Wise Sorted Matrix](https://leetcode.com/problems/median-of-a-row-wise-sorted-matrix)

// ## Description

// Given an m x n matrix grid containing an odd number of integers where each row is sorted in non-decreasing order,
// return the median of the matrix.

// You must solve the problem in O(m * log(n)) time complexity.

// Example 1:

// Input: grid = [[1,1,2],[2,3,3],[1,3,4]]
// Output: 2
// Explanation: The elements of the matrix in sorted order are 1,1,1,2,2,3,3,3,4. The median is 2.

// Example 2:

// Input: grid = [[1,1,3,3,4]]
// Output: 3
// Explanation: The elements of the matrix in sorted order are 1,1,3,3,4. The median is 3.

// Constraints:

// 	m == grid.length
// 	n == grid[i].length
// 	1 <= m, n <= 500
// 	m and n are both odd.
// 	1 <= grid[i][j] <= 10^6
// 	grid[i] is sorted in non-decreasing order.

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_median(arr: Vec<Vec<i32>>) -> i32 {
        let (mut l, mut r) = (0, i32::MAX / 2);
        let mut ans = 0;
        let med = arr.len() as i64 * arr[0].len() as i64 / 2 + 1;
        while l <= r {
            let m = l + (r - l) / 2;
            let mut now = 0;
            for a in &arr {
                let i = a.partition_point(|x| *x < m);
                now += (a.len() - i) as i64;
            }
            if now >= med {
                ans = ans.max(m);
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_find_median_1() {
        assert_eq!(
            2,
            Solution::find_median(vec![vec![1, 1, 2], vec![2, 3, 3], vec![1, 3, 4]])
        );
    }
    #[test]
    pub fn test_find_median_2() {
        assert_eq!(3, Solution::find_median(vec![vec![1, 1, 3, 3, 4]]));
    }
}
