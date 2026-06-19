// 3831. Median of a Binary Search Tree Level
// ## Description
// You are given the `root` of a **Binary Search Tree (BST)** and an integer `level`.
// The root node is at level 0. Each level represents the distance from the root.
// Return the **median value** of all node values present at the given `level`. If the level does not exist or contains no nodes, return -1.
// The **median** is defined as the middle element after sorting the values at that level in **non-decreasing** order. If the number of values at that level is even, return the **upper** median (the larger of the two middle elements after sorting).

// **Example 1:**
// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3831.Median%20of%20a%20Binary%20Search%20Tree%20Level/images/screenshot-2026-01-27-at-20801pm.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3831.Median of a Binary Search Tree Level/images/screenshot-2026-01-27-at-20801pm.png>)
// **Input:** root = [4,null,5,null,7], level = 2
// **Output:** 7
// **Explanation:**
// The nodes at `level = 2` are `[7]`. The median value is 7.
// **Example 2:**
// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3831.Median%20of%20a%20Binary%20Search%20Tree%20Level/images/screenshot-2026-01-27-at-20926pm.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3831.Median of a Binary Search Tree Level/images/screenshot-2026-01-27-at-20926pm.png>)
// **Input:** root = [6,3,8], level = 1
// **Output:** 8
// **Explanation:**
// The nodes at `level = 1` are `[3, 8]`. There are two possible median values, so the larger one 8 is the answer.
// **Example 3:**
// **​​​​​​​​​​​​​​**[![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3831.Median%20of%20a%20Binary%20Search%20Tree%20Level/images/screenshot-2026-01-27-at-21001pm.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3831.Median of a Binary Search Tree Level/images/screenshot-2026-01-27-at-21001pm.png>)
// **Input:** root = [2,1], level = 2
// **Output:** -1
// **Explanation:**
// There is no node present at `level = 2`​​​​​​​, so the answer is -1.

// **Constraints:**
// * The number of nodes in the tree is in the range `[1, 2 \* 105]`.
// * `1 \<= Node.val \<= 106`
// * `0 \<= level \<= 2 \* 10​​​​​​​5`

// int level_median(TreeNode* root, int level) {

use super::util::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_median(root: Option<Rc<RefCell<TreeNode>>>, level: i32) -> i32 {
        0
    }
}

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
    pub fn test_level_median_rec_1() {
        assert_eq!(
            Solution::level_median(tree![4,null,5,null,7], 2),
            7
        );
    }

    #[test]
    pub fn test_level_median_rec_2() {
        assert_eq!(
            Solution::level_median(tree![6,3,8], 1),
            8
        );
    }

    #[test]
    pub fn test_level_median_rec_3() {
        assert_eq!(
            Solution::level_median(tree![2,1], 2),
            -1
        );
    }
}
