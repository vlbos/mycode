/*
 * @lc app=leetcode id=863 lang=rust
 *
 * [863] All Nodes Distance K in Binary Tree
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        fn find_parent(
            root: &Option<Rc<RefCell<TreeNode>>>,
            parents: &mut HashMap<i32, Option<Rc<RefCell<TreeNode>>>>,
        ) {
            if let Some(n) = root {
                let node = n.borrow();
                if let Some(l) = &node.left {
                    parents.insert(l.borrow().val, root.clone());
                    find_parent(&node.left, parents);
                }
                if let Some(r) = &node.right {
                    parents.insert(r.borrow().val, root.clone());
                    find_parent(&node.right, parents);
                }
            }
        }
        fn find_ans(
            root: &Option<Rc<RefCell<TreeNode>>>,
            from: &Option<Rc<RefCell<TreeNode>>>,
            parents: &HashMap<i32, Option<Rc<RefCell<TreeNode>>>>,
            ans: &mut Vec<i32>,
            depth: i32,
            k: i32,
        ) {
            if let Some(n) = root {
                let node = n.borrow();

                if depth == k {
                    ans.push(node.val);
                    return;
                }
                if &node.left != from {
                    find_ans(&node.left, root, parents, ans, depth + 1, k);
                }
                if &node.right != from {
                    find_ans(&node.right, root, parents, ans, depth + 1, k);
                }
                if let Some(f) = parents.get(&node.val) {
                    if f != from {
                        find_ans(&f, root, parents, ans, depth + 1, k);
                    }
                }
            }
        }
        let mut ans = Vec::new();
        let mut parents = HashMap::new();
        find_parent(&root, &mut parents);
        let from: Option<Rc<RefCell<TreeNode>>> = None;
        find_ans(&target, &from, &parents, &mut ans, 0, k);
        ans
    }
}
// @lc code=end
