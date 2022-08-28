// 1973\. Count Nodes Equal to Sum of Descendants[](https://leetcode.ca/2021-08-18-1973-Count-Nodes-Equal-to-Sum-of-Descendants/#1973-count-nodes-equal-to-sum-of-descendants)
// ===========================================================================================================================================================================

// Level[](https://leetcode.ca/2021-08-18-1973-Count-Nodes-Equal-to-Sum-of-Descendants/#level)
// -------------------------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-08-18-1973-Count-Nodes-Equal-to-Sum-of-Descendants/#description)
// -------------------------------------------------------------------------------------------------------

// Given the `root` of a binary tree, return _the number of nodes where the value of the node is equal to the **sum** of the values of its descendants_.

// A **descendant** of a node `x` is any node that is on the path from node `x` to some leaf node. The sum is considered to be `0` if the node has no descendants.

// **Example 1:**

// ![Image text](https://assets.leetcode.com/uploads/2021/08/17/screenshot-2021-08-17-at-17-16-50-diagram-drawio-diagrams-net.png)

// **Input:** root = \[10,3,4,2,1\]

// **Output:** 2

// **Explanation:**

// For the node with value 10: The sum of its descendants is 3+4+2+1 = 10.

// For the node with value 3: The sum of its descendants is 2+1 = 3.

// **Example 2:**

// ![Image text](https://assets.leetcode.com/uploads/2021/08/17/screenshot-2021-08-17-at-17-25-21-diagram-drawio-diagrams-net.png)

// **Input:** root = \[2,3,null,2,null\]

// **Output:** 0

// **Explanation:**

// No node has a value that is equal to the sum of its descendants.

// **Example 3:**

// ![Image text](https://assets.leetcode.com/uploads/2021/08/17/screenshot-2021-08-17-at-17-23-53-diagram-drawio-diagrams-net.png)

// **Input:** root = \[0\]

// **Output:** 1

// For the node with value 0: The sum of its descendants is 0 since it has no descendants.

// Constraints:

// *   The number of nodes in the tree is in the range `[1, 10^5]`.
// *   `0 <= Node.val <= 10^5`

// Solution[](https://leetcode.ca/2021-08-18-1973-Count-Nodes-Equal-to-Sum-of-Descendants/#solution)
// -------------------------------------------------------------------------------------------------

// Do depth first search on the binary tree. For each node, calculate the sum of descedants and check whether the sum of descedants is equal to the node’s value. If so, add the counter by 1. Finally, return the counter.

//     /**
//      * Definition for a binary tree node.
//      * public class TreeNode {
//      *     int val;
//      *     TreeNode left;
//      *     TreeNode right;
//      *     TreeNode() {}
//      *     TreeNode(int val) { this.val = val; }
//      *     TreeNode(int val, TreeNode left, TreeNode right) {
//      *         this.val = val;
//      *         this.left = left;
//      *         this.right = right;
//      *     }
//      * }
//      */
//     class Solution {
//         int count = 0;

//         public int equalToDescendants(TreeNode root) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_word_1() {
        assert_eq!(
            "kiran".to_string(),
            Solution::longest_word(
                ["k", "ki", "kir", "kira", "kiran"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_2() {
        assert_eq!(
            "apple".to_string(),
            Solution::longest_word(
                ["a", "banana", "app", "appl", "ap", "apply", "apple"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_3() {
        assert_eq!(
            String::new(),
            Solution::longest_word(
                ["abc", "bc", "ab", "qwe"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
}
