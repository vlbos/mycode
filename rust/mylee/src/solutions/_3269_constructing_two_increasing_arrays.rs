// 3269. Constructing Two Increasing Arrays

// Hard

// Hint

// Given 2 integer arrays `nums1` and `nums2` consisting only of 0 and 1, your task is to calculate the **minimum** possible **largest** number in arrays `nums1` and `nums2`, after doing the following.

// Replace every 0 with an _even positive integer_ and every 1 with an _odd positive integer_. After replacement, both arrays should be **increasing** and each integer should be used **at most** once.

// Return the _minimum possible largest number_ after applying the changes.

// **Example 1:**

// **Input:** nums1 = \[\], nums2 = \[1,0,1,1\]

// **Output:** 5

// **Explanation:**

// After replacing, `nums1 = []`, and `nums2 = [1, 2, 3, 5]`.

// **Example 2:**

// **Input:** nums1 = \[0,1,0,1\], nums2 = \[1,0,0,1\]

// **Output:** 9

// **Explanation:**

// One way to replace, having 9 as the largest element is `nums1 = [2, 3, 8, 9]`, and `nums2 = [1, 4, 6, 7]`.

// **Example 3:**

// **Input:** nums1 = \[0,1,0,0,1\], nums2 = \[0,0,0,1\]

// **Output:** 13

// **Explanation:**

// One way to replace, having 13 as the largest element is `nums1 = [2, 3, 4, 6, 7]`, and `nums2 = [8, 10, 12, 13]`.

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_largest(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (m, n) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];
        dp[0][0] = 0;
        let get = |x: i32, p: i32| x + if (x ^ p) & 1 == 0 { 2 } else { 1 };
        for i in 0..=m {
            for j in (if i == 0 { 1 } else { 0 })..=n {
                if i > 0 {
                    dp[i][j] = get(dp[i - 1][j], nums1[i - 1]);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].min(get(dp[i][j - 1], nums2[j - 1]));
                }
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_largest_1() {
        assert_eq!(5, Solution::min_largest(vec![], vec![1, 0, 1, 1]));
    }
    #[test]
    pub fn test_min_largest_2() {
        assert_eq!(9, Solution::min_largest(vec![0, 1, 0, 1], vec![1, 0, 0, 1]));
    }
}
