// 1246\. Palindrome Removal
// =========================

// Given an integer array `arr`, in one move you can select a **palindromic** subarray `arr[i], arr[i+1], ..., arr[j]` where `i <= j`,
// and remove that subarray from the given array. Note that after removing a subarray,
// the elements on the left and on the right of that subarray move to fill the gap left by the removal.

// Return the minimum number of moves needed to remove all numbers from the array.

// **Example 1:**

// **Input:** arr = \[1,2\]
// **Output:** 2

// **Example 2:**

// **Input:** arr = \[1,3,4,1,5\]
// **Output:** 3
// **Explanation:** Remove \[4\] then remove \[1,3,1\] then remove \[5\].

// **Constraints:**

// *   `1 <= arr.length <= 100`
// *   `1 <= arr[i] <= 20`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Microsoft](https://leetcode.ca/tags/#Microsoft)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn minimum_moves(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in 0..=n {
            dp[i][i] = 1;
        }
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                dp[i][j] = dp[i + 1][j] + 1;
                if arr[i] == arr[i + 1] {
                    dp[i][j] = dp[i][j].min(dp[i + 2][j] + 1);
                }
                for k in i + 2..=j {
                    if arr[i] == arr[k] {
                        dp[i][j] = dp[i][j].min(dp[i + 1][k - 1] + dp[k + 1][j]);
                    }
                }
            }
        }
        dp[0][n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minimum_moves_1() {
        assert_eq!(2, Solution::minimum_moves(vec![1, 2]));
    }
    #[test]
    pub fn test_minimum_moves_2() {
        assert_eq!(3, Solution::minimum_moves(vec![1, 3, 4, 1, 5]));
    }
}
