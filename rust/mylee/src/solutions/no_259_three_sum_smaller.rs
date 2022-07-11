// 259\. 3Sum Smaller
// ==================

// Given an array of _n_ integers _nums_ and a _target_, find the number of index triplets `i, j, k` with `0 <= i < j < k < n` that satisfy the condition `nums[i] + nums[j] + nums[k] < target`.

// **Example:**

// **Input:** _nums_ = `[-2,0,1,3]`, and _target_ = 2
// **Output:** 2
// **Explanation:** Because there are two triplets which sums are less than 2:
//              \[-2,0,1\]
//              \[-2,0,3\]

// **Follow up:** Could you solve it in _O_(_n_2) runtime?

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google) [IBM](https://leetcode.ca/tags/#IBM) [Mathworks](https://leetcode.ca/tags/#Mathworks)
// @lc code=start
impl Solution {
    pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        if nums.len() < 3 {
            return 0;
        }
        let mut count = 0i32;
        for i in (2..nums.len()).rev() {
            let threshold = target - nums[i];
            let mut j = 0;
            let mut k = i - 1;
            while j < k {
                if nums[j] + nums[k] < threshold {
                    count += (k - j) as i32;
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        count
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_three_sum_smaller() {
        let nums = vec![-2, 0, 1, 3];
        assert_eq!(Solution::three_sum_smaller(nums, 4), 3);
    }
}
