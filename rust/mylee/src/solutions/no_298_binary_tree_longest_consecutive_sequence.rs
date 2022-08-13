// 298\. Binary Tree Longest Consecutive Sequence
// ==============================================

// Given a binary tree, find the length of the longest consecutive sequence path.

// The path refers to any sequence of nodes from some starting node to any node in the tree along the parent-child connections.
// The longest consecutive path need to be from parent to child (cannot be the reverse).

// **Example 1:**

// **Input:**

//    1
//     \\
//      3
//     / \\
//    2   4
//         \\
//          5

// **Output:** `3`

// **Explanation:** Longest consecutive sequence path is `3-4-5`, so return `3`.

// **Example 2:**

// **Input:

// **   2
//     \\
//      3
//     /
//    2
//   /
//  1

// **Output: 2

// Explanation:** Longest consecutive sequence path is `2-3`, not `3-2-1`, so return `2`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Uber](https://leetcode.ca/tags/#Uber)
use super::util::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Solution::longest_consecutive_rec(&root).2 as i32
        pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                let l = dfs(&node.left, ans);
                let r = dfs(&node.right, ans);
                let mut max = 1;

                if node.left.is_none() || node.left.as_ref().unwrap().borrow().val == node.val + 1 {
                    max = max.max(l + 1);
                }
                if node.right.is_none() || node.right.as_ref().unwrap().borrow().val == node.val + 1
                {
                    max = max.max(r + 1);
                }
                *ans = max.max(*ans);
                max
            } else {
                0
            }
        }
        let mut ans = 0;
        dfs(&root, &mut ans);
        ans
    }

    //pub fn  longest_consecutive_rec(root: &Option<Rc<RefCell<TreeNode>>>) -> (usize, i32, usize) {
    //     match root {
    //         Some(node_ref) => {
    //             let node_borrow = node_ref.borrow();
    //             let mut res = (1, node_borrow.val, 1);
    //             let left_res = Solution::longest_consecutive_rec(&node_borrow.left);
    //             let right_res = Solution::longest_consecutive_rec(&node_borrow.right);
    //             if left_res.0 != 0 || left_res.1 != 0 {
    //                 if node_borrow.val + 1 == left_res.1 {
    //                     res.0 = usize::max(1 + left_res.0, res.0);
    //                 }
    //             }
    //             if right_res.0 != 0 || right_res.1 != 0 {
    //                 if node_borrow.val + 1 == right_res.1 {
    //                     res.0 = usize::max(1 + right_res.0, res.0);
    //                 }
    //             }
    //             res.2 = usize::max(
    //                 usize::max(res.2, res.0),
    //                 usize::max(left_res.2, right_res.2),
    //             );
    //             res
    //         }
    //         None => (0, 0, 0),
    //     }
    // }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
    pub fn test_longest_consecutive1() {
        let tree = tree![1, null, 3, 2, 4, null, null, null, 5];
        assert_eq!(Solution::longest_consecutive(tree), 3);
    }

    #[test]
    pub fn test_longest_consecutive2() {
        let tree = tree![2, null, 3, 2, null, 1];
        assert_eq!(Solution::longest_consecutive(tree), 2);
    }
}
