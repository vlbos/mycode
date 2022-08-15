// 1214\. Two Sum BSTs
// ===================

// Given two binary search trees, return `True` if and only if there is a node in the first tree and a node in the second tree whose values sum up to a given integer `target`.

// **Example 1:**

// **![](https://assets.leetcode.com/uploads/2019/05/31/1368_1_a2.png)![](https://assets.leetcode.com/uploads/2019/05/31/1368_1_b.png)**

// **Input:** root1 = \[2,1,4\], root2 = \[1,0,3\], target = 5
// **Output:** true
// **Explanation:** 2 and 3 sum up to 5.

// **Example 2:**

// **![](https://assets.leetcode.com/uploads/2019/05/31/1368_2_a.png)![](https://assets.leetcode.com/uploads/2019/05/31/1368_2_b.png)**

// **Input:** root1 = \[0,-10,10\], root2 = \[5,1,7,0,2\], target = 18
// **Output:** false

// **Constraints:**

// *   Each tree has at most `5000` nodes.
// *   `-10^9 <= target, node.val <= 10^9`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

// Definition for a binary tree node
use super::util::tree::TreeNode;
#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn two_sum_bs_ts(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> bool {
        if root1.is_none() || root2.is_none() {
            return false;
        }
        let (node1, node2) = (
            root1.as_ref().unwrap().borrow(),
            root2.as_ref().unwrap().borrow(),
        );
        if node1.val + node2.val == target {
            return true;
        }
        if node1.val + node2.val > target {
            Self::two_sum_bs_ts(node1.left.clone(), root2.clone(), target)
                || Self::two_sum_bs_ts(root1.clone(), node2.left.clone(), target)
        } else {
            Self::two_sum_bs_ts(node1.right.clone(), root2.clone(), target)
                || Self::two_sum_bs_ts(root1.clone(), node2.right.clone(), target)
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    #[test]
    pub fn test_two_sum_bs_ts_1() {
        assert!(Solution::two_sum_bs_ts(tree![2, 1, 4], tree![1, 0, 3], 5));
    }
    #[test]
    pub fn test_two_sum_bs_ts_2() {
        assert!(!Solution::two_sum_bs_ts(
            tree![0, -10, 10],
            tree![5, 1, 7, 0, 2],
            18
        ));
    }
}
