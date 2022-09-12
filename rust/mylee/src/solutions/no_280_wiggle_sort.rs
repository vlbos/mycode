// # [280. Wiggle Sort](https://leetcode.com/problems/wiggle-sort)

// ## Description

// <p>Given an integer array <code>nums</code>, reorder it such that <code>nums[0] &lt;= nums[1] &gt;= nums[2] &lt;= nums[3]...</code>.</p>

// <p>You may assume the input array always has a valid answer.</p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>

// <pre>
// <strong>Input:</strong> nums = [3,5,2,1,6,4]
// <strong>Output:</strong> [3,5,1,6,2,4]
// <strong>Explanation:</strong> [1,6,2,5,3,4] is also accepted.
// </pre>

// <p><strong>Example 2:</strong></p>

// <pre>
// <strong>Input:</strong> nums = [6,6,5,6,3,8]
// <strong>Output:</strong> [6,6,5,6,3,8]
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>1 &lt;= nums.length &lt;= 5 * 10<sup>4</sup></code></li>
// 	<li><code>0 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
// 	<li>It is guaranteed that there will be an answer for the given input <code>nums</code>.</li>
// </ul>

// <p>&nbsp;</p>
// <p><strong>Follow up:</strong> Could you solve the problem in <code>O(n)</code> time complexity?</p>

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
 nums.sort();
        let mut i = 1;

        while i < nums.len() - 1 {
            nums.swap(i, i + 1);
            i += 2;
        }
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