/**
 * Definition for Node.
 * type Node struct {
 *     Val int
 *     Left *Node
 *     Right *Node
 *     Parent *Node
 * }
 */

func inorderSuccessor(node *Node) *Node {
	var findRoot func(cur *Node) *Node
	findRoot = func(cur *Node) *Node {
		if cur.Parent == nil {
			return cur
		}
		return findRoot(cur.Parent)
	}
	root := findRoot(node)
	//fmt.Printf("root=%d\n",root.Val)
	var findMostLeft func(cur *Node) *Node
	findMostLeft = func(cur *Node) *Node {
		if cur == nil || cur.Left == nil {
			return cur
		}
		return findMostLeft(cur.Left)
	}

	var dfs func(r *Node, p *Node) *Node
	dfs = func(r *Node, p *Node) *Node {
		if r == nil {
			return r
		}
		if r.Val == p.Val {
			return findMostLeft(r.Right)
		}
		if r.Val < p.Val {
			return dfs(r.Right, p)
		}
		t := dfs(r.Left, p)
		if t == nil {
			return r
		}
		return t
	}
	return dfs(root, node)
}