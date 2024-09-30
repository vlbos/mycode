// 3284. Sum of Consecutive Subarrays

// Medium

// Hint

// We call an array `arr` of length `n` **consecutive** if one of the following holds:

// *   `arr[i] - arr[i - 1] == 1` for _all_ `1 <= i < n`.
// *   `arr[i] - arr[i - 1] == -1` for _all_ `1 <= i < n`.

// The **value** of an array is the sum of its elements.

// For example, `[3, 4, 5]` is a consecutive array of value 12 and `[9, 8]` is another of value 17. While `[3, 4, 3]` and `[8, 6]` are not consecutive.

// Given an array of integers `nums`, return the _sum_ of the **values** of all **consecutive** 

// subarrays

// .

// Since the answer may be very large, return it **modulo** `109 + 7.`

// **Note** that an array of length 1 is also considered consecutive.

// **Example 1:**

// **Input:** nums = \[1,2,3\]

// **Output:** 20

// **Explanation:**

// The consecutive subarrays are: `[1]`, `[2]`, `[3]`, `[1, 2]`, `[2, 3]`, `[1, 2, 3]`.  
// Sum of their values would be: `1 + 2 + 3 + 3 + 5 + 6 = 20`.

// **Example 2:**

// **Input:** nums = \[1,3,5,7\]

// **Output:** 16

// **Explanation:**

// The consecutive subarrays are: `[1]`, `[3]`, `[5]`, `[7]`.  
// Sum of their values would be: `1 + 3 + 5 + 7 = 16`.

// **Example 3:**

// **Input:** nums = \[7,6,1,2\]

// **Output:** 32

// **Explanation:**

// The consecutive subarrays are: `[7]`, `[6]`, `[1]`, `[2]`, `[7, 6]`, `[1, 2]`.  
// Sum of their values would be: `7 + 6 + 1 + 2 + 13 + 3 = 32`.

// **Constraints:**

// *   `1 <= nums.length <= 105`
// *   `1 <= nums[i] <= 105`
#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn get_sum(nums: Vec<i32>) -> i32 {
 let (mut ans,mut cur,mut prior,mut l)=(nums[0] as i64,nums[0] as i64,0,0);
        for (r,&x) in nums.iter().enumerate().skip(1){
            let diff=x-nums[r-1];
            if diff.abs()!=1{
                prior=0;
                l=r;
                cur=0;
            }else if prior!=diff{
                cur=nums[r-1] as i64;
                l=r-1;
                prior=diff;
            }
            cur+=x as i64*(r-l+1) as i64;
            ans=(ans+cur)%1_000_000_007;
        }
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_get_sum_1() {
        assert_eq!(20, Solution::get_sum(vec![1,2,3]));
    }
    #[test]
    pub fn test_get_sum_2() {
        assert_eq!(16, Solution::get_sum(vec![1,3,5,7]));
    }
    #[test]
    pub fn test_get_sum_3() {
        assert_eq!(32, Solution::get_sum(vec![7,6,1,2]));
    }
}