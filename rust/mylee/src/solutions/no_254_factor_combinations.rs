// # [254. Factor Combinations](https://leetcode.com/problems/factor-combinations)

// ## Description

// Numbers can be regarded as the product of their factors.

//
// 	For example, 8 = 2 x 2 x 2 = 2 x 4.
//

// Given an integer n, return all possible combinations of its factors. You may return the answer in any order.

// Note that the factors should be in the range [2, n - 1].

// Example 1:

//
// Input: n = 1
// Output: []
//

// Example 2:

//
// Input: n = 12
// Output: [[2,6],[3,4],[2,2,3]]
//

// Example 3:

//
// Input: n = 37
// Output: []
//

// Constraints:

//
// 	1 <= n <= 107
//

impl Solution {
    pub fn get_factors(n: i32) -> Vec<Vec<i32>> {
        fn dfs(n: i32, start: i32) -> Vec<Vec<i32>> {
            let mut ans = vec![];
            let mut i = start;
            while i * i <= n {
                if n % i == 0 {
                    ans.push(vec![i, n / i]);
                    for sub in dfs(n / i, i).iter() {
                        ans.push([vec![i], sub.clone()].concat());
                    }
                }
                i += 1;
            }
            ans
        }
        dfs(n, 2)
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::tree;
    #[test]
    pub fn test_read() {
        // assert_eq!(
        //     Solution::read(tree![1, 2, 3, 4, 5]),
        //     tree![4, 5, 2, null, null, 3, 1]
        // );
    }
}
