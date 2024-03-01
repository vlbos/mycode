// 1788\. Maximize the Beauty of the Garden
// ========================================

// There is a garden of `n` flowers, and each flower has an integer beauty value.
// The flowers are arranged in a line. You are given an integer array `flowers` of size `n` and each `flowers[i]` represents the beauty of the `ith` flower.

// A garden is **valid** if it meets these conditions:

// *   The garden has at least two flowers.
// *   The first and the last flower of the garden have the same beauty value.

// As the appointed gardener, you have the ability to **remove** any (possibly none) flowers from the garden.
// You want to remove flowers in a way that makes the remaining garden **valid**. The beauty of the garden is the sum of the beauty of all the remaining flowers.

// Return the maximum possible beauty of some **valid** garden after you have removed any (possibly none) flowers.

// **Example 1:**

// **Input:** flowers = \[1,2,3,1,2\]
// **Output:** 8
// **Explanation:** You can produce the valid garden \[2,3,1,2\] to have a total beauty of 2 + 3 + 1 + 2 = 8.

// **Example 2:**

// **Input:** flowers = \[100,1,1,-3,1\]
// **Output:** 3
// **Explanation:** You can produce the valid garden \[1,1,1\] to have a total beauty of 1 + 1 + 1 = 3.

// **Example 3:**

// **Input:** flowers = \[-1,-2,0,-1\]
// **Output:** -2
// **Explanation:** You can produce the valid garden \[-1,-1\] to have a total beauty of -1 + -1 = -2.

// **Constraints:**

// *   `2 <= flowers.length <= 105`
// *   `-104 <= flowers[i] <= 104`
// *   It is possible to create a valid garden by removing some (possibly none) flowers.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

// int maximum_beauty(int[] flowers) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn maximum_beauty(flowers: Vec<i32>) -> i32 {
        let mut lookup = std::collections::HashMap::new();
        let mut prefix = vec![0];
        let mut ans = i32::MIN;
        for (i, f) in flowers.into_iter().enumerate() {
            let last = prefix[prefix.len() - 1] + if f > 0 { f } else { 0 };
            prefix.push(last);
            if !lookup.contains_key(&f) {
                lookup.insert(f, i);
                continue;
            }
            ans = ans.max(prefix[i + 1] - prefix[lookup[&f]] + if f < 0 { f * 2 } else { 0 });
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_maximum_beauty_1() {
        assert_eq!(8, Solution::maximum_beauty(vec![1, 2, 3, 1, 2],));
    }
    #[test]
    pub fn test_maximum_beauty_2() {
        assert_eq!(3, Solution::maximum_beauty(vec![100, 1, 1, -3, 1],));
    }
    #[test]
    pub fn test_maximum_beauty_3() {
        assert_eq!(-2, Solution::maximum_beauty(vec![-1, -2, 0, -1],));
    }
}
