// 1740\. Find Distance in a Binary Tree
// =====================================

// Given the root of a binary tree and two integers `p` and `q`, return _the **distance** between the nodes of value_ `p` _and value_ `q` _in the tree_.

// The **distance** between two nodes is the number of edges on the path from one to the other.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], p = 5, q = 0
// **Output:** 3
// **Explanation:** There are 3 edges between 5 and 0: 5-3-1-0.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], p = 5, q = 7
// **Output:** 2
// **Explanation:** There are 2 edges between 5 and 7: 5-2-7.

// **Example 3:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], p = 5, q = 5
// **Output:** 0
// **Explanation:** The distance between a node and itself is 0.

// **Constraints:**

// *   The number of nodes in the tree is in the range `[1, 104]`.
// *   `0 <= Node.val <= 109`
// *   All `Node.val` are **unique**.
// *   `p` and `q` are values in the tree.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

//  int find_distance(TreeNode root, int p, int q)

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_distance(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> i32 {
        if p == q {
            return 0;
        }
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, v: i32, path: &mut Vec<i32>) -> bool {
            if root.is_none() {
                return false;
            }

            let node = root.as_ref().unwrap().borrow();
            path.push(node.val);
            if node.val == v || dfs(&node.left, v, path) || dfs(&node.right, v, path) {
                return true;
            }
            path.pop();
            false
        }
        let mut pp = Vec::new();
        dfs(&root, p, &mut pp);
        let mut pq = Vec::new();
        dfs(&root, q, &mut pq);
        let mut ans = (pq.len() + pp.len()) as i32;
        for (p1, p2) in pp.into_iter().zip(pq) {
            if p1 != p2 {
                break;
            }
            ans -= 2;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    #[test]
    pub fn test_find_distance_1() {
        assert_eq!(
            3,
            Solution::find_distance(tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], 5, 0)
        );
    }
    #[test]
    pub fn test_find_distance_2() {
        assert_eq!(
            2,
            Solution::find_distance(tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], 5, 7)
        );
    }
    #[test]
    pub fn test_find_distance_3() {
        assert_eq!(
            0,
            Solution::find_distance(tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], 5, 5)
        );
    }
}
