// [3802\. Number of Ways to Paint Sheets 🔒](https://leetcode.com/problems/number-of-ways-to-paint-sheets)

// Description
// -----------

// You are given an integer `n` representing the number of sheets.

// You are also given an integer array `limit` of size `m`,
// where `limit[i]` is the **maximum** number of sheets that can be painted using color `i`.

// You must paint **all** `n` sheets under the following conditions:

// *   **Exactly two distinct** colors are used.
// *   Each color must cover a **single contiguous** segment of sheets.
// *   The number of sheets painted with color `i` cannot exceed `limit[i]`.

// Return an integer denoting the number of **distinct** ways to paint all sheets.
// Since the answer may be large, return it **modulo** `109 + 7`.

// **Note:** Two ways differ if **at least** one sheet is painted with a different color.

// **Example 1:**

// **Input:** n = 4, limit = \[3,1,2\]

// **Output:** 6

// **Explanation:**​​​​​​​

// For each ordered pair `(i, j)`,
// where color `i` is used for the first segment and color `j` for the second segment (`i != j`),
//  a split of `x` and `4 - x` is valid if `1 <= x <= limit[i]` and `1 <= 4 - x <= limit[j]`.

// Valid pairs and counts are:

// *   `(0, 1): x = 3`
// *   `(0, 2): x = 2, 3`
// *   `(1, 0): x = 1`
// *   `(2, 0): x = 1, 2`

// Therefore, there are 6 valid ways in total.

// **Example 2:**

// **Input:** n = 3, limit = \[1,2\]

// **Output:** 2

// **Explanation:**

// For each ordered pair `(i, j)`,
// where color `i` is used for the first segment and color `j` for the second segment (`i != j`),
//  a split of `x` and `3 - x` is valid if `1 <= x <= limit[i]` and `1 <= 3 - x <= limit[j]`.

// Valid pairs and counts are:

// *   `(0, 1): x = 1`
// *   `(1, 0): x = 2`

// Hence, there are 2 valid ways in total.

// **Example 3:**

// **Input:** n = 3, limit = \[2,2\]

// **Output:** 4

// **Explanation:**

// For each ordered pair `(i, j)`,
//  where color `i` is used for the first segment and color `j` for the second segment (`i != j`),
// a split of `x` and `3 - x` is valid if `1 <= x <= limit[i]` and `1 <= 3 - x <= limit[j]`.

// Valid pairs and counts are:

// *   `(0, 1): x = 1, 2`
// *   `(1, 0): x = 1, 2`

// Therefore, there are 4 valid ways in total.

// **Constraints:**

// *   `2 <= n <= 109`
// *   `2 <= m == limit.length <= 105`
// *   `1 <= limit[i] <= 109`

//  int number_of_ways(int n, vector<int>& limit) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn number_of_ways(n: i32, mut limit: Vec<i32>) -> i32 {
        let m = limit.len();
        let n1 = n - 1;
        limit.sort_unstable();
        let mut suffix = vec![0; limit.len() + 1];
        for (i, x) in limit.iter_mut().enumerate().rev() {
            *x = n1.min(*x);
            suffix[i] = suffix[i + 1] + *x as i64;
        }
        let (mut ans, mut j) = (0, 0);
        for (i, &x) in limit.iter().enumerate().rev() {
            while j < m {
                if x + limit[j] >= n {
                    break;
                }
                j += 1;
            }
            let mut cnt = (x - n + 1) as i64 * (m - j) as i64 + suffix[j];
            if i >= j {
                cnt -= (x + x - n + 1) as i64;
            }
            ans = (ans + cnt) % 1_000_000_007;
        }
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_number_of_ways_1() {
        assert_eq!(6, Solution::number_of_ways(4, vec![3, 1, 2]));
    }
    #[test]
    pub fn test_number_of_ways_2() {
        assert_eq!(2, Solution::number_of_ways(3, vec![1, 2]));
    }
    #[test]
    pub fn test_number_of_ways_3() {
        assert_eq!(4, Solution::number_of_ways(3, vec![2, 2]));
    }
}
