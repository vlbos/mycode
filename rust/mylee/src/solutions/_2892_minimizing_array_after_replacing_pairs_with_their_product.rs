// # [2892. Minimizing Array After Replacing Pairs With Their Product](https://leetcode.com/problems/minimizing-array-after-replacing-pairs-with-their-product)

// ## Description

// Given an integer array nums and an integer k, you can perform the following operation on the array any number of times:

//
// 	Select two adjacent elements of the array like x and y, such that x * y  <= k,
//  and replace both of them with a single element with value x * y
// (e.g. in one operation the array [1, 2, 2, 3] with k = 5 can become [1, 4, 3] or [2, 2, 3],
// but can 't become [1, 2, 6]).
//

// Return the minimum possible length of nums after any number of operations.

//
// Example 1:

//
// Input: nums = [2,3,3,7,3,5], k = 20
// Output: 3
// Explanation: We perform these operations:
// 1. [2,3,3,7,3,5] -> [6,3,7,3,5]
// 2. [6,3,7,3,5] -> [18,7,3,5]
// 3. [18,7,3,5] -> [18,7,15]
// It can be shown that 3 is the minimum length possible to achieve with the given operation.
//

// Example 2:

//
// Input: nums = [3,3,3,3], k = 6
// Output: 4
// Explanation: We can 't perform any operations since the product of every two adjacent elements is greater than 6.
// Hence, the answer is 4.

//
// Constraints:

//
// 	1  <= nums.length  <= 105
// 	0  <= nums[i]  <= 109
// 	1  <= k  <= 109
//

//     int min_array_length(vector<int>& nums, int k) {
//

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_array_lengthwrong(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let (mut ans, mut y) = (1, nums[0] as i64);
        for &x in &nums[1..] {
            if x == 0 {
                return 1;
            }
            let x = x as i64;
            if y * x <= k {
                y *= x;
            } else {
                y = x;
                ans += 1;
            }
        }
        ans
    }
    pub fn min_array_length(mut nums: Vec<i32>, k: i32) -> i32 {
        let (mut ans, mut i) = (0, 0);
        while i + 1 < nums.len() {
            if nums[i] as i64 * nums[i + 1] as i64 <= k as i64 {
                nums[i + 1] = nums[i] * nums[i + 1];
                if nums[i + 1] == 0 {
                    return 1;
                }
            } else {
                ans += 1;
            }
            i += 1;
        }
        if i != nums.len() {
            ans += 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // nums =
    // [3282,8828,214,5710,6096,7158,3092,3826,5765,5591,6918,3564,8210,3060,5879,8333,6300,8861,1968,3392,7894,3217,3719,1340,5102,2093,1618,2162,2080,7599,8108,7282,7622,3441,8674,1704,1406,2438,1738,8536,4206,3138,5465,6525,2975,1060,4544,6350,5334,7396,3581,3347,1825,3216,4470,4734,7661,7153,2405,7361,8200,7501,7981,4316,6819,469,8046,3530,3158,2290,2136,341,8125,6191,7742,3704,2463,3142,158,5252,3773,1807,7347,7502,6018,4672,5229,593,3008,5983,2049,4573,2947,4725,622,3481,3776,3866,2602,2728,3204,6380,8289,732,7489,134,3212,7990,2572,6143,2294,4605,6563,6610,4554,1840,2521,2075,6469,6454,4564,2973,5743,8423,8981,5932,5622,6660,5536,7085,399,3854,5415,5936,2340,5167,1738,3134,2963,6913,5548,1814,6533,6865,5227,2033,6159,8471,2361,8128,6645,4385,6150,2123,5802,2240,6892,106,392,5651,4922,3010,5251,6497,727,7934,358,7278,653,5704,7090,1220,5211,3961,94,2403,6775,7725,4331,1825,5061,4498,1134,2022,1337,6180,8877,6204,8430,910,3973,3063,3007,4981,8212,6836,5996,6950,4929,163,5494,5358,7919,61...
    // View less
    // k =
    // 899913

    // Use Testcase
    // Output
    // 234
    // Expected
    // 239
    #[test]
    pub fn test_min_array_length_1() {
        assert_eq!(3, Solution::min_array_length(vec![2, 3, 3, 7, 3, 5], 20));
    }
    #[test]
    pub fn test_min_array_length_2() {
        assert_eq!(4, Solution::min_array_length(vec![3, 3, 3, 3], 6));
    }
}
