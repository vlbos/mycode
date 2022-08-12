// 1120\. Maximum Average Subtree
// ==============================

// Given the `root` of a binary tree, find the maximum average value of any subtree of that tree.

// (A subtree of a tree is any node of that tree plus all its descendants. The average value of a tree is the sum of its values, divided by the number of nodes.)

// **Example 1:**

// ![](https://leetcode.ca/all/img/1120.png)

// **Input:** \[5,6,1\]
// **Output:** 6.00000
// **Explanation:**
// For the node with value = 5 we have an average of (5 + 6 + 1) / 3 = 4.
// For the node with value = 6 we have an average of 6 / 1 = 6.
// For the node with value = 1 we have an average of 1 / 1 = 1.
// So the answer is 6 which is the maximum.

// **Note:**

// 1.  The number of nodes in the tree is between `1` and `5000`.
// 2.  Each node will have a value between `0` and `100000`.
// 3.  Answers will be accepted as correct if they are within `10^-5` of the correct answer.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Atlassian](https://leetcode.ca/tags/#Atlassian)

// Definition for a binary tree node
// #[derive(Debug, PartialEq, Eq)]
// pub  struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     #[inline]
//     pub fn   new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

// pub fn   to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
//     use std::collections::VecDeque;
//     let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
//     let mut queue = VecDeque::new();
//     queue.push_back(head.as_ref().unwrap().clone());

//     for children in vec[1..].chunks(2) {
//         let parent = queue.pop_front().unwrap();
//         if let Some(v) = children[0] {
//             parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
//             queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
//         }
//         if children.len() > 1 {
//             if let Some(v) = children[1] {
//                 parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
//                 queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
//             }
//         }
//     }
//     head
// }
use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn   maximum_average_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
       pub fn  dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut f64) -> (i32, i32) {
            if root.is_none() {
                return (0, 0);
            }
            let node = root.as_ref().unwrap().borrow();
            let (left_sum, left_cnt) = dfs(&node.left, ans);
            let (right_sum, right_cnt) = dfs(&node.right, ans);
            let (sum, cnt) = (left_sum + right_sum + node.val, left_cnt + right_cnt + 1);
            *ans = (*ans).max((node.val as f64).max(sum as f64 / cnt as f64));
            (sum, cnt)
        }
        let mut ans = 0.0;
        dfs(&root, &mut ans);
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    #[test]
   pub fn  test_maximum_average_subtree_1() {
        assert_eq!(6.0000, Solution::maximum_average_subtree(tree![5, 6, 1]));
    }
}
