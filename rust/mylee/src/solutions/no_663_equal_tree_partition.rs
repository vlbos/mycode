// 663\. Equal Tree Partition
// ==========================

// Given a binary tree with `n` nodes, your task is to check if it's possible to partition the tree to two trees
// which have the equal sum of values after removing **exactly** one edge on the original tree.

// **Example 1:**

// **Input:**
//     5
//    / \\
//   10 10
//     /  \\
//    2   3

// **Output:** True
// **Explanation:**
//     5
//    /
//   10

// Sum: 15

//    10
//   /  \\
//  2    3

// Sum: 15

// **Example 2:**

// **Input:**
//     1
//    / \\
//   2  10
//     /  \\
//    2   20

// **Output:** False
// **Explanation:** You can't split the tree into two trees with equal sum after removing exactly one edge on the tree.

// **Note:**

// 1.  The range of tree node value is in the range of \[-100000, 100000\].
// 2.  1 <= n <= 10000

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook)
use crate::solutions::util::tree::TreeNode;

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub  struct TreeNode {
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

impl Solution {
    pub fn check_equal_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // let root_sum = Solution::sum_tree_rec(root.clone());
        // if root_sum.is_none() || root_sum.unwrap() % 2 == 1 {
        //     return false;
        // }
        // Solution::check_equal_tree_rec(root, &root_sum).1
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, parent_sum: i32, ans: &mut bool) -> i32 {
            if root.is_none() {
                return 0;
            }
            let node = root.as_ref().unwrap().borrow();
            let sum = parent_sum + node.val;
            let mut sums = Vec::new();
            for child in [&node.left, &node.right] {
                let child_sum = dfs(child, sum, ans);
                if *ans {
                    return 0;
                }
                sums.push(child_sum);
            }

            if (sums[0] - sums[1]).abs() == sum {
                *ans = true;
                return 0;
            }
            node.val + sums.into_iter().sum::<i32>()
        }
        let mut ans = false;
        dfs(&root, 0, &mut ans);
        ans
    }

    // fn sum_tree_rec(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    //     match root {
    //         None => None,
    //         Some(node_ref) => {
    //             let node = node_ref.borrow();
    //             let left = Solution::sum_tree_rec(node.left.clone());
    //             let right = Solution::sum_tree_rec(node.right.clone());
    //             if left.is_none() && right.is_none() {
    //                 Some(node.val)
    //             } else {
    //                 Some(left.unwrap_or(0) + node.val + right.unwrap_or(0))
    //             }
    //         }
    //     }
    // }

    // fn check_equal_tree_rec(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     root_sum: &Option<i32>,
    // ) -> (Option<i32>, bool) {
    //     match root {
    //         None => (None, false),
    //         Some(node_ref) => {
    //             let node = node_ref.borrow();
    //             let (left, left_success) =
    //                 Solution::check_equal_tree_rec(node.left.clone(), &root_sum);
    //             let (right, right_success) =
    //                 Solution::check_equal_tree_rec(node.right.clone(), &root_sum);
    //             if left_success || right_success {
    //                 (None, true)
    //             } else if left.is_none() && right.is_none() {
    //                 (Some(node.val), false)
    //             } else {
    //                 let clip_left = left == Some(root_sum.unwrap_or(0) - left.unwrap_or(0));
    //                 let clip_right = Some(root_sum.unwrap_or(0) - right.unwrap_or(0)) == right;
    //                 let sum = Some(left.unwrap_or(0) + node.val + right.unwrap_or(0));
    //                 if clip_left || clip_right {
    //                     (sum, true)
    //                 } else {
    //                     (sum, false)
    //                 }
    //             }
    //         }
    //     }
    // }
}
// @lc code=end

#[allow(dead_code)]
pub  struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
    fn test_check_equal_tree_1() {
        assert_eq!(
            Solution::check_equal_tree(tree![5, 10, 10, null, null, 2, 3]),
            true
        );
    }

    #[test]
    fn test_check_equal_tree_2() {
        let tree = tree![1, 2, 10, null, null, 2, 20];
        assert_eq!(Solution::check_equal_tree(tree), false);
    }

    #[test]
    fn test_check_equal_tree_3() {
        let tree = tree![1, null, 2, 2];
        assert_eq!(Solution::check_equal_tree(tree), false);
    }

    #[test]
    fn test_check_equal_tree_4() {
        let tree = tree![2, 1, 3, 0, 2, null, null, null, null, 2, null, 1, null, null, 1];
        assert_eq!(Solution::check_equal_tree(tree), true);
    }
}
