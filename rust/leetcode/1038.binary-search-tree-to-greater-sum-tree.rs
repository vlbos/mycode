/*
 * @lc app=leetcode id=1038 lang=rust
 *
 * [1038] Binary Search Tree to Greater Sum Tree
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,sum:&mut i32)->Option<Rc<RefCell<TreeNode>>> {
            if let Some(node)=root{
                    let mut n = node.borrow_mut();
                    n.right=dfs(&n.right,sum);
                    n.val+=*sum;
                    *sum=n.val;
                    n.left=dfs(&n.left,sum);
                return Some(node.clone());
            }
            None
        }
        dfs(&root,&mut 0)
    }
}
// @lc code=end

