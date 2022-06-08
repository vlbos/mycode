/*
 * @lc app=leetcode id=1373 lang=rust
 *
 * [1373] Maximum Sum BST in Binary Tree
 */

// @lc code=start
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
impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
         fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> Vec<i32> {
            if let Some(pnode) = root {
                let node = pnode.borrow();
                let l = dfs(&node.left, ans);
                let r = dfs(&node.right, ans);
                if l[1] < node.val && r[0] > node.val {
                    let sum = node.val + l[2] + r[2];
                    *ans = (*ans).max(sum);
                    return vec![l[0].min(node.val),node.val.max(r[1]),sum];
                }
                return vec![i32::MIN, i32::MAX, 0];
            }
            vec![i32::MAX, i32::MIN, 0]
        }
        let mut ans = 0;
        dfs(&root, &mut ans);
        ans
    }
}
// @lc code=end
