// # [2495. Number of Subarrays Having Even Product](https://leetcode.com/problems/number-of-subarrays-having-even-product)
// ## Description

//  Given a  0-indexed  integer array  nums , return  the number of <span data-keyword="subarray-nonempty">subarrays  of   nums   having an even product .

//  Example 1:

//  Input:  nums = [9,6,7,13]
//  Output:  6
//  Explanation:  There are 6 subarrays with an even product:
// - nums[0..1] = 9 * 6 = 54.
// - nums[0..2] = 9 * 6 * 7 = 378.
// - nums[0..3] = 9 * 6 * 7 * 13 = 4914.
// - nums[1..1] = 6.
// - nums[1..2] = 6 * 7 = 42.
// - nums[1..3] = 6 * 7 * 13 = 546.

//  Example 2:

//  Input:  nums = [7,3,5]
//  Output:  0
//  Explanation:  There are no subarrays with an even product.

//   Constraints:

// 	  1 <= nums.length <= 10^5
// 	  1 <= nums[i] <= 10^5
//   long long even_product(vector<int>& nums) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn even_product(nums: Vec<i32>) -> i64 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_even_product_1() {
        assert_eq!(6, Solution::even_product(vec![9, 6, 7, 13]));
    }
    #[test]
    pub fn test_even_product_2() {
        assert_eq!(0, Solution::even_product(vec![7, 3, 5]));
    }
}
