// 3874. Valid Subarrays With Exactly One Peak
// ## Description
// You are given an integer array `nums` of length `n` and an integer `k`.
// An index `i` is a **peak** if:
// * `0 \< i \< n - 1`
// * `nums[i] \> nums[i - 1]` and `nums[i] \> nums[i + 1]`
// A subarray `[l, r]` is **valid** if:
// * It contains **exactly one** peak at index `i` from `nums`
// * `i - l \<= k` and `r - i \<= k`
// Return an integer denoting the number of **valid subarrays** in `nums`.
// A **subarray** is a contiguous **non-empty** sequence of elements within an array.

// **Example 1:**
// **Input:** nums = [1,3,2], k = 1
// **Output:** 4
// **Explanation:**
// * Index `i = 1` is a peak because `nums[1] = 3` is greater than `nums[0] = 1` and `nums[2] = 2`.
// * Any valid subarray must include index 1, and the distance from the peak to both ends of the subarray must not exceed `k = 1`.
// * The valid subarrays are `[3]`, `[1, 3]`, `[3, 2]`, and `[1, 3, 2]`, so the answer is 4.
// **Example 2:**
// **Input:** nums = [7,8,9], k = 2
// **Output:** 0
// **Explanation:**
// * There is no index `i` such that `nums[i]` is greater than both `nums[i - 1]` and `nums[i + 1]`.
// * Therefore, the array contains no peak. Thus, the number of valid subarrays is 0.
// **Example 3:**
// **Input:** nums = [4,3,5,1], k = 2
// **Output:** 6
// **Explanation:**
// * Index `i = 2` is a peak because `nums[2] = 5` is greater than `nums[1] = 3` and `nums[3] = 1`.
// * Any valid subarray must contain this peak, and the distance from the peak to both ends of the subarray must not exceed `k = 2`.
// * The valid subarrays are `[5]`, `[3, 5]`, `[5, 1]`, `[3, 5, 1]`, `[4, 3, 5]`, and `[4, 3, 5, 1]`, so the answer is 6.

// **Constraints:**
// * `1 \<= n == nums.length \<= 105`
// * `-105 \<= nums[i] \<= 105`
// * `1 \<= k \<= n`

//   long long count_the_num_of_k_free_subsets(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_the_num_of_k_free_subsets(mut nums: Vec<i32>, k: i32) -> i64 {
        let (n, k) = (nums.len(), k as usize);
        let mut peak_indices = vec![];
        for (i, &x) in nums.iter().enumerate().skip(1).take(n - 2) {
            if (i == 0 || nums[i - 1] < x) && (i + 1 == n || x > nums[i + 1]) {
                peak_indices.push(i);
            }
        }
        let mut ans = 0;
        for (j, &p) in peak_indices.iter().enumerate() {
            let mut left_min = p.saturating_sub(k);
            if j > 0 {
                left_min = left_min.max(peak_indices[j - 1] + 1);
            }
            let mut right_max = (p + k).min(n - 1);
            if j + 1 < peak_indices.len() {
                right_max = right_max.min(peak_indices[j + 1] - 1);
            }
            ans += (p - left_min + 1) * (right_max - p + 1);
        }
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_the_num_of_k_free_subsets_1() {
        assert_eq!(
            4,
            Solution::count_the_num_of_k_free_subsets(vec![1, 3, 2], 1)
        );
    }
    #[test]
    pub fn test_count_the_num_of_k_free_subsets_2() {
        assert_eq!(
            0,
            Solution::count_the_num_of_k_free_subsets(vec![7, 8, 9], 2)
        );
    }
    #[test]
    pub fn test_count_the_num_of_k_free_subsets_3() {
        assert_eq!(
            6,
            Solution::count_the_num_of_k_free_subsets(vec![4, 3, 5, 1], 2)
        );
    }
}
