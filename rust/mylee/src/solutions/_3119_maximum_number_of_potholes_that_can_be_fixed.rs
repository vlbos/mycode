// # [3119. Maximum Number of Potholes That Can Be Fixed 🔒](https://leetcode.com/problems/maximum-number-of-potholes-that-can-be-fixed)

// ## Description

//

// You are given a string road, consisting only of characters "x" and ".",
// where each "x" denotes a pothole and each "." denotes a smooth road,
// and an integer budget.

// In one repair operation, you can repair n consecutive potholes for a price of n + 1.

// Return the maximum number of potholes that can be fixed such that the sum of the prices of all of the fixes doesn't go over the given budget.

//
// Example 1:

//
// Input: road = "..", budget = 5

// Output: 0

// Explanation:

// There are no potholes to be fixed.
//

// Example 2:

//
// Input: road = "..xxxxx", budget = 4

// Output: 3

// Explanation:

// We fix the first three potholes (they are consecutive). The budget needed for this task is 3 + 1 = 4.
//

// Example 3:

//
// Input: road = "x.x.xxx...x", budget = 14

// Output: 6

// Explanation:

// We can fix all the potholes. The total cost would be (1 + 1) + (1 + 1) + (3 + 1) + (1 + 1) = 10 which is within our budget of 14.
//

//
// Constraints:

//
// 	1 <= road.length <= 105
// 	1 <= budget <= 105 + 1
// 	road consists only of characters '.' and 'x'.
//

//     int max_potholes(string road, int budget) {

impl Solution {
    pub fn max_potholes(mut road: String, mut budget: i32) -> i32 {
        let mut cnt = std::collections::BTreeMap::new();
        road.push('.');
        let mut k = 0;
        for c in road.chars() {
            if c == 'x' {
                k += 1;
            } else if k > 0 {
                *cnt.entry(k).or_insert(0) += 1;
                k = 0;
            }
        }

        let mut ans = 0;
        for (k, v) in cnt.into_iter().rev() {
            if budget < k + 1 {
                ans += budget - 1;
                break;
            }
            if budget <= v * (k + 1) {
                ans += budget - (budget + k) / (k + 1);
                budget = 0;
            } else {
                ans += v * k;
                budget -= (k + 1) * v;
            }
            if budget == 0 {
                break;
            }
        }
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    // road =
    // "xx..x"
    // budget =
    // 4

    // Use Testcase
    // Stdout
    // {1: 1, 2: 1}
    // Output
    // 3
    // Expected
    // 2
    #[test]
    pub fn test_max_potholes_1() {
        assert_eq!(Solution::max_potholes(String::from(".."), 5), 0);
    }

    #[test]
    pub fn test_max_potholes_2() {
        assert_eq!(Solution::max_potholes(String::from("..xxxxx"), 4), 3);
    }
    #[test]
    pub fn test_max_potholes_3() {
        assert_eq!(Solution::max_potholes(String::from("x.x.xxx...x"), 14), 6);
    }
}
