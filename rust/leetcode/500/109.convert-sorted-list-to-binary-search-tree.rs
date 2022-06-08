/*
 * @lc app=leetcode id=109 lang=rust
 *
 * [109] Convert Sorted List to Binary Search Tree
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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = &head {
            if n.next.is_none() {
                return Some(Rc::new(RefCell::new(TreeNode::new(n.val))));
            }
        } else {
            return None;
        }
        let mut p = &head;
        let mut v = Vec::new();
        while let Some(n) = &p {
            v.push(n.val);
            p = &n.next;
        }
        let mut root = None;

        fn build_bst(v: &[i32], mut root: &mut Option<Rc<RefCell<TreeNode>>>) {
            if v.is_empty() {
                *root = None;
                return;
            }
            let mut n = TreeNode::new(v[v.len() / 2]);
            build_bst(&v[..v.len() / 2], &mut n.left);
            build_bst(&v[v.len() / 2 + 1..], &mut n.right);
            *root = Some(Rc::new(RefCell::new(n)));
        }
        build_bst(&v, &mut root);
        root
    }
}
// @lc code=end
