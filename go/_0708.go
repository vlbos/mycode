/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Next *Node
 * }
 */

func inserterror(head *Node, x int) *Node {
    node := &Node{Val: x}
    if aNode == nil {
        node.Next = node
        return node
    }
    if aNode.Next == aNode {
        aNode.Next = node
        aNode.Next = aNode
        return aNode
    }
    curr, next := aNode, aNode.Next
    for next != aNode {
        if x >= curr.Val && x <= next.Val {
            break
        }
        if curr.Val > next.Val {
            if x > curr.Val || x < next.Val {
                break
            }
        }
        curr = curr.Next
        next = next.Next
    }
    curr.Next = node
    node.Next = next
    return aNode
}


func insert(aNode *Node, x int) *Node {
	n := &Node{
		Val: x,
	}

	// no node
	if aNode == nil {
		n.Next = n
		aNode = n
	} else  {
		node := aNode
		for node.Next != aNode {
			if node.Val <= node.Next.Val {
				if node.Val <= x && x <= node.Next.Val {
					break
				}
			} else {
				if node.Val <= x || x <= node.Next.Val {
					break
				}
			}
			node = node.Next
		}
		
		node.Next, n.Next = n, node.Next
	}
	
	return aNode
}

// head =
// [1]
// insertVal =
// 0

// Use Testcase
// Output
// [1]
// Expected
// [1,0]