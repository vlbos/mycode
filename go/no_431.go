/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

type Codec struct {
}

func Constructor() *Codec {
	return &Codec{}
}

func (this *Codec) encode(root *Node) *TreeNode {

	if root == nil {
		return nil
	}

	rt := &TreeNode{}
	n2t := make(map[*Node]*TreeNode)

	n2t[root] = rt
	rt.Val = root.Val

	q := make([]*Node, 0)
	q = append(q, root)
	for len(q) > 0 {
		cn := q[0]
		q = q[1:]

		ct := n2t[cn]

		for i, child := range cn.Children {
			q = append(q, child)
			nt := &TreeNode{}
			nt.Val = child.Val
			n2t[child] = nt

			if i > 0 { // 兄弟在左
				ct.Left = nt
			} else { // 孩子在右
				ct.Right = nt
			}
			ct = nt
		}

	}

	return n2t[root]
}

func (this *Codec) decode(root *TreeNode) *Node {

	if root == nil {
		return nil
	}

	t2n := make(map[*TreeNode]*Node)
	rn := &Node{root.Val, make([]*Node, 0)}
	t2n[root] = rn

	q := make([]*TreeNode, 0)
	q = append(q, root)

	for len(q) > 0 {
		ctn := q[0]
		q = q[1:]

		cn := t2n[ctn]

		child := ctn.Right
		for child != nil {
			q = append(q, child)
			nc := &Node{child.Val, make([]*Node, 0)}
			t2n[child] = nc
			cn.Children = append(cn.Children, nc)
			child = child.Left
		}

	}

	return t2n[root]
}

/**
 * Your Codec object will be instantiated and called as such:
 * obj := Constructor();
 * bst := obj.encode(root);
 * ans := obj.decode(bst);
 */