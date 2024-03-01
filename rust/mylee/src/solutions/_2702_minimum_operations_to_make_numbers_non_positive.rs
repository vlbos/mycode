// # [2702. Minimum Operations to Make Numbers Non-positive](https://leetcode.com/problems/minimum-operations-to-make-numbers-non-positive)

// ## Description

//  You are given a  0-indexed  integer array  nums  and two integers  x  and  y .
// In one operation, you must choose an index  i  such that  0  <= i  < nums.length  and perform the following:

// 	 Decrement  nums[i]  by  x .
// 	 Decrement values by  y  at all indices except the  i th   one.

//  Return  the minimum number of operations to make all the integers in   nums    less than or equal to zero.

//   ### Example 1:

//  Input:  nums = [3,4,1,7,6], x = 4, y = 2
//  Output:  3
//  Explanation:  You will need three operations. One of the optimal sequence of operations is:
// Operation 1: Choose i = 3. Then, nums = [1,2,-1,3,4].
// Operation 2: Choose i = 3. Then, nums = [-1,0,-3,-1,2].
// Operation 3: Choose i = 4. Then, nums = [-3,-2,-5,-3,-2].
// Now, all the numbers in nums are non-positive. Therefore, we return 3.

//   ### Example 2:

//  Input:  nums = [1,2,1], x = 2, y = 1
//  Output:  1
//  Explanation:  We can perform the operation once on i = 1. Then, nums becomes [0,0,0].
// All the positive numbers are removed, and therefore, we return 1.

//   Constraints:

// 	  1  <= nums.length  <= 10^5
// 	  1  <= nums[i]  <= 10^9
// 	  1  <= y  < x  <= 10^9

// ## Solutions

//  int min_operations(vector<int>& nums, int x, int y) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, y: i32) -> i32 {
        let (mut l, mut r, x, y) = (0, *nums.iter().max().unwrap() as i64, x as i64, y as i64);
        let is_possible = |m: i64| {
            let mut ans = 0;
            for &num in &nums {
                ans += 0i64.max((num as i64 - y * m + x - y - 1) / (x - y));
            }
            ans <= m
        };
        while l < r {
            let m = (l + r) / 2;
            if is_possible(m) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_operations_1() {
        assert_eq!(3, Solution::min_operations(vec![3, 4, 1, 7, 6], 4, 2));
    }
    #[test]
    pub fn test_min_operations_2() {
        assert_eq!(1, Solution::min_operations(vec![1, 2, 1], 2, 1));
    }
}
