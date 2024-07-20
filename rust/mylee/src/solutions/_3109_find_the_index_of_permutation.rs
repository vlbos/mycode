// # [3109. Find the Index of Permutation 🔒](https://leetcode.com/problems/find-the-index-of-permutation)

// ## Description

//

// Given an array perm of length n which is a permutation of [1, 2, ..., n], return the index of perm in the lexicographically sorted array of all of the permutations of [1, 2, ..., n].

// Since the answer may be very large, return it modulo 109 + 7.

//
// Example 1:

//
// Input: perm = [1,2]

// Output: 0

// Explanation:

// There are only two permutations in the following order:

// [1,2], [2,1]
//
// And [1,2] is at index 0.
//

// Example 2:

//
// Input: perm = [3,1,2]

// Output: 4

// Explanation:

// There are only six permutations in the following order:

// [1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]
//
// And [3,1,2] is at index 4.
//

//
// Constraints:

//
// 	1 <= n == perm.length <= 105
// 	perm is a permutation of [1, 2, ..., n].
//

//     int get_permutation_index(vector<int>& perm) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn get_permutation_index(nums: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_get_permutation_index_1() {
        assert_eq!(0, Solution::get_permutation_index(vec![1, 2]));
    }
    #[test]
    pub fn test_get_permutation_index_2() {
        assert_eq!(4, Solution::get_permutation_index(vec![3, 1, 2]));
    }
}
