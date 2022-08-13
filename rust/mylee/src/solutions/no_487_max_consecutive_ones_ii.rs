// 487\. Max Consecutive Ones II
// =============================

// Given a binary array, find the maximum number of consecutive 1s in this array if you can flip at most one 0.

// **Example 1:**

// **Input:** \[1,0,1,1,0\]
// **Output:** 4
// **Explanation:** Flip the first zero will get the the maximum number of consecutive 1s.
//     After flipping, the maximum number of consecutive 1s is 4.

// **Note:**

// *   The input array will only contain `0` and `1`.
// *   The length of input array is a positive integer and will not exceed 10,000

// **Follow up:**
// What if the input numbers come in one by one as an **infinite stream**? In other words,
//  you can't store all numbers coming from the stream as it's too large to hold in memory. Could you solve it efficiently?

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        // nums.push(0);
        // let mut last_max_len = -1;
        // let mut curr_max_len = 0;
        // let mut sum_max_len = 0;
        // for n in nums {
        //     match n {
        //         1 => {
        //             curr_max_len += 1;
        //             sum_max_len = i32::max(sum_max_len, last_max_len + curr_max_len + 1);
        //         }
        //         0 => {
        //             sum_max_len = i32::max(sum_max_len, last_max_len + curr_max_len + 1);
        //             last_max_len = curr_max_len;
        //             curr_max_len = 0;
        //         }
        //         _ => unreachable!(),
        //     }
        // }
        // sum_max_len
        let mut ans = 0;
        let (mut i, mut j) = (0, 0);
        let mut digits = vec![0; 2];
        while j < nums.len() {
            digits[nums[j] as usize] += 1;
            if digits[0] > 1 {
                while i < j && digits[0] > 1 {
                    digits[nums[i] as usize] -= 1;
                    i += 1;
                }
            }
            ans = ans.max(j - i + 1);
            j += 1;
        }
        ans as _
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_find_max_consecutive_ones_1() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0]), 4);
    }

    #[test]
    pub fn test_find_max_consecutive_ones_2() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1]), 1);
    }
}
