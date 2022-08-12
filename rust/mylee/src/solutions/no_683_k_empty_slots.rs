// 683\. K Empty Slots
// ===================

// You have `N` bulbs in a row numbered from `1` to `N`. Initially, all the bulbs are turned off. We turn on exactly one bulb everyday until all bulbs are on after `N` days.

// You are given an array `bulbs` of length `N` where `bulbs[i] = x` means that on the `(i+1)th` day, we will turn on the bulb at position `x` where `i` is `0-indexed` and `x` is `1-indexed.`

// Given an integer `K`, find out the **minimum day number** such that there exists two **turned on** bulbs that have **exactly** `K` bulbs between them that are **all turned off**.

// If there isn't such day, return `-1`.

// **Example 1:**

// **Input:**
// bulbs: \[1,3,2\]
// K: 1
// **Output:** 2
// **Explanation:**
// On the first day: bulbs\[0\] = 1, first bulb is turned on: \[1,0,0\]
// On the second day: bulbs\[1\] = 3, third bulb is turned on: \[1,0,1\]
// On the third day: bulbs\[2\] = 2, second bulb is turned on: \[1,1,1\]
// We return 2 because on the second day, there were two on bulbs with one off bulb between them.

// **Example 2:**

// **Input:**
// bulbs: \[1,2,3\]
// K: 1
// **Output:** -1

// **Note:**

// 1.  `1 <= N <= 20000`
// 2.  `1 <= bulbs[i] <= N`
// 3.  `bulbs` is a permutation of numbers from `1` to `N`.
// 4.  `0 <= K <= 20000`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
// use std::collections::BTreeSet;
// use std::ops::Bound::{Excluded, Unbounded};

impl Solution {
    pub fn   k_empty_slots(bulbs: Vec<i32>, k: i32) -> i32 {
        // let mut opened = BTreeSet::<i32>::new();
        // for i in 0..bulbs.len() {
        //     let index = bulbs[i] - 1;
        //     let day = i as i32 + 1;
        //     opened.insert(index);
        //     let before = opened.range((Unbounded, Excluded(&index))).rev().next();
        //     let after = opened.range((Excluded(&index), Unbounded)).next();
        //     if let Some(&b) = before {
        //         if i32::abs(index - b) == k + 1 {
        //             return day;
        //         }
        //     }
        //     if let Some(&a) = after {
        //         if i32::abs(index - a) == k + 1 {
        //             return day;
        //         }
        //     }
        // }
        // -1
        let mut days = vec![0; bulbs.len()];
        for (i, &v) in bulbs.iter().enumerate() {
            days[v as usize - 1] = i as i32 + 1;
        }
        let k = k as usize;
        let (mut left, mut right) = (0, k + 1);
        let mut ans = i32::MAX;
        let mut i = 0;
        while right < days.len() {
            if days[i] < days[left] || days[i] < days[right] || i == right {
                if i == right {
                    ans = ans.min(days[left].max(days[right]));
                }
                left = i;
                right = i + k + 1;
            }
            i += 1;
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_k_empty_slots_1() {
        assert_eq!(Solution::k_empty_slots(vec![1, 3, 2], 1), 2);
    }

    #[test]
   pub fn  test_k_empty_slots_2() {
        assert_eq!(Solution::k_empty_slots(vec![1, 2, 3], 1), -1);
    }
}
