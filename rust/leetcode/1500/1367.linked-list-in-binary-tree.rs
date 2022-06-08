/*
 * @lc app=leetcode id=1367 lang=rust
 *
 * [1367] Linked List in Binary Tree
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if head.is_none() {
                return true;
            }
            if root.is_none() {
                return false;
            }
            if head.as_ref().unwrap().val != root.as_ref().unwrap().borrow().val {
                return false;
            }
            dfs(
                &head.as_ref().unwrap().next,
                &root.as_ref().unwrap().borrow().left,
            ) || dfs(
                &head.as_ref().unwrap().next,
                &root.as_ref().unwrap().borrow().right,
            )
        }
        if root.is_none() {
            return false;
        }
        dfs(&head, &root)
            || Self::is_sub_path(head.clone(), root.as_ref().unwrap().borrow().left.clone())
            || Self::is_sub_path(head.clone(), root.as_ref().unwrap().borrow().right.clone())
    }
}
// @lc code=end
