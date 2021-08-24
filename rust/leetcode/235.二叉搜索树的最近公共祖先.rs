/*
 * @lc app=leetcode.cn id=235 lang=rust
 *
 * [235] 二叉搜索树的最近公共祖先
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
use std::mem;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // let mut root = root.unwrap();
        // let mut p = p.unwrap().borrow().val;
        // let mut q = q.unwrap().borrow().val;

        // if p > q {
        //     mem::swap(&mut p, &mut q);
        // }

        // loop {
        //     root = {
        //         let root_ref = root.borrow();

        //         if q < root_ref.val {
        //             root_ref.left.clone().unwrap()
        //         } else if p > root_ref.val {
        //             root_ref.right.clone().unwrap()
        //         } else {
        //             break;
        //         }
        //     }
        // }

        // Some(root)
            fn inner_lowest_common_ancestor(root: &Rc<RefCell<TreeNode>>, p: &TreeNode, q: &TreeNode) -> Rc<RefCell<TreeNode>> {
                let root_ref = root.borrow();

                if q.val < root_ref.val {
                    inner_lowest_common_ancestor(root_ref.left.as_ref().unwrap(), p, q)
                } else if p.val > root_ref.val {
                    inner_lowest_common_ancestor(root_ref.right.as_ref().unwrap(), p, q)
                } else {
                    Rc::clone(root)
                }
            }

        let root = root.unwrap();
        let p = p.unwrap();
        let q = q.unwrap();
        let p_ref = p.borrow();
        let q_ref = q.borrow();

        Some(if p_ref.val < q_ref.val {
            inner_lowest_common_ancestor(&root, &p_ref, &q_ref)
        } else {
            inner_lowest_common_ancestor(&root, &q_ref, &p_ref)
        })

    }
}
// @lc code=end

