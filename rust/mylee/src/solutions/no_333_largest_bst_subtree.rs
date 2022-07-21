// 333\. Largest BST Subtree
// =========================

// Given a binary tree, find the largest subtree which is a Binary Search Tree (BST),
// where largest means subtree with largest number of nodes in it.

// **Note:**
// A subtree must include all of its descendants.

// **Example:**

// **Input:** \[10,5,15,1,8,null,7\]

//    10
//    / \\
//  5  15
//  / \\   \\
// 1   8   7

// **Output:** 3
// **Explanation:** The Largest BST Subtree in this case is the highlighted one.
//              The return value is the subtree's size, which is 3.

// **Follow up:**
// Can you figure out ways to solve it with O(n) time complexity?

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Google](https://leetcode.ca/tags/#Google) [Lyft](https://leetcode.ca/tags/#Lyft) [Microsoft](https://leetcode.ca/tags/#Microsoft)
// @lc code=start
use crate::solutions::util::tree::TreeNode;
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

// #[derive(Debug)]
// struct LargetBSTSolution {
//     max_bst: usize,
//     is_bst: bool,
//     min: i32,
//     max: i32,
// }

impl Solution {
    pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Solution::largest_bst_subtree_recursive(root).max_bst as i32
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, bool, i32, i32) {
            if let Some(node) = root {
                let node = node.borrow();
                let l = dfs(&node.left);
                let r = dfs(&node.right);
                if l.1 && r.1  && node.val > l.3 && node.val < r.2 {
                        (l.0 + r.0 + 1, true, l.2.min(node.val), r.3.max(node.val))
                } else {
                        (l.0.max(r.0), false,  l.2.min(node.val), r.3.max(node.val))
                }
            } else {
                (0, true, i32::MAX, i32::MIN)
            }
        }
        dfs(&root).0
    }

    // fn largest_bst_subtree_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> LargetBSTSolution {
    //     match root {
    //         None => LargetBSTSolution {
    //             max_bst: 0,
    //             is_bst: true,
    //             min: i32::max_value(),
    //             max: i32::min_value(),
    //         },
    //         Some(node_ref) => {
    //             let mut node = node_ref.borrow_mut();
    //             let value = node.val;
    //             let left_solution = Solution::largest_bst_subtree_recursive(node.left.take());
    //             let right_solution = Solution::largest_bst_subtree_recursive(node.right.take());
    //             if left_solution.is_bst
    //                 && right_solution.is_bst
    //                 && left_solution.max < value
    //                 && right_solution.min > value
    //             {
    //                 LargetBSTSolution {
    //                     max_bst: left_solution.max_bst + right_solution.max_bst + 1,
    //                     is_bst: true,
    //                     min: i32::min(left_solution.min, value),
    //                     max: i32::max(right_solution.max, value),
    //                 }
    //             } else {
    //                 LargetBSTSolution {
    //                     max_bst: usize::max(left_solution.max_bst, right_solution.max_bst),
    //                     is_bst: false,
    //                     min: i32::min(left_solution.min, value),
    //                     max: i32::max(right_solution.max, value),
    //                 }
    //             }
    //         }
    //     }
    // }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
    fn test_largest_bst_subtree_1() {
        let tree = tree![10,5,15,1,8,null,7];
        assert_eq!(Solution::largest_bst_subtree(tree), 3);
    }

    #[test]
    fn test_largest_bst_subtree_2() {
        let tree = tree![10,5,15,null,8,null,7];
        assert_eq!(Solution::largest_bst_subtree(tree), 2);
    }
}
