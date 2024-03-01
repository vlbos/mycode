// 1950\. Maximum of Minimum Values in All Subarrays[](https://leetcode.ca/2021-08-11-1950-Maximum-of-Minimum-Values-in-All-Subarrays/#1950-maximum-of-minimum-values-in-all-subarrays)
// ====================================================================================================================================================================================

// Level[](https://leetcode.ca/2021-08-11-1950-Maximum-of-Minimum-Values-in-All-Subarrays/#level)
// ----------------------------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-08-11-1950-Maximum-of-Minimum-Values-in-All-Subarrays/#description)
// ----------------------------------------------------------------------------------------------------------

// You are given an integer array `nums` of size `n`. You are asked to solve `n` queries for each integer `i` in the range `0 <= i < n`.

// To solve the `i-th` query:

// 1.  Find the **minimum value** in each possible subarray of size `i + 1` of the array `nums`.
// 2.  Find the **maximum** of those minimum values. This maximum is the **answer** to the query.

// Return _a **0-indexed** integer array `ans` of size `n` such that `ans[i]` is the answer to the `i-th` query_.

// A **subarray** is a contiguous sequence of elements in an array.

// **Example 1:**

// **Input:** nums = \[0,1,2,4\]

// **Output:** \[4,2,1,0\]

// **Explanation:** i=0:

// *   The subarrays of size 1 are \[0\], \[1\], \[2\], \[4\]. The minimum values are 0, 1, 2, 4.
// *   The maximum of the minimum values is 4.

// i=1:

// *   The subarrays of size 2 are \[0,1\], \[1,2\], \[2,4\]. The minimum values are 0, 1, 2.
// *   The maximum of the minimum values is 2.

// i=2:

// *   The subarrays of size 3 are \[0,1,2\], \[1,2,4\]. The minimum values are 0, 1.
// *   The maximum of the minimum values is 1.

// i=3:

// *   There is one subarray of size 4, which is \[0,1,2,4\]. The minimum value is 0.
// *   There is only one value, so the maximum is 0.

// **Example 2:**

// **Input:** nums = \[10,20,50,10\]

// **Output:** \[50,20,10,10\]

// **Explanation:** i=0:

// *   The subarrays of size 1 are \[10\], \[20\], \[50\], \[10\]. The minimum values are 10, 20, 50, 10.
// *   The maximum of the minimum values is 50.

// i=1:

// *   The subarrays of size 2 are \[10,20\], \[20,50\], \[50,10\]. The minimum values are 10, 20, 10.
// *   The maximum of the minimum values is 20.

// i=2:

// *   The subarrays of size 3 are \[10,20,50\], \[20,50,10\]. The minimum values are 10, 10.
// *   The maximum of the minimum values is 10.

// i=3:

// *   There is one subarray of size 4, which is \[10,20,50,10\]. The minimum value is 10.
// *   There is only one value, so the maximum is 10.

// **Constraints:**

// *   `n == nums.length`
// *   `1 <= n <= 10^5`
// *   `0 <= nums[i] <= 10^9`

// Solution[](https://leetcode.ca/2021-08-11-1950-Maximum-of-Minimum-Values-in-All-Subarrays/#solution)
// ----------------------------------------------------------------------------------------------------

// Use monotonic stack, where the elements of the corresponding indices are ascending from bottom to top of the stack. Use two arrays `left` and `right` to store each element’s closest left index and right index such that the elements at the indices are less than the current element. Loop over `nums` from left to right, and fill in the arrays `left` and `right`.

// Then loop over `0 <= i < n`. For each `i`, let `num = nums[i]`, and the range length with `num` as the smallest element is `range = right[i] - left[i] - 1`. Let `ans[range - 1]` be the maximum of such values of `num`. Then for `i` from `n - 2` to 0, `ans[i]` is the maximum of `ans[i]` and `ans[i + 1]`. Finally, return `ans`.

//     class Solution {
//         public int[] find_maximums(int[] nums) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn find_maximums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (mut left, mut right) = (vec![0; n], vec![n as i32; n]);
        let mut min = i32::MAX;
        let mut s = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            min = min.min(num);
            while !s.is_empty() && nums[s[s.len() - 1]] >= num {
                right[s.pop().unwrap()] = i as i32;
            }
            left[i] = if let Some(&j) = s.last() {
                j as i32
            } else {
                -1
            };
            s.push(i);
        }
        let mut maximums = vec![min; n];
        for (i, &num) in nums.iter().enumerate() {
            let range = right[i] - left[i] - 1;
            maximums[range as usize - 1] = maximums[range as usize - 1].max(num);
        }
        for i in (0..n - 1).rev() {
            maximums[i] = maximums[i].max(maximums[i + 1]);
        }
        maximums
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // [10,2,8,3,2,7,6,4,10,8]
    // 输出：
    // [10,8,2,4,4,2,2,2,2,2]
    // 预期结果：
    // [10,8,4,4,4,2,2,2,2,2]
    #[test]
    pub fn test_find_maximums_1() {
        assert_eq!(vec![4, 2, 1, 0], Solution::find_maximums(vec![0, 1, 2, 4]));
    }
    #[test]
    pub fn test_find_maximums_2() {
        assert_eq!(
            vec![50, 20, 10, 10],
            Solution::find_maximums(vec![10, 20, 50, 10])
        );
    }
}
