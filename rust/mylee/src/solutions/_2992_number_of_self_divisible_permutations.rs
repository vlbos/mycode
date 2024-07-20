// # [2992. Number of Self-Divisible Permutations](https://leetcode.com/problems/number-of-self-divisible-permutations)

// ## Description

// Given an integer n, return the number of permutations of the 1-indexed array nums = [1, 2, ..., n],
// such that it 's self-divisible.

// A 1-indexed array a of length n is self-divisible if for every 1  <= i  <= n,
// gcd(a[i], i) == 1.

// A permutation of an array is a rearrangement of the elements of that array,
// for example here are all of the permutations of the array [1, 2, 3]:

// 	[1, 2, 3]
// 	[1, 3, 2]
// 	[2, 1, 3]
// 	[2, 3, 1]
// 	[3, 1, 2]
// 	[3, 2, 1]

//
// Example 1:

// Input: n = 1
// Output: 1
// Explanation: The array [1] has only 1 permutation which is self-divisible.

// Example 2:

// Input: n = 2
// Output: 1
// Explanation: The array [1,2] has 2 permutations and only one of them is self-divisible:
// nums = [1,2]: This is not self-divisible since gcd(nums[2], 2) != 1.
// nums = [2,1]: This is self-divisible since gcd(nums[1], 1) == 1 and gcd(nums[2], 2) == 1.

// Example 3:

// Input: n = 3
// Output: 3
// Explanation: The array [1,2,3] has 3 self-divisble permutations: [1,3,2], [3,1,2], [2,3,1].
// It can be shown that the other 3 permutations are not self-divisible. Hence the answer is 3.

//
// Constraints:

// 	1  <= n  <= 12

//     int self_divisible_permutation_count(int n) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn self_divisible_permutation_count(n: i32) -> i32 {
        let mut memo = vec![-1; 1 << (n + 1)];
        fn dfs(mask: i32, n: i32, memo: &mut Vec<i32>) -> i32 {
            if memo[mask as usize] != -1 {
                return memo[mask as usize];
            }
            let i = mask.count_ones() as i32 + 1;
            if i > n {
                return 1;
            }
            memo[mask as usize] = 0;
            for j in 1..=n {
                if mask >> j & 1 == 0 && (i % j == 0 || j % i == 0) {
                    println!("{i},{j},{mask}");
                    memo[mask as usize] += dfs(mask | (1 << j), n, memo);
                }
            }
            memo[mask as usize]
        }
        dfs(0, n, &mut memo)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_self_divisible_permutation_count_1() {
        assert_eq!(1, Solution::self_divisible_permutation_count(1));
    }
    #[test]
    pub fn test_self_divisible_permutation_count_2() {
        assert_eq!(1, Solution::self_divisible_permutation_count(2));
    }
    #[test]
    pub fn test_self_divisible_permutation_count_3() {
        assert_eq!(3, Solution::self_divisible_permutation_count(3));
    }
}
