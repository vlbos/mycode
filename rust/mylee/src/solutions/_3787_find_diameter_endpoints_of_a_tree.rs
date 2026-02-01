// [3787\. Find Diameter Endpoints of a Tree 🔒](https://leetcode.com/problems/find-diameter-endpoints-of-a-tree)
// ==============================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// Description
// -----------

// You are given an **undirected tree** with `n` nodes, numbered from 0 to `n - 1`. It is represented by a 2D integer array `edges`​​​​​​​ of length `n - 1`, where `edges[i] = [ai, bi]` indicates that there is an edge between nodes `ai` and `bi` in the tree.

// A node is called **special** if it is an **endpoint** of any **diameter path** of the tree.

// Return a binary string `s` of length `n`, where `s[i] = '1'` if node `i` is special, and `s[i] = '0'` otherwise.

// A **diameter path** of a tree is the **longest** simple path between any two nodes. A tree may have multiple diameter paths.

// An **endpoint** of a path is the **first** or **last** node on that path.

// **Example 1:**

// **[![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3700-3799/3787.Find%20Diameter%20Endpoints%20of%20a%20Tree/images/pic1.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3700-3799/3787.Find%20Diameter%20Endpoints%20of%20a%20Tree/images/pic1.png)**

// **Input:** n = 3, edges = \[\[0,1\],\[1,2\]\]

// **Output:** "101"

// **Explanation:**

// *   The diameter of this tree consists of 2 edges.
// *   The only diameter path is the path from node 0 to node 2
// *   The endpoints of this path are nodes 0 and 2, so they are special.

// **Example 2:**

// **[![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3700-3799/3787.Find%20Diameter%20Endpoints%20of%20a%20Tree/images/pic2.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3700-3799/3787.Find%20Diameter%20Endpoints%20of%20a%20Tree/images/pic2.png)**

// **Input:** n = 7, edges = \[\[0,1\],\[1,2\],\[2,3\],\[3,4\],\[3,5\],\[1,6\]\]

// **Output:** "1000111"

// **Explanation:**

// The diameter of this tree consists of 4 edges. There are 4 diameter paths:

// *   The path from node 0 to node 4
// *   The path from node 0 to node 5
// *   The path from node 6 to node 4
// *   The path from node 6 to node 5

// The special nodes are nodes `0, 4, 5, 6`, as they are endpoints in at least one diameter path.

// **Example 3:**

// **[![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3700-3799/3787.Find%20Diameter%20Endpoints%20of%20a%20Tree/images/pic3.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3700-3799/3787.Find%20Diameter%20Endpoints%20of%20a%20Tree/images/pic3.png)​​​​​​​**

// **Input:** n = 2, edges = \[\[0,1\]\]

// **Output:** "11"

// **Explanation:**

// *   The diameter of this tree consists of 1 edge.
// *   The only diameter path is the path from node 0 to node 1
// *   The endpoints of this path are nodes 0 and 1, so they are special.

// **Constraints:**

// *   `2 <= n <= 105`
// *   `edges.length == n - 1`
// *   `edges[i] = [ai, bi]`
// *   `0 <= ai, bi < n`
// *   The input is generated such that `edges` represents a valid tree.

//  pub fn find_special_nodes(n: i32, edges: Vec<Vec<i32>>) -> String {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn find_special_nodes(n: i32, edges: Vec<Vec<i32>>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    // "a"
    // [[1,1],[1,1],[0,2],[1,3],[0,0]]
    #[test]
    pub fn test_find_special_nodes_1() {
        assert_eq!(
            "101".to_owned(),
            Solution::find_special_nodes(3, lc_matrix![[0, 1], [1, 2]])
        );
    }
    #[test]
    pub fn test_find_special_nodes_2() {
        assert_eq!(
            "1000111".to_owned(),
            Solution::find_special_nodes(
                7,
                lc_matrix![[0, 1], [1, 2], [2, 3], [3, 4], [3, 5], [1, 6]]
            )
        );
    }
    #[test]
    pub fn test_find_special_nodes_3() {
        assert_eq!(
            "11".to_owned(),
            Solution::find_special_nodes(2, lc_matrix![[0, 1]])
        );
    }
}
