// # [2464. Minimum Subarrays in a Valid Split](https://leetcode.com/problems/minimum-subarrays-in-a-valid-split)
// ## Description

//  You are given an integer array  nums .

//  Splitting of an integer array  nums  into  subarrays  is  valid  if:

// 	 the  greatest common divisor  of the first and last elements of each subarray is  greater  than  1 , and
// 	 each element of  nums  belongs to exactly one subarray.

//  Return  the  minimum  number of subarrays in a  valid  subarray splitting of   nums . If a valid subarray splitting is not possible, return  -1 .

//   Note  that:

// 	 The  greatest common divisor  of two numbers is the largest positive integer that evenly divides both numbers.
// 	 A  subarray  is a contiguous non-empty part of an array.

//  Example 1:

//  Input:  nums = [2,6,3,4,3]
//  Output:  2
//  Explanation:  We can create a valid split in the following way: [2,6] | [3,4,3].
// - The starting element of the 1 st  subarray is 2 and the ending is 6. Their greatest common divisor is 2, which is greater than 1.
// - The starting element of the 2 nd  subarray is 3 and the ending is 3. Their greatest common divisor is 3, which is greater than 1.
// It can be proved that 2 is the minimum number of subarrays that we can obtain in a valid split.

//  Example 2:

//  Input:  nums = [3,5]
//  Output:  2
//  Explanation:  We can create a valid split in the following way: [3] | [5].
// - The starting element of the 1 st  subarray is 3 and the ending is 3. Their greatest common divisor is 3, which is greater than 1.
// - The starting element of the 2 nd  subarray is 5 and the ending is 5. Their greatest common divisor is 5, which is greater than 1.
// It can be proved that 2 is the minimum number of subarrays that we can obtain in a valid split.

//  Example 3:

//  Input:  nums = [1,2,1]
//  Output:  -1
//  Explanation:  It is impossible to create valid split.

//   Constraints:

// 	  1 <= nums.length <= 1000
// 	  1 <= nums[i] <= 10^5
// int valid_subarray_split(vector<int>& nums) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn valid_subarray_split(nums: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        fn dfs(i: usize, nums: &Vec<i32>, f: &mut Vec<i32>) -> i32 {
            let n = nums.len();
            if i >= n {
                return 0;
            }
            if f[i] > 0 {
                return f[i];
            }
            let mut ans = i32::MAX;
            for j in i..n {
                if gcd(nums[i], nums[j]) > 1 {
                    ans = ans.min(1 + dfs(j + 1, nums, f));
                }
            }
            f[i] = ans;
            ans
        }
        let mut f = vec![0; nums.len()];
        let ans = dfs(0, &nums, &mut f);
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_valid_subarray_split_1() {
        assert_eq!(2, Solution::valid_subarray_split(vec![2, 6, 3, 4, 3]));
    }
    #[test]
    pub fn test_valid_subarray_split_2() {
        assert_eq!(2, Solution::valid_subarray_split(vec![3, 5]));
    }
    #[test]
    pub fn test_valid_subarray_split_3() {
        assert_eq!(-1, Solution::valid_subarray_split(vec![1, 2, 1]));
    }
}
