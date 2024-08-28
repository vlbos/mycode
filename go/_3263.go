

// 3263. Convert Doubly Linked List to Array I
// Easy

// You are given the `head` of a **doubly linked list**, which contains nodes that have a next pointer and a previous pointer.

// Return an integer array which contains the elements of the linked list **in order**.

// **Example 1:**

// **Input:** head = \[1,2,3,4,3,2,1\]

// **Output:** \[1,2,3,4,3,2,1\]

// **Example 2:**

// **Input:** head = \[2,2,2,2,2\]

// **Output:** \[2,2,2,2,2\]

// **Example 3:**

// **Input:** head = \[3,2,3,2,3,2\]

// **Output:** \[3,2,3,2,3,2\]

// **Constraints:**

// *   The number of nodes in the given list is in the range `[1, 50]`.
// *   `1 <= Node.val <= 50`

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Next *Node
 *     Prev *Node
 * }
 */

func toArray(head *Node) []int {
    ans:=[]int{}
    p:=head
    for p!=nil{
        ans=append(ans,p.Val)
        p=p.Next
    }
    return ans
}