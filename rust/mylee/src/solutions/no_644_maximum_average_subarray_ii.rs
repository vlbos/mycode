// 644\. Maximum Average Subarray II
// =================================

// Given an array consisting of `n` integers, find the contiguous subarray whose **length is greater than or equal to** `k` that has the maximum average value.
// And you need to output the maximum average value.

// **Example 1:**

// **Input:** \[1,12,-5,-6,50,3\], k = 4
// **Output:** 12.75
// **Explanation:**
// when length is 5, maximum average value is 10.8,
// when length is 6, maximum average value is 9.16667.
// Thus return 12.75.

// **Note:**

// 1.  1 <= `k` <= `n` <= 10,000.
// 2.  Elements of the given array will be in range \[-10,000, 10,000\].
// 3.  The answer with the calculation error less than 10\-5 will be accepted.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @star
// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        // let len = nums.len();
        // let k = k as usize;
        // if len == 0 || k > len || k == 0 {
        //     return 0f64;
        // }
        // let mut min_val = nums.iter().cloned().min().unwrap() as f64;
        // let mut max_val = nums.iter().cloned().max().unwrap() as f64;
        // let mut prev_mid = max_val;
        // let mut error = i32::max_value() as f64;
        // while error > 0.00001 {
        //     let mid = (min_val + max_val) / 2.0;
        //     if Solution::has_k_more_average(&nums, mid, k) {
        //         min_val = mid;
        //     } else {
        //         max_val = mid;
        //     }
        //     error = f64::abs(prev_mid - mid);
        //     prev_mid = mid;
        // }
        // min_val
        let k = k as usize;
        let (mut left, mut right) = (
            *nums.iter().min().unwrap() as f64,
            *nums.iter().max().unwrap() as f64,
        );
        while right - left > 1e-5 {
            let (mut min_sum, mut pre_sum, mut sum) = (0.0f64, 0.0f64, 0.0f64);
            let mid = left + (right - left) / 2.0;
            let mut check = false;
            for (i, &v) in nums.iter().enumerate() {
                sum += v as f64 - mid;
                if i >= k {
                    pre_sum += nums[i - k] as f64 - mid;
                    min_sum = min_sum.min(pre_sum);
                }
                if i + 1 >= k && sum > min_sum {
                    check = true;
                    break;
                }
            }
            if check {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }

    // fn has_k_more_average(nums: &[i32], ave: f64, k: usize) -> bool {
    //     let mut min_sum = 0f64;
    //     let mut sum = 0f64;
    //     for i in 0..k {
    //         sum += (nums[i] as f64) - ave;
    //     }
    //     if sum > 0f64 {
    //         return true;
    //     }
    //     let mut prev = 0f64;
    //     for i in k..nums.len() {
    //         sum += (nums[i] as f64) - ave;
    //         prev += (nums[i - k] as f64) - ave;
    //         min_sum = f64::min(prev, min_sum);
    //         if sum - min_sum > 0.0 {
    //             return true;
    //         }
    //     }
    //     false
    // }
}
// @lc code=end

#[allow(dead_code)]
pub  struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_feq;

    #[test]
    fn test_find_max_average_1() {
        assert_feq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75,
            1e-5
        );
    }
}
