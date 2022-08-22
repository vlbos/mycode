// 1708\. Largest Subarray Length K
// ================================

// An array `A` is larger than some array `B` if for the first index `i` where `A[i] != B[i]`, `A[i] > B[i]`.

// For example, consider `0`\-indexing:

// *   `[1,3,2,4] > [1,2,2,4]`, since at index `1`, `3 > 2`.
// *   `[1,4,4,4] < [2,1,1,1]`, since at index `0`, `1 < 2`.

// A subarray is a contiguous subsequence of the array.

// Given an integer array `nums` of **distinct** integers, return the **largest** subarray of `nums` of length `k`.

// **Example 1:**

// **Input:** nums = \[1,4,5,2,3\], k = 3
// **Output:** \[5,2,3\]
// **Explanation:** The subarrays of size 3 are: \[1,4,5\], \[4,5,2\], and \[5,2,3\].
// Of these, \[5,2,3\] is the largest.

// **Example 2:**

// **Input:** nums = \[1,4,5,2,3\], k = 4
// **Output:** \[4,5,2,3\]
// **Explanation:** The subarrays of size 4 are: \[1,4,5,2\], and \[4,5,2,3\].
// Of these, \[4,5,2,3\] is the largest.

// **Example 3:**

// **Input:** nums = \[1,4,5,2,3\], k = 1
// **Output:** \[5\]

// **Constraints:**

// *   `1 <= k <= nums.length <= 105`
// *   `1 <= nums[i] <= 109`
// *   All the integers of `nums` are **unique**.

// **Follow up:** What if the integers in `nums` are not distinct?

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

//  int[] largestSubarray(int[] nums, int k)

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn check_equivalence(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        use std::collections::HashMap;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, v: i32, freq: &mut HashMap<i32, i32>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            if (node.val as u8 as char).is_ascii_alphabetic() {
                *freq.entry(node.val).or_insert(0) += v;
            } else {
                dfs(&node.left, v, freq);
                dfs(&node.right, v, freq);
            }
        }
        let mut freq = HashMap::new();
        dfs(&root1, 1, &mut freq);
        dfs(&root2, -1, &mut freq);
        if freq.values().any(|v| *v > 0) {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::tree;
    use super::super::util::tree::to_tree;
    fn to_exp_tree(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        to_tree(
            s.split(',')
                .map(|x| {
                    if x == "null" {
                        None
                    } else {
                        Some(x.as_bytes()[0] as i32)
                    }
                })
                .collect::<Vec<Option<i32>>>(),
        )
    }
    #[test]
    pub fn test_check_equivalence_1() {
        assert!(Solution::check_equivalence(
            to_exp_tree("x"),
            to_exp_tree("x")
        ));
    }
    #[test]
    pub fn test_check_equivalence_2() {
        assert!(Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,c,a")
        ));
    }
    #[test]
    pub fn test_check_equivalence_3() {
        assert!(!Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,d,a")
        ));
    }
}
