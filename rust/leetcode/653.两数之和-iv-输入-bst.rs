/*
 * @lc app=leetcode.cn id=653 lang=rust
 *
 * [653] 两数之和 IV - 输入 BST
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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, mut s: &mut Vec<i32>, k: i32) -> bool {
            if let Some(_n) = root {
                let n = _n.borrow();
                if inorder(&n.left, &mut s, k) || s.contains(&(k - n.val)) {
                    return true;
                }
                if !s.contains(&n.val) {
                    s.push(n.val);
                }
                return inorder(&n.right, &mut s, k);
            }

            false
        }
        let mut s = Vec::<i32>::new();
        inorder(&root, &mut s, k)
    }
}
// @lc code=end
