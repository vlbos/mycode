// [2107\. Number of Unique Flavors After Sharing K Candies (Medium)](https://leetcode.com/problems/number-of-unique-flavors-after-sharing-k-candies/)[](https://leetcode.ca/2021-12-30-2107-Number-of-Unique-Flavors-After-Sharing-K-Candies/#2107-number-of-unique-flavors-after-sharing-k-candies-medium)
// =========================================================================================================================================================================================================================================================================================================

// You are given a **0-indexed** integer array `candies`, where `candies[i]` represents the flavor of the `ith` candy.
// Your mom wants you to share these candies with your little sister by giving her `k` **consecutive** candies, but you want to keep as many flavors of candies as possible.

// Return _the **maximum** number of **unique** flavors of candy you can keep after sharing_ _with your sister._

// **Example 1:**

// **Input:** candies = \[1,2,2,3,4,3\], k = 3
// **Output:** 3
// **Explanation:**
// Give the candies in the range \[1, 3\] (inclusive) with flavors \[2,2,3\].
// You can eat candies with flavors \[1,4,3\].
// There are 3 unique flavors, so return 3.

// **Example 2:**

// **Input:** candies = \[2,2,2,2,3,3\], k = 2
// **Output:** 2
// **Explanation:**
// Give the candies in the range \[3, 4\] (inclusive) with flavors \[2,3\].
// You can eat candies with flavors \[2,2,2,3\].
// There are 2 unique flavors, so return 2.
// Note that you can also share the candies with flavors \[2,2\] and eat the candies with flavors \[2,2,3,3\].

// **Example 3:**

// **Input:** candies = \[2,4,5\], k = 0
// **Output:** 3
// **Explanation:**
// You do not have to give any candies.
// You can eat the candies with flavors \[2,4,5\].
// There are 3 unique flavors, so return 3.

// **Constraints:**

// *   `1 <= candies.length <= 105`
// *   `1 <= candies[i] <= 105`
// *   `0 <= k <= candies.length`

// **Companies**:
// [Microsoft](https://leetcode.com/company/microsoft)

// **Related Topics**:
// [Array](https://leetcode.com/tag/array/), [Hash Table](https://leetcode.com/tag/hash-table/), [Sliding Window](https://leetcode.com/tag/sliding-window/)

// **Similar Questions**:

// *   [Remove Boxes (Hard)](https://leetcode.com/problems/remove-boxes/)
// *   [Subarrays with K Different Integers (Hard)](https://leetcode.com/problems/subarrays-with-k-different-integers/)

// Solution 1. Fixed-length Sliding Window[](https://leetcode.ca/2021-12-30-2107-Number-of-Unique-Flavors-After-Sharing-K-Candies/#solution-1-fixed-length-sliding-window)
// -----------------------------------------------------------------------------------------------------------------------------------------------------------------------

//     // OJ: https://leetcode.com/problems/number-of-unique-flavors-after-sharing-k-candies/
//     // Time: O(N)
//     // Space: O(N)
//     class Solution {
//     public:
//         int share_candies(vector<int>& A, int k) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn share_candies(candies: Vec<i32>, k: i32) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let k = k as usize;
        for &c in &candies[k..] {
            *cnt.entry(c).or_insert(0) += 1;
        }
        let mut ans = cnt.len();
        for (i, &c) in candies[k..].iter().enumerate() {
            *cnt.entry(candies[0]).or_insert(0) += 1;
            *cnt.entry(c).or_insert(0) -= 1;
            if *cnt.get(&c).unwrap() == 0 {
                cnt.remove(&c);
            }
            if cnt.len() > ans {
                ans = cnt.len();
            }
        }
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_share_candies_1() {
        assert_eq!(3, Solution::share_candies(vec![1, 2, 2, 3, 4, 3], 3));
    }
    #[test]
    pub fn test_share_candies_2() {
        assert_eq!(2, Solution::share_candies(vec![2, 2, 2, 2, 3, 3], 2));
    }
    #[test]
    pub fn test_share_candies_3() {
        assert_eq!(3, Solution::share_candies(vec![2, 4, 5], 0));
    }
}
