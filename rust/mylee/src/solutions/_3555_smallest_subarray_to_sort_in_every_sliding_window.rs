// ## [3555\. Smallest Subarray to Sort in Every Sliding Window 🔒](https://leetcode.com/problems/smallest-subarray-to-sort-in-every-sliding-window)

// ## Description

// You are given an integer array `nums` and an integer `k`.

// For each contiguous subarray of length `k`, determine the **minimum** length of a continuous segment that must be sorted so that the entire window becomes **non‑decreasing**;
//if the window is already sorted, its required length is zero.

// Return an array of length `n − k + 1` where each element corresponds to the answer for its window.

// **Example 1:**

// **Input:** nums = \[1,3,2,4,5\], k = 3

// **Output:** \[2,2,0\]

// **Explanation:**

// +   `nums[0...2] = [1, 3, 2]`. Sort `[3, 2]` to get `[1, 2, 3]`, the answer is 2.
// +   `nums[1...3] = [3, 2, 4]`. Sort `[3, 2]` to get `[2, 3, 4]`, the answer is 2.
// +   `nums[2...4] = [2, 4, 5]` is already sorted, so the answer is 0.

// **Example 2:**

// **Input:** nums = \[5,4,3,2,1\], k = 4

// **Output:** \[4,4\]

// **Explanation:**

// +   `nums[0...3] = [5, 4, 3, 2]`. The whole subarray must be sorted, so the answer is 4.
// +   `nums[1...4] = [4, 3, 2, 1]`. The whole subarray must be sorted, so the answer is 4.

// **Constraints:**

// +   `1 <= nums.length <= 1000`
// +   `1 <= k <= nums.length`
// +   `1 <= nums[i] <= 106`

//     // vector<int> min_subarray_sort(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn min_subarray_sort(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = vec![];
        for w in nums.windows(k as usize) {
            let mut sw = w.to_vec();
            sw.sort_unstable();
            let (l, r) = (
                sw.iter()
                    .zip(w)
                    .position(|(a, b)| a != b)
                    .unwrap_or(sw.len()),
                sw.iter().zip(w).rposition(|(a, b)| a != b).unwrap_or(0) + 1,
            );
            ans.push(r.saturating_sub(l) as i32);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_subarray_sort_1() {
        assert_eq!(
            vec![2, 2, 0],
            Solution::min_subarray_sort(vec![1, 3, 2, 4, 5], 3)
        );
    }
    #[test]
    pub fn test_min_subarray_sort_2() {
        assert_eq!(
            vec![4, 4],
            Solution::min_subarray_sort(vec![5, 4, 3, 2, 1], 4)
        );
    }
}
