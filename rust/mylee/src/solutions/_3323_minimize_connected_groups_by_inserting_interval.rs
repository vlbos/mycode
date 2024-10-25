// [3323\. Minimize Connected Groups by Inserting Interval 🔒](https://leetcode.com/problems/minimize-connected-groups-by-inserting-interval)
// ==========================================================================================================================================



// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// Description
// -----------

// You are given a 2D array `intervals`, where `intervals[i] = [starti, endi]` represents the start and the end of interval `i`.
//  You are also given an integer `k`.

// You must add **exactly one** new interval `[startnew, endnew]` to the array such that:

// *   The length of the new interval, `endnew - startnew`, is at most `k`.
// *   After adding, the number of **connected groups** in `intervals` is **minimized**.

// A **connected group** of intervals is a maximal collection of intervals that, 
// when considered together, cover a continuous range from the smallest point to the largest point with no gaps between them. 
// Here are some examples:

// *   A group of intervals `[[1, 2], [2, 5], [3, 3]]` is connected because together they cover the range from 1 to 5 without any gaps.
// *   However, a group of intervals `[[1, 2], [3, 4]]` is not connected because the segment `(2, 3)` is not covered.

// Return the **minimum** number of connected groups after adding **exactly one** new interval to the array.

// **Example 1:**

// **Input:** intervals = \[\[1,3\],\[5,6\],\[8,10\]\], k = 3

// **Output:** 2

// **Explanation:**

// After adding the interval `[3, 5]`, we have two connected groups: `[[1, 3], [3, 5], [5, 6]]` and `[[8, 10]]`.

// **Example 2:**

// **Input:** intervals = \[\[5,10\],\[1,1\],\[3,3\]\], k = 1

// **Output:** 3

// **Explanation:**

// After adding the interval `[1, 1]`, we have three connected groups: `[[1, 1], [1, 1]]`, `[[3, 3]]`, and `[[5, 10]]`.

// **Constraints:**

// *   `1 <= intervals.length <= 105`
// *   `intervals[i] == [starti, endi]`
// *   `1 <= starti <= endi <= 109`
// *   `1 <= k <= 109`

// int min_connected_groups(vector<vector<int>>& intervals, int k) 

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_connected_groups(mut intervals: Vec<Vec<i32>>,  k: i32) -> i32 {
        intervals.sort_unstable();
        let mut merged=vec![intervals[0].clone()];
        for interval in intervals{
            if merged.last().unwrap()[1]<interval[0]{
                merged.push(interval);
            }else{
                merged.last_mut().unwrap()[1]=merged.last().unwrap()[1].max(interval[1]);
            }
        }
        let n=merged.len();
        let mut ans=n;
        for (i,v) in merged.iter().enumerate(){
            let j=merged.partition_point(|x|x<&vec![v[1]+k+1,0]);
            ans=ans.min(n-(j-i-1));
        }
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_min_connected_groups_1() {
        assert_eq!(
            2,
            Solution::min_connected_groups(lc_matrix![[1,3],[5,6],[8,10]], 3),
        );
    }
    #[test]
    pub fn test_min_connected_groups_2() {
        assert_eq!(3, Solution::min_connected_groups(lc_matrix![[5,10],[1,1],[3,3]], 1));
    }
}
