// 1746\. Maximum Subarray Sum After One Operation
// ===============================================

// You are given an integer array `nums`. You must perform **exactly one** operation where you can **replace** one element `nums[i]` with `nums[i] * nums[i]`.

// Return _the **maximum** possible subarray sum after **exactly one** operation_. The subarray must be non-empty.

// **Example 1:**

// **Input:** nums = \[2,-1,-4,-3\]
// **Output:** 17
// **Explanation:** You can perform the operation on index 2 (0-indexed) to make nums = \[2,-1,**16**,-3\]. Now, the maximum subarray sum is 2 + -1 + 16 = 17.

// **Example 2:**

// **Input:** nums = \[1,-1,1,1,-1,-1,1\]
// **Output:** 4
// **Explanation:** You can perform the operation on index 1 (0-indexed) to make nums = \[1,**1**,1,1,-1,-1,1\]. Now, the maximum subarray sum is 1 + 1 + 1 + 1 = 4.

// **Constraints:**

// *   `1 <= nums.length <= 105`
// *   `-104 <= nums[i] <= 104`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Sprinklr](https://leetcode.ca/tags/#Sprinklr)

// int maxSumAfterOperation(int[] nums)

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
