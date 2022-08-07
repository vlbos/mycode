// 1060\. Missing Element in Sorted Array
// ======================================

// Given a sorted array `A` of **unique** numbers, find the `_K_-th` missing number starting from the leftmost number of the array.

// **Example 1:**

// **Input:** A = \[4,7,9,10\], K = 1
// **Output:** 5
// **Explanation:**
// The first missing number is 5.

// **Example 2:**

// **Input:** A = \[4,7,9,10\], K = 3
// **Output:** 8
// **Explanation:**
// The missing numbers are \[5,6,8,...\], hence the third missing number is 8.

// **Example 3:**

// **Input:** A = \[1,2,4\], K = 3
// **Output:** 6
// **Explanation:**
// The missing numbers are \[3,5,6,7,...\], hence the third missing number is 6.

// **Note:**

// 1.  `1 <= A.length <= 50000`
// 2.  `1 <= A[i] <= 1e7`
// 3.  `1 <= K <= 1e8`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)
pub struct Solution {}
impl Solution {
    pub fn missing_element(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = nums[0] + 1;
        let mut cnt = 0;
        for w in nums.windows(2) {
            cnt += w[1] - w[0] - 1;
            if cnt >= k {
                ans = w[1] + k - cnt - 1;
                break;
            }
        }
        if cnt < k {
            ans = nums[nums.len() - 1] + k - cnt;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_missing_element_1() {
        assert_eq!(5, Solution::missing_element(vec![4, 7, 9, 10], 1));
    }
    #[test]
    fn test_missing_element_2() {
        assert_eq!(8, Solution::missing_element(vec![4, 7, 9, 10], 3));
    }
    #[test]
    fn test_missing_element_3() {
        assert_eq!(6, Solution::missing_element(vec![1, 2, 4], 3));
    }
}
