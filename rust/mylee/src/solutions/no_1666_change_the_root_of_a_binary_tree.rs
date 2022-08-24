// 1666\. Change the Root of a Binary Tree
// =======================================

// Given the `root` of a binary tree and a `leaf` node, reroot the tree so that the `leaf` is the new root.

// You can reroot the tree with the following steps for each node `cur` on the path **starting from the** `leaf` up to the `root`​​​ **excluding the root**:

// 1.  If `cur` has a left child, then that child becomes `cur`'s right child. Note that it is guaranteed that `cur` will have at most one child.
// 2.  `cur`'s original parent becomes `cur`'s left child.

// Return _the new root_ _of the rerooted tree._

// **Note:** Ensure that your solution sets the `Node.parent` pointers correctly after rerooting or you will receive "Wrong Answer".

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2020/11/24/flitree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], leaf = 7
// **Output:** \[7,2,null,5,4,3,6,null,null,null,1,null,null,0,8\]

// **Example 2:**

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], leaf = 0
// **Output:** \[0,1,null,3,8,5,null,null,null,6,2,null,null,7,4\]

// **Constraints:**

// *   The number of nodes in the tree is in the range `[2, 100]`.
// *   `-109 <= Node.val <= 109`
// *   All `Node.val` are **unique**.
// *   `leaf` exist in the tree.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

//  Node flip_binary_tree(Node root, Node leaf)

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flip_binary_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        leaf: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // let mut curr = leaf.clone();
        // let mut new_parent: Option<Rc<RefCell<TreeNode>>> = None;
        // let mut i = 0;
        // while curr!=root {
        //     if i>3{
        //     break;
        //     }
        //     if curr.as_ref().unwrap().borrow().left.is_some(){
        //          curr.as_ref().unwrap().borrow_mut().right =curr.as_ref().unwrap().borrow_mut().left.take();
        //     }
        //      if curr.as_ref().unwrap().borrow().parent.is_some(){
        //         let parent=curr.as_ref().unwrap().borrow_mut().parent.take();
        //         if parent.as_ref().unwrap().borrow().left==curr{
        //             parent.as_ref().unwrap().borrow_mut().left=None;
        //         }else{
        //             parent.as_ref().unwrap().borrow_mut().right=None;
        //         }
        //         curr.as_ref().unwrap().borrow_mut().left = parent;
        //      }
        //     curr.as_ref().unwrap().borrow_mut().parent = new_parent;
        //     let left = curr.as_ref().unwrap().borrow_mut().left.take();
        //     new_parent = curr;
        //     curr = left;
        //     println!("{:?}",&curr);
        //     i+=1;
        // }
        // curr.as_ref().unwrap().borrow_mut().parent = new_parent;
        // leaf
        // fn dfs(
        //     root: Option<Rc<RefCell<TreeNode>>>,
        //     node: Option<Rc<RefCell<TreeNode>>>,
        //     new_parent: Option<Rc<RefCell<TreeNode>>>,
        // ) -> Option<Rc<RefCell<TreeNode>>> {
        //     // let old_parent = node.as_ref().unwrap().borrow_mut().parent.take();
        //     // node.as_ref().unwrap().borrow_mut().parent = new_parent.clone();
        //     // println!("===={:?}",&node);
        //     // if  node.as_ref().unwrap().borrow().left== new_parent {
        //     //     node.as_ref().unwrap().borrow_mut().left = None;
        //     // }
        //     // if node.as_ref().unwrap().borrow().right== new_parent {
        //     //     node.as_ref().unwrap().borrow_mut().right = None;
        //     // }

        //     // if node == root{
        //     //     return node;
        //     // }
        //     // if node.as_ref().unwrap().borrow().left.is_some() {
        //     //     let left= node.as_ref().unwrap().borrow_mut().left.take();
        //     //     node.as_ref().unwrap().borrow_mut().right =left;
        //     // }
        //     // node.as_ref().unwrap().borrow_mut().left = dfs(root, old_parent, node.clone());
        //     // node
        //     if node == root{
        //         return node;
        //     }
        //     if node.as_ref().unwrap().borrow().parent.is_none(){
        //     return new_parent
        //     }
        //     // dfs(root,node.as_ref().unwrap().borrow().parent.clone(),new_parent)
        //     None
        // }
        // println!("===={:?}",&root);
        // dfs(root, leaf, None)
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, v: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let node = root.as_ref().unwrap().borrow();
            if node.val == v {
                return root.clone();
            }
            let left = dfs(&node.left, v);
            if left.is_some() {
                return left;
            }
            dfs(&node.right, v)
        } else {
            None
        }
    }
    #[test]
    pub fn test_flip_binary_tree_1() {
        let mut t = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let leaf = dfs(&t, 7);
        assert_eq!(
            tree![7, 2, null, 5, 4, 3, 6, null, null, null, 1, null, null, 0, 8],
            Solution::flip_binary_tree(t, leaf)
        );
    }
    #[test]
    pub fn test_flip_binary_tree_2() {
        let mut t = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let leaf = dfs(&t, 0);
        assert_eq!(
            tree![0, 1, null, 3, 8, 5, null, null, null, 6, 2, null, null, 7, 4],
            Solution::flip_binary_tree(t, leaf)
        );
    }
}
