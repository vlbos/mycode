// [3294\. Convert Doubly Linked List to Array II 🔒](https://leetcode.com/problems/convert-doubly-linked-list-to-array-ii)
// ========================================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// Description
// -----------

// You are given an **arbitrary** `node` from a **doubly linked list**, which contains nodes that have a next pointer and a previous pointer.

// Return an integer array which contains the elements of the linked list **in order**.

// **Example 1:**

// **Input:** head = \[1,2,3,4,5\], node = 5

// **Output:** \[1,2,3,4,5\]

// **Example 2:**

// **Input:** head = \[4,5,6,7,8\], node = 8

// **Output:** \[4,5,6,7,8\]

// **Constraints:**

// *   The number of nodes in the given list is in the range `[1, 500]`.
// *   `1 <= Node.val <= 1000`
// *   All nodes have unique `Node.val`.


```

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Next *Node
 *     Prev *Node
 * }
 */

func toArray(node *Node) (ans []int) {
    for node != nil && node.Prev != nil {
        node = node.Prev
    }
    for ; node != nil; node = node.Next {
        ans = append(ans, node.Val)
    }
    return
}













```

