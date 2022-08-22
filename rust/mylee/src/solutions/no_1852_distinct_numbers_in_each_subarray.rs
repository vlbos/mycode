// 1852\. Distinct Numbers in Each Subarray
// ========================================

// Given an integer array `nums` and an integer `k`, you are asked to construct the array `ans` of size `n-k+1` where `ans[i]` is the number of **distinct** numbers in the subarray `nums[i:i+k-1] = [nums[i], nums[i+1], ..., nums[i+k-1]]`.

// Return _the array_ `ans`.

// **Example 1:**

// **Input:** nums = \[1,2,3,2,2,1,3\], k = 3
// **Output:** \[3,2,2,2,3\]
// **Explanation:** The number of distinct elements in each subarray goes as follows:
// - nums\[0:2\] = \[1,2,3\] so ans\[0\] = 3
// - nums\[1:3\] = \[2,3,2\] so ans\[1\] = 2
// - nums\[2:4\] = \[3,2,2\] so ans\[2\] = 2
// - nums\[3:5\] = \[2,2,1\] so ans\[3\] = 2
// - nums\[4:6\] = \[2,1,3\] so ans\[4\] = 3

// **Example 2:**

// **Input:** nums = \[1,1,1,1,2,3,4\], k = 4
// **Output:** \[1,2,3,4\]
// **Explanation:** The number of distinct elements in each subarray goes as follows:
// - nums\[0:3\] = \[1,1,1,1\] so ans\[0\] = 1
// - nums\[1:4\] = \[1,1,1,2\] so ans\[1\] = 2
// - nums\[2:5\] = \[1,1,2,3\] so ans\[2\] = 3
// - nums\[3:6\] = \[1,2,3,4\] so ans\[3\] = 4

// **Constraints:**

// *   `1 <= k <= nums.length <= 105`
// *   `1 <= nums[i] <= 105`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

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
