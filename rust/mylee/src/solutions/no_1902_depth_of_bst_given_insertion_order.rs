// 1902\. Depth of BST Given Insertion Order[](https://leetcode.ca/2021-07-28-1902-Depth-of-BST-Given-Insertion-Order/#1902-depth-of-bst-given-insertion-order)
// ============================================================================================================================================================

// Level[](https://leetcode.ca/2021-07-28-1902-Depth-of-BST-Given-Insertion-Order/#level)
// --------------------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-07-28-1902-Depth-of-BST-Given-Insertion-Order/#description)
// --------------------------------------------------------------------------------------------------

// You are given a **0-indexed** integer array `order` of length `n`, a **permutation** of integers from `1` to `n` representing the **order** of insertion into a **binary search tree**.

// A binary search tree is defined as follows:

// *   The left subtree of a node contains only nodes with keys **less than** the node’s key.
// *   The right subtree of a node contains only nodes with keys **greater than** the node’s key.
// *   Both the left and right subtrees must also be binary search trees.

// The binary search tree is constructed as follows:

// *   `order[0]` will be the **root** of the binary search tree.
// *   All subsequent elements are inserted as the **child** of **any** existing node such that the binary search tree properties hold.

// Return _the **depth** of the binary search tree_.

// A binary tree’s **depth8\* is the number of \*\*nodes** along the **longest path** from the root node down to the farthest leaf node.

// **Example 1:**

// ![Image text](https://assets.leetcode.com/uploads/2021/06/15/1.png)

// **Input:** order = \[2,1,4,3\]

// **Output:** 3

// **Explanation:** The binary search tree has a depth of 3 with path 2->3->4.

// **Example 2:**

// ![Image text](https://assets.leetcode.com/uploads/2021/06/15/2.png)

// **Input:** order = \[2,1,3,4\]

// **Output:** 3

// **Explanation:** The binary search tree has a depth of 3 with path 2->3->4.

// **Example 3:**

// ![Image text](https://assets.leetcode.com/uploads/2021/06/15/3.png)

// **Input:** order = \[1,2,3,4\]

// **Output:** 4

// **Explanation:** The binary search tree has a depth of 4 with path 1->2->3->4.

// **Constraints:**

// *   `n == order.length`
// *   `1 <= n <= 10^5`
// *   `order` is a permutation of integers between `1` and `n`.

// Solution[](https://leetcode.ca/2021-07-28-1902-Depth-of-BST-Given-Insertion-Order/#solution)
// --------------------------------------------------------------------------------------------

// Use a TreeMap to store each value’s depth. Initially, value `order[0]` has depth 1.

// Loop over the remaining elements in `order`. For each value `curr`, find the two adjacent values of `curr` in the TreeMap, which are `prev` and `next` such that `prev <= curr <= next`. If both values exist (not `null`), then since both `prev` and `next` come before `curr`, the depth of `curr` is the maximum of the depths of `prev` and `next` plus 1. If only one value exists, then the depth of `curr` is the depth of the existing value plus 1.

// Maintain the maximum depth during the process. Finally, return the maximum depth.

//     class Solution {
//         public int maxDepthBST(int[] order) {

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
