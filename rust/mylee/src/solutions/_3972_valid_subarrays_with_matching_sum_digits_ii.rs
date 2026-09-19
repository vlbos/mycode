// # [3972. Valid Subarrays With Matching Sum Digits II ]

// ## Description

// You are given an integer array `nums` and an integer digit `x`.

// A **subarray** `nums[l..r]` is considered **valid** if the sum of its elements satisfies both of the following conditions:

// - The first digit of the sum is equal to `x`.
// - The last digit of the sum is equal to `x`.

// Return the number of valid subarrays.

// **Example 1:**

// **Input:** nums = [1,100,1], x = 1

// **Output:** 4

// **Explanation:**

// The valid subarrays are:

// - `nums[0..0]`: `sum = 1`
// - `nums[0..1]`: `sum = 1 + 100 = 101`
// - `nums[1..2]`: `sum = 100 + 1 = 101`
// - `nums[2..2]`: `sum = 1`

// Thus, the answer is 4.

// **Example 2:**

// **Input:** nums = [1], x = 2

// **Output:** 0

// **Explanation:**

// The only subarray is `nums[0..0]` with a sum of 1, which does not satisfy the conditions.

// Thus, the answer is 0.

// **Constraints:**

// - `1 <= nums.length <= 105`
// - `1 <= nums[i] <= 109`
// - `1 <= x <= 9`

//  long long count_valid_subarrays(vector<int>& nums, int x) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_valid_subarrays(mut nums: Vec<i32>, x: i32) -> i64 {
        let n = nums.len();
        let mut pre = vec![0; n + 1];
        for i in 0..n {
            pre[i + 1] = pre[i] + nums[i] as i64;
        }
        let (last, x) = (pre[n], x as i64);
        let (mut ans, mut base) = (0, 1);
        while x * base <= last {
            let (mut cnt, mut left, mut right) = ([0; 10], 0, 0);
            for i in 0..n {
                let xb = pre[i + 1] - x * base;
                while pre[right] <= xb {
                    cnt[pre[right] as usize % 10] += 1;
                    right += 1;
                }
                let xb = pre[i + 1] - (x + 1) * base;
                while pre[left] <= xb {
                    cnt[pre[left] as usize % 10] -= 1;
                    left += 1;
                }
                ans += cnt[(pre[i + 1] - x) as usize % 10]
            }
            base *= 10;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_count_valid_subarrays_1() {
        assert_eq!(4, Solution::count_valid_subarrays(vec![1, 100, 1], 1),);
    }
    #[test]
    pub fn test_count_valid_subarrays_2() {
        assert_eq!(0, Solution::count_valid_subarrays(vec![1], 2));
    }
}
