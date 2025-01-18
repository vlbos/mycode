/*
Given a root of an N-ary tree, you need to compute the length of the diameter of the tree.

The diameter of an N-ary tree is the length of the longest path between any two nodes in the tree. This path may or may not pass through the root.

(Nary-Tree input serialization is represented in their level order traversal, each group of children is separated by the null value.)


Example 1:
Input: root = [1,null,3,2,4,null,5,6]
Output: 3
Explanation: Diameter is shown in red color.

Example 2:
Input: root = [1,null,2,null,3,4,null,5,null,6]
Output: 4

Example 3:
Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
Output: 7


Constraints:
    The depth of the n-ary tree is less than or equal to 1000.
    The total number of nodes is between [0, 10^4].

*/
/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

func diameter(root *Node) int {
     ans:=0
    var dfs func(node *Node) int
    dfs = func(node *Node) int{
        if node==nil{
            return 0
        }
        dep1,dep2:=0,0
        for _,n:=range node.Children{
            h:=dfs(n)
            if h>=dep1{
                dep2=dep1
                dep1=h 
            }else if h>dep2{
                dep2=h
            }
        }
        ans=max(ans,dep1+dep2)
        return max(dep1,dep2)+1
    }
    dfs(root)
    return ans
}

func max( a,b int) int{
    if a>b{
        return a
    }
    return b
}
