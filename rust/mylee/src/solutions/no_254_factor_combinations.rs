// # [254. Factor Combinations](https://leetcode.com/problems/factor-combinations)

// ## Description

// <p>Numbers can be regarded as the product of their factors.</p>

// <ul>
// 	<li>For example, <code>8 = 2 x 2 x 2 = 2 x 4</code>.</li>
// </ul>

// <p>Given an integer <code>n</code>, return <em>all possible combinations of its factors</em>. You may return the answer in <strong>any order</strong>.</p>

// <p><strong>Note</strong> that the factors should be in the range <code>[2, n - 1]</code>.</p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>

// <pre>
// <strong>Input:</strong> n = 1
// <strong>Output:</strong> []
// </pre>

// <p><strong>Example 2:</strong></p>

// <pre>
// <strong>Input:</strong> n = 12
// <strong>Output:</strong> [[2,6],[3,4],[2,2,3]]
// </pre>

// <p><strong>Example 3:</strong></p>

// <pre>
// <strong>Input:</strong> n = 37
// <strong>Output:</strong> []
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>1 &lt;= n &lt;= 10<sup>7</sup></code></li>
// </ul>

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