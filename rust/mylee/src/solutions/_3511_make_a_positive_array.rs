// ## [3511\. Make a Positive Array 🔒](https://leetcode.com/problems/make-a-positive-array)

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// ## Description

// You are given an array `nums`.
// An array is considered **positive** if the sum of all numbers in each **subarray** with **more than two** elements is positive.

// You can perform the following operation any number of times:

// +   Replace **one** element in `nums` with any integer between -1018 and 1018.

// Find the **minimum** number of operations needed to make `nums` **positive**.

// **Example 1:**

// **Input:** nums = \[-10,15,-12\]

// **Output:** 1

// **Explanation:**

// The only subarray with more than 2 elements is the array itself.
// The sum of all elements is `(-10) + 15 + (-12) = -7`.
// By replacing `nums[0]` with 0, the new sum becomes `0 + 15 + (-12) = 3`. Thus, the array is now positive.

// **Example 2:**

// **Input:** nums = \[-1,-2,3,-1,2,6\]

// **Output:** 1

// **Explanation:**

// The only subarrays with more than 2 elements and a non-positive sum are:

// | Subarray Indices | Subarray | Sum | Subarray After Replacement (Set nums\[1\] = 1) | New Sum |
// | --- | --- | --- | --- | --- |
// | nums\[0...2\] | \[-1, -2, 3\] | 0 | \[-1, 1, 3\] | 3 |
// | nums\[0...3\] | \[-1, -2, 3, -1\] | \-1 | \[-1, 1, 3, -1\] | 2 |
// | nums\[1...3\] | \[-2, 3, -1\] | 0 | \[1, 3, -1\] | 3 |

// Thus, `nums` is positive after one operation.

// **Example 3:**

// **Input:** nums = \[1,2,3\]

// **Output:** 0

// **Explanation:**

// The array is already positive, so no operations are needed.

// **Constraints:**

// +   `3 <= nums.length <= 105`
// +   `-109 <= nums[i] <= 109`

//  int make_array_positive(vector<int>& nums) {

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn make_array_positive(nums: Vec<i32>) -> i32 {
        // let mut q=std::collections::BinaryHeap::new();
        // let (mut ans,mut pre)=(0,0);
        // for x in nums{
        //     pre+=x;
        //     if x<0{
        //         q.push(-x);
        //     }
        //     while pre<=0 && !q.is_empty(){
        //         pre+=2*q.pop().unwrap();
        //         ans+=1;
        //     }
        // }
        // ans
        let (mut l, mut ans, mut pre_mx, mut s) = (-1, 0, 0, 0);
        for (r, &x) in nums.iter().enumerate() {
            let rr = r as i32;
            s += x;
            if l + 2 < rr && s <= pre_mx {
                ans += 1;
                l = rr;
                (pre_mx, s) = (0, 0);
            } else if l + 2 <= rr {
                pre_mx = pre_mx.max(s - x - nums[r - 1]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_make_array_positive_1() {
        assert_eq!(1, Solution::make_array_positive(vec![-10, 15, -12]));
    }
    #[test]
    pub fn test_make_array_positive_2() {
        assert_eq!(1, Solution::make_array_positive(vec![-1, -2, 3, -1, 2, 6]));
    }
    #[test]
    pub fn test_make_array_positive_3() {
        assert_eq!(0, Solution::make_array_positive(vec![1, 2, 3]));
    }
}
