// 3879. Maximum Distinct Path Sum in a Binary Tree
// ## Description
// You are given the `root` of a **binary tree**, where each node contains an integer value.
// A **valid path** in the tree is a sequence of **connected** nodes such that:
// * The path can start and end at **any node** in the tree.
// * The path does **not** need to pass through the root.
// * All node values along the path are **distinct**.
// Return an integer denoting the **maximum** possible sum of node values among all valid paths.

// **Example 1:**
// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3879.Maximum%20Distinct%20Path%20Sum%20in%20a%20Binary%20Tree/images/screenshot-2026-01-29-at-12940am.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3879.Maximum Distinct Path Sum in a Binary Tree/images/screenshot-2026-01-29-at-12940am.png>)
// **Input:** root = [2,2,1]
// **Output:** 3
// **Explanation:**
// * The path `2 → 2` is invalid because the value 2 is not distinct.
// * The maximum-sum valid path is `2 → 1`, with a sum = `2 + 1 = 3`.
// **Example 2:**
// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3879.Maximum%20Distinct%20Path%20Sum%20in%20a%20Binary%20Tree/images/screenshot-2026-01-29-at-15149am.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3879.Maximum Distinct Path Sum in a Binary Tree/images/screenshot-2026-01-29-at-15149am.png>)
// **Input:** root = [1,-2,5,null,null,3,5]
// **Output:** 9
// **Explanation:**
// * The path `3 → 5 → 5` is invalid due to duplicate value 5.
// * The maximum-sum valid path is `1 → 5 → 3`, with a sum = `1 + 5 + 3 = 9`.
// **Example 3:**
// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3879.Maximum%20Distinct%20Path%20Sum%20in%20a%20Binary%20Tree/images/screenshot-2026-01-29-at-15555am.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3879.Maximum Distinct Path Sum in a Binary Tree/images/screenshot-2026-01-29-at-15555am.png>)​​​​​​​
// **Input:** root = [4,6,6,null,null,null,9]
// **Output:** 19
// **Explanation:**
// * The path `6 → 4 → 6 → 9` is invalid because the value 6 appears more than once.
// * The maximum-sum valid path is `4 → 6 → 9`, with a sum = `4 + 6 + 9 = 19`.

// **Constraints:**
// * The number of nodes in the tree is in the range `[1, 1000]`.
// * `-1000 \<= Node.val \<= 1000​​​​​​​`

// int max_sum(TreeNode* root) {

use crate::solutions::util::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
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
    pub fn test_max_sum_1() {
        assert_eq!(Solution::max_sum(tree![2,2,1]), 3);
    }
 #[test]
    pub fn test_max_sum_2() {
        assert_eq!(Solution::max_sum(tree![1,-2,5,null,null,3,5]), 9);
    }
 #[test]
    pub fn test_max_sum_3() {
        assert_eq!(Solution::max_sum(tree![4,6,6,null,null,null,9]), 19);
    }
}
