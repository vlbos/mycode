/*
 * @lc app=leetcode id=1932 lang=rust
 *
 * [1932] Merge BSTs to Create Single BST
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
    pub fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut leaves = HashSet::new();
        let mut candidates = HashMap::new();
        for tree in &trees {
            let root = tree.as_ref().unwrap().borrow();
            if let Some(l) = &root.left {
                leaves.insert(l.borrow().val);
            }
            if let Some(r) = &root.right {
                leaves.insert(r.borrow().val);
            }
            candidates.insert(root.val, tree.clone());
        }
        fn dfs(
            tree: &mut Option<Rc<RefCell<TreeNode>>>,
            prev: &mut i32,
            candidates: &mut HashMap<i32, Option<Rc<RefCell<TreeNode>>>>,
        ) -> bool {
            if tree.is_none() {
                return true;
            }
            let mut node = tree.as_mut().unwrap().borrow_mut();
            if node.left.is_none() && node.right.is_none() && candidates.contains_key(&node.val) {
                node.left = candidates
                    .get(&node.val)
                    .unwrap().as_ref()
                    .unwrap()
                    .borrow()
                    .left
                    .clone();
                node.right = candidates
                    .get(&node.val)
                    .unwrap().as_ref()
                    .unwrap()
                    .borrow()
                    .right
                    .clone();
                candidates.remove(&node.val);
            }
            if !dfs(&mut node.left, prev, candidates) {
                return false;
            }
            if node.val <= *prev {
                return false;
            }
            *prev=node.val;
            dfs(&mut node.right, prev, candidates)
        }
        let mut trees=trees;
        let mut prev=0;
        for  tree in &mut trees{
            if !leaves.contains(&tree.as_mut().unwrap().borrow().val) {
                candidates.remove(&tree.as_mut().unwrap().borrow().val);
                return if dfs(tree, &mut prev, &mut candidates) && candidates.is_empty() {
                    tree.clone()
                } else {
                    None
                };
            }
        }
        None
    }
}
// @lc code=end
