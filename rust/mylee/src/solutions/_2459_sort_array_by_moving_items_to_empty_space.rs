// # [2459. Sort Array by Moving Items to Empty Space](https://leetcode.com/problems/sort-array-by-moving-items-to-empty-space)
// ## Description

//  You are given an integer array  nums  of size  n  containing  each  element from  0  to  n - 1  ( inclusive ).
//  Each of the elements from  1  to  n - 1  represents an item, and the element  0  represents an empty space.

//  In one operation, you can move  any  item to the empty space.
// nums  is considered to be sorted if the numbers of all the items are in  ascending  order
// and the empty space is either at the beginning or at the end of the array.

//  For example, if  n = 4 ,  nums  is sorted if:

// 	  nums = [0,1,2,3]  or
// 	  nums = [1,2,3,0]

//  ...and considered to be unsorted otherwise.

//  Return  the  minimum  number of operations needed to sort   nums .

//  Example 1:

//  Input:  nums = [4,2,0,3,1]
//  Output:  3
//  Explanation:
// - Move item 2 to the empty space. Now, nums = [4,0,2,3,1].
// - Move item 1 to the empty space. Now, nums = [4,1,2,3,0].
// - Move item 4 to the empty space. Now, nums = [0,1,2,3,4].
// It can be proven that 3 is the minimum number of operations needed.

//  Example 2:

//  Input:  nums = [1,2,3,4,0]
//  Output:  0
//  Explanation:  nums is already sorted so return 0.

//  Example 3:

//  Input:  nums = [1,0,2,4,3]
//  Output:  2
//  Explanation:
// - Move item 2 to the empty space. Now, nums = [1,2,0,4,3].
// - Move item 3 to the empty space. Now, nums = [1,2,3,4,0].
// It can be proven that 2 is the minimum number of operations needed.

//   Constraints:

// 	  n == nums.length
// 	  2 <= n <= 10^5
// 	  0 <= nums[i] < n
// 	 All the values of  nums  are  unique .
//  int sort_array(vector<int>& nums) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let f = |nums: &Vec<i32>, k: i32| {
            let mut vis = vec![false; n];
            let mut cnt = 0;
            for (i, &v) in nums.iter().enumerate() {
                if v == i as i32 || vis[i] {
                    continue;
                }
                cnt += 1;
                let mut j = i;
                while !vis[j] {
                    vis[j] = true;
                    cnt += 1;
                    j = nums[j] as usize;
                }
            }
            cnt - if nums[k as usize] == k { 0 } else { 2 }
        };
        let nn = n as i32;
        f(&nums, 0).min(f(
            &nums
                .iter()
                .map(|&v| (v - 1 + nn) % nn)
                .collect::<Vec<i32>>(),
            nn - 1,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_sort_array_1() {
        assert_eq!(3, Solution::sort_array(vec![4, 2, 0, 3, 1]));
    }
    #[test]
    pub fn test_sort_array_2() {
        assert_eq!(0, Solution::sort_array(vec![1, 2, 3, 4, 0]));
    }
    #[test]
    pub fn test_sort_array_3() {
        assert_eq!(2, Solution::sort_array(vec![1, 0, 2, 4, 3]));
    }
}
