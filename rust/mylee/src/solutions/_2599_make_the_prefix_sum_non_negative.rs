// # [2599. Make the Prefix Sum Non-negative](https://leetcode.com/problems/make-the-prefix-sum-non-negative)

// ## Description

//  You are given a  0-indexed  integer array  nums . You can apply the following operation any number of times:

// 	 Pick any element from  nums  and put it at the end of  nums .

//  The prefix sum array of  nums  is an array  prefix  of the same length as  nums
// such that  prefix[i]  is the sum of all the integers  nums[j]  where  j  is in the inclusive range  [0, i] .

//  Return  the minimum number of operations such that the prefix sum array does not contain negative integers .
// The test cases are generated such that it is always possible to make the prefix sum array non-negative.

//   ### Example 1:

//  Input:  nums = [2,3,-5,4]
//  Output:  0
//  Explanation:  we do not need to do any operations.
// The array is [2,3,-5,4]. The prefix sum array is [2, 5, 0, 4].

//   ### Example 2:

//  Input:  nums = [3,-5,-2,6]
//  Output:  1
//  Explanation:  we can do one operation on index 1.
// The array after the operation is [3,-2,6,-5]. The prefix sum array is [3, 1, 7, 2].

//   Constraints:

// 	  1  <= nums.length  <= 10^5
// 	  -10^9   <= nums[i]  <= 10^9

// ## Solutions

// **Method 1: Greedy + Priority Queue (Min Heap)**

// We use a variable $s$ to record the prefix sum of the current array.

// Traverse the array $nums$, add the current element $x$ to the prefix sum $s$. If $x$ is a negative number, add $x$ to the min heap. If $s$ is negative at this time, greedily take out the smallest negative number and subtract it from $s$, and add one to the answer. Finally, return the answer.

// The time complexity is $O(n \times \log n)$, and the space complexity is $O(n)$, where $n$ is the length of the array $nums$.

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn make_pref_sum_non_negative(nums: Vec<i32>) -> i32 {
        let mut pre = 0;
        let mut q = std::collections::BinaryHeap::new();
        let mut ans = 0;
        for &num in &nums {
            pre += num;
            if num < 0 {
                q.push(-num);
            }
            while pre < 0 {
                pre += q.pop().unwrap();
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_make_pref_sum_non_negative_1() {
        assert_eq!(0, Solution::make_pref_sum_non_negative(vec![2, 3, -5, 4]));
    }
    #[test]
    pub fn test_make_pref_sum_non_negative_2() {
        assert_eq!(1, Solution::make_pref_sum_non_negative(vec![3, -5, -2, 6]));
    }
}
