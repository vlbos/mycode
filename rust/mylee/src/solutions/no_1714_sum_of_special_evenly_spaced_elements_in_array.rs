// 1714\. Sum Of Special Evenly-Spaced Elements In Array
// =====================================================

// You are given a **0-indexed** integer array `nums` consisting of `n` non-negative integers.

// You are also given an array `queries`, where `queries[i] = [xi, yi]`.
// The answer to the `ith` query is the sum of all `nums[j]` where `xi <= j < n` and `(j - xi)` is divisible by `yi`.

// Return _an array_ `answer` _where_ `answer.length == queries.length` _and_ `answer[i]` _is the answer to the_ `ith` _query **modulo**_ `109 + 7`.

// **Example 1:**

// **Input:** nums = \[0,1,2,3,4,5,6,7\], queries = \[\[0,3\],\[5,1\],\[4,2\]\]
// **Output:** \[9,18,10\]
// **Explanation:** The answers of the queries are as follows:
// 1) The j indices that satisfy this query are 0, 3, and 6. nums\[0\] + nums\[3\] + nums\[6\] = 9
// 2) The j indices that satisfy this query are 5, 6, and 7. nums\[5\] + nums\[6\] + nums\[7\] = 18
// 3) The j indices that satisfy this query are 4 and 6. nums\[4\] + nums\[6\] = 10

// **Example 2:**

// **Input:** nums = \[100,200,101,201,102,202,103,203\], queries = \[\[0,7\]\]
// **Output:** \[303\]

// **Constraints:**

// *   `n == nums.length`
// *   `1 <= n <= 5 * 104`
// *   `0 <= nums[i] <= 109`
// *   `1 <= queries.length <= 1.5 * 105`
// *   `0 <= xi < n`
// *   `1 <= yi <= 5 * 104`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [MakeMyTrip](https://leetcode.ca/tags/#MakeMyTrip) [Sprinklr](https://leetcode.ca/tags/#Sprinklr)

// int[] solve(int[] nums, int[][] queries)

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn solve(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix = std::collections::HashMap::new();
        let mut ans = Vec::new();
        let n = nums.len();
        let m = 1_000_000_007;
        for q in &queries {
            let (x, y) = (q[0], q[1]);
            if y * y > n as i32 {
                let mut total = 0;
                for i in (x as usize..n).step_by(y as usize) {
                    total += nums[i];
                    total %= m;
                }
                ans.push(total);
            } else {
                let begin = x % y;
                if !prefix.contains_key(&(begin, y)) {
                    prefix.insert((begin, y), vec![0]);
                    for i in (begin as usize..n).step_by(y as usize) {
                        if let Some(v) = prefix.get_mut(&(begin, y)) {
                            let t = (*v.last().unwrap() + nums[i]) % m;
                            v.push(t);
                        }
                    }
                }
                let v = prefix.get(&(begin, y)).unwrap();
                ans.push((*v.last().unwrap() - v[(x / y) as usize]) % m);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_solve_1() {
        assert_eq!(
            vec![9, 18, 10],
            Solution::solve(
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                vec![vec![0, 3], vec![5, 1], vec![4, 2]]
            )
        );
    }
    #[test]
    pub fn test_solve_2() {
        assert_eq!(
            vec![303],
            Solution::solve(
                vec![100, 200, 101, 201, 102, 202, 103, 203],
                vec![vec![0, 7]]
            )
        );
    }
}
