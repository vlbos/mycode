// # [3109. Find the Index of Permutation 🔒](https://leetcode.com/problems/find-the-index-of-permutation)

// ## Description

//

// Given an array perm of length n which is a permutation of [1, 2, ..., n],
// return the index of perm in the lexicographically sorted array of all of the permutations of [1, 2, ..., n].

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
    pub fn get_permutation_index(perm: Vec<i32>) -> i32 {
        let n = perm.len();
        let update = |mut x: i32, delta: i32, tree: &mut Vec<i32>| {
            while x <= n as i32 {
                tree[x as usize] += delta;
                x += x & -x;
            }
        };
        let query = |mut x: i32, tree: &[i32]| {
            let mut s = 0;
            while x > 0 {
                s += tree[x as usize];
                x -= x & -x;
            }
            s
        };
        let mut ans = 0;
        let mut tree = vec![0; n + 1];
        let mut f = vec![1; n];
        for i in 1..n {
            f[i] = ((f[i - 1] as i64 * i as i64) % 1_000_000_007) as i32;
        }
        for (i, x) in perm.into_iter().enumerate() {
            let cnt = x - 1 - query(x, &tree);
            ans += cnt as i64 * f[n - i - 1] as i64 % 1_000_000_007;
            update(x, 1, &mut tree);
        }
        (ans % 1_000_000_007) as _
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
