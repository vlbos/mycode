// 1121\. Divide Array Into Increasing Sequences
// =============================================

// Given a **non-decreasing** array of positive integers `nums` and an integer `K`,
// find out if this array can be divided into one or more **disjoint increasing subsequences of length at least** `K`.

// **Example 1:**

// **Input:** nums = \[1,2,2,3,3,4,4\], K = 3
// **Output:** true
// **Explanation:**
// The array can be divided into the two subsequences \[1,2,3,4\] and \[2,3,4\] with lengths at least 3 each.

// **Example 2:**

// **Input:** nums = \[5,6,6,7,8\], K = 3
// **Output:** false
// **Explanation:**
// There is no way to divide the array using the conditions required.

// **Note:**

// 1.  `1 <= nums.length <= 10^5`
// 2.  `1 <= K <= nums.length`
// 3.  `1 <= nums[i] <= 10^5`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn   can_divide_into_subsequences(nums: Vec<i32>, k: i32) -> bool {
        let (mut cur, mut groups) = (1, 1);
        for num in nums.windows(2) {
            cur = if num[0] < num[1] { 1 } else { cur + 1 };
            groups = groups.max(cur);
        }
        (nums.len() as i32) >= k * groups
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_can_divide_into_subsequences_1() {
        assert!(Solution::can_divide_into_subsequences(
            vec![1, 2, 2, 3, 3, 4, 4],
            3
        ));
    }

    #[test]
   pub fn  test_can_divide_into_subsequences_2() {
        assert!(!Solution::can_divide_into_subsequences(
            vec![5, 6, 6, 7, 8],
            3
        ));
    }
}
