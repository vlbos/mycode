/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Left *Node
 *     Right *Node
 * }
 */

func treeToDoublyList(root *Node) *Node {
	if root == nil {
		return nil
	}
	first := &Node{}
	last := first
	var dfs func(*Node)
	dfs = func(node *Node) {
		if node == nil {
			return
		}
		dfs(node.Left)
		last.Right = node
		node.Left = last
		last = last.Right
		dfs(node.Right)
	}
	dfs(root)
	//构造环
	head := first.Right
	head.Left = last
	last.Right = head
	return head
}