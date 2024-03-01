// 1469\. Find All the Lonely Nodes
// ================================

// In a binary tree, a **lonely** node is a node that is the only child of its parent node. The root of the tree is not lonely because it does not have a parent node.

// Given the `root` of a binary tree, return _an array containing the values of all lonely nodes_ in the tree. Return the list **in any order**.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2020/06/03/e1.png)

// **Input:** root = \[1,2,3,null,4\]
// **Output:** \[4\]
// **Explanation:** Light blue node is the only lonely node.
// Node 1 is the root and is not lonely.
// Nodes 2 and 3 have the same parent and are not lonely.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2020/06/03/e2.png)

// **Input:** root = \[7,1,4,6,null,5,3,null,null,null,null,null,2\]
// **Output:** \[6,2\]
// **Explanation:** Light blue nodes are lonely nodes.
// Please remember that order doesn't matter, \[2,6\] is also an acceptable answer.

// **Example 3:**

// **![](https://assets.leetcode.com/uploads/2020/06/03/tree.png)**

//  **Input:** root = \[11,99,88,77,null,null,66,55,null,null,44,33,null,null,22\]
// **Output:** \[77,55,33,66,44,22\]
// **Explanation:** Nodes 99 and 88 share the same parent. Node 11 is the root.
// All other nodes are lonely.

// **Example 4:**

// **Input:** root = \[197\]
// **Output:** \[\]

// **Example 5:**

// **Input:** root = \[31,null,78,null,28\]
// **Output:** \[78,28\]

// **Constraints:**

// *   The number of nodes in the `tree` is in the range `[1, 1000].`
// *   Each node's value is between `[1, 10^6]`.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Microsoft](https://leetcode.ca/tags/#Microsoft)
use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_lonely_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            if node.left.is_some() {
                dfs(&node.left, ans);
                if node.right.is_none() {
                    ans.push(node.left.as_ref().unwrap().borrow().val);
                }
            }
            if node.right.is_some() {
                dfs(&node.right, ans);
                if node.left.is_none() {
                    ans.push(node.right.as_ref().unwrap().borrow().val);
                }
            }
        }
        dfs(&root, &mut ans);
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    use std::collections::HashSet;
    #[test]
    pub fn test_get_lonely_nodes_1() {
        assert_eq!(vec![4], Solution::get_lonely_nodes(tree![1, 2, 3, null, 4]));
    }
    #[test]
    pub fn test_get_lonely_nodes_2() {
        assert_eq!(
            HashSet::from([6, 2]),
            Solution::get_lonely_nodes(tree![
                7, 1, 4, 6, null, 5, 3, null, null, null, null, null, 2
            ])
            .into_iter()
            .collect::<HashSet<i32>>()
        );
    }
    #[test]
    pub fn test_get_lonely_nodes_3() {
        assert_eq!(
            HashSet::from([77, 55, 33, 66, 44, 22]),
            Solution::get_lonely_nodes(tree![
                11, 99, 88, 77, null, null, 66, 55, null, null, 44, 33, null, null, 22
            ])
            .into_iter()
            .collect::<HashSet<i32>>()
        );
    }
    #[test]
    pub fn test_get_lonely_nodes_4() {
        assert_eq!(Vec::<i32>::new(), Solution::get_lonely_nodes(tree![197]));
    }
    #[test]
    pub fn test_get_lonely_nodes_5() {
        assert_eq!(
            HashSet::from([78, 28]),
            Solution::get_lonely_nodes(tree![31, null, 78, null, 28])
                .into_iter()
                .collect::<HashSet<i32>>()
        );
    }
}
