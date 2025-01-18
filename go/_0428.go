/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

type Codec struct {
}

func Constructor() *Codec {
	return &Codec{}
}

func (this *Codec) serialize(root *Node) string {
	if root == nil {
		return "[]"
	}
	var res = strconv.Itoa(root.Val) + ","
	if root.Children == nil {
		return res
	}
	res += "["
	for _, n := range root.Children {
		res += this.serialize(n)

	}
	return res + "]"
}

func (this *Codec) deserialize(data string) *Node {
	if data == "[]" {
		return nil
	}
	var parent = &Node{}
	var res = parent
	var sum int
	var stack []*Node
	var node *Node
	for _, v := range data {
		switch true {
		case 48 <= v && v <= 57:
			sum = sum*10 + int(v-48)
		case v == ',':
			node = &Node{Val: sum}
			parent.Children = append(parent.Children, node)
			sum = 0
		case v == '[':
			stack = append(stack, parent)
			parent = node
			sum = 0
		case v == ']':
			top := len(stack) - 1
			parent = stack[top]
			stack = stack[:top]
		}
	}
	return res.Children[0]
}

/**
 * Your Codec object will be instantiated and called as such:
 * obj := Constructor();
 * data := obj.serialize(root);
 * ans := obj.deserialize(data);
 */
