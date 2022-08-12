// 545\. Boundary of Binary Tree
// =============================

// Given a binary tree, return the values of its boundary in **anti-clockwise** direction starting from root.
// Boundary includes left boundary, leaves, and right boundary in order without duplicate **nodes**.  (The values of the nodes may still be duplicates.)

// **Left boundary** is defined as the path from root to the **left-most** node.
// **Right boundary** is defined as the path from root to the **right-most** node.
//  If the root doesn't have left subtree or right subtree, then the root itself is left boundary or right boundary.
// Note this definition only applies to the input binary tree, and not applies to any subtrees.

// The **left-most** node is defined as a **leaf** node you could reach when you always firstly travel to the left subtree if exists.
// If not, travel to the right subtree. Repeat until you reach a leaf node.

// The **right-most** node is also defined by the same way with left and right exchanged.

// **Example 1**

// **Input:**
//   1
//    \\
//     2
//    / \\
//   3   4

// **Ouput:**
// \[1, 3, 4, 2\]

// **Explanation:**
// The root doesn't have left subtree, so the root itself is left boundary.
// The leaves are node 3 and 4.
// The right boundary are node 1,2,4. Note the anti-clockwise direction means you should output reversed right boundary.
// So order them in anti-clockwise without duplicates and we have \[1,3,4,2\].

// **Example 2**

// **Input:**
//     \_\_\_\_1\_\_\_\_\_
//    /          \\
//   2            3
//  / \\          /
// 4   5        6
//    / \\      / \\
//   7   8    9  10

// **Ouput:**
// \[1,2,4,7,8,9,10,6,3\]

// **Explanation:**
// The left boundary are node 1,2,4. (4 is the left-most node according to definition)
// The leaves are node 4,7,8,9,10.
// The right boundary are node 1,3,6,10. (10 is the right-most node).
// So order them in anti-clockwise without duplicate nodes we have \[1,2,4,7,8,9,10,6,3\].

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Atlassian](https://leetcode.ca/tags/#Atlassian) [eBay](https://leetcode.ca/tags/#eBay) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Oracle](https://leetcode.ca/tags/#Oracle)

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
//   pub fn   new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::solutions::util::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// enum BoundaryState {
//     Root,
//     Left,
//     Right,
//     Leaf,
//     Unknown,
// }

impl Solution {
    //pub fn  boundary_of_binary_tree_recursive(
    //     curr: Option<Rc<RefCell<TreeNode>>>,
    //     mut bs: BoundaryState,
    //     mut res: &mut Vec<i32>,
    // ) {
    //     if let Some(curr_ref) = curr {
    //         let mut curr_node = curr_ref.borrow_mut();
    //         if curr_node.left.is_none() && curr_node.right.is_none() {
    //             bs = BoundaryState::Leaf;
    //         }
    //         match bs {
    //             BoundaryState::Root => {
    //                 res.push(curr_node.val);
    //                 Solution::boundary_of_binary_tree_recursive(
    //                     curr_node.left.take(),
    //                     BoundaryState::Left,
    //                     &mut res,
    //                 );
    //                 Solution::boundary_of_binary_tree_recursive(
    //                     curr_node.right.take(),
    //                     BoundaryState::Right,
    //                     &mut res,
    //                 );
    //             }
    //             BoundaryState::Left => {
    //                 res.push(curr_node.val);
    //                 let left_empty = curr_node.left.is_none();
    //                 Solution::boundary_of_binary_tree_recursive(
    //                     curr_node.left.take(),
    //                     BoundaryState::Left,
    //                     &mut res,
    //                 );
    //                 Solution::boundary_of_binary_tree_recursive(
    //                     curr_node.right.take(),
    //                     if left_empty {
    //                         BoundaryState::Left
    //                     } else {
    //                         BoundaryState::Unknown
    //                     },
    //                     &mut res,
    //                 );
    //             }
    //             BoundaryState::Right => {
    //                 let right_empty = curr_node.right.is_none();
    //                 Solution::boundary_of_binary_tree_recursive(
    //                     curr_node.left.take(),
    //                     if right_empty {
    //                         BoundaryState::Right
    //                     } else {
    //                         BoundaryState::Unknown
    //                     },
    //                     &mut res,
    //                 );
    //                 Solution::boundary_of_binary_tree_recursive(
    //                     curr_node.right.take(),
    //                     BoundaryState::Right,
    //                     &mut res,
    //                 );
    //                 res.push(curr_node.val);
    //             }
    //             BoundaryState::Unknown => {
    //                 Solution::boundary_of_binary_tree_recursive(
    //                     curr_node.left.take(),
    //                     BoundaryState::Unknown,
    //                     &mut res,
    //                 );
    //                 Solution::boundary_of_binary_tree_recursive(
    //                     curr_node.right.take(),
    //                     BoundaryState::Unknown,
    //                     &mut res,
    //                 );
    //             }
    //             BoundaryState::Leaf => {
    //                 res.push(curr_node.val);
    //             }
    //         }
    //     }
    // }

    pub fn   boundary_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // let mut res = vec![];
        // Solution::boundary_of_binary_tree_recursive(root, BoundaryState::Root, &mut res);
        // res
        let (mut ans, mut left, mut right, mut leaf) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());

       pub fn  in_order(root: &Option<Rc<RefCell<TreeNode>>>, leaf: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                in_order(&node.left, leaf);
                if node.left.is_none() && node.right.is_none() {
                    leaf.push(node.val);
                }
                in_order(&node.right, leaf);
            }
        }
       pub fn  left_order(root: &Option<Rc<RefCell<TreeNode>>>, left: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                left.push(node.val);
                left_order(&node.left, left);
            }
        }
       pub fn  right_order(root: &Option<Rc<RefCell<TreeNode>>>, right: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                right.push(node.val);
                right_order(&node.right, right);
            }
        }
        left_order(&root, &mut left);
        in_order(&root, &mut leaf);
        right_order(&root, &mut right);
        right.reverse();
        let mut dup = std::collections::HashSet::new();
        for &v in left.iter().chain(&leaf).chain(&right) {
            if !dup.contains(&v) {
                ans.push(v);
                dup.insert(v);
            }
        }
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
   pub fn  test_boundary_of_binary_tree_1() {
        let tree = tree![1, null, 2, 3, 4];
        assert_eq!(Solution::boundary_of_binary_tree(tree), vec![1, 3, 4, 2]);
    }

    #[test]
   pub fn  test_boundary_of_binary_tree_2() {
        let tree = tree![1, 2, 3, 4, 5, 6, null, 7, 8, 9, 10];
        assert_eq!(
            Solution::boundary_of_binary_tree(tree),
            vec![1, 2, 4, 7, 8, 9, 10, 6, 3]
        );
    }
}
