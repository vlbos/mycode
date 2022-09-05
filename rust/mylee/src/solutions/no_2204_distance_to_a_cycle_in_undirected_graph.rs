// # [2204. Distance to a Cycle in Undirected Graph](https://leetcode.com/problems/distance-to-a-cycle-in-undirected-graph)

// ## Description

// You are given a positive integer n representing the number of nodes in a connected undirected graph containing exactly one cycle. The nodes are numbered from 0 to n - 1 (inclusive).

// You are also given a 2D integer array edges, where edges[i] = [node1i, node2i] denotes that there is a bidirectional edge connecting node1i and node2i in the graph.

// The distance between two nodes a and b is defined to be the minimum number of edges that are needed to go from a to b.

// Return an integer array answer of size n, where answer[i] is the minimum distance between the ith node and any node in the cycle.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2204.Distance%20to%20a%20Cycle%20in%20Undirected%20Graph/images/image-20220315154238-1.png" style="width: 350px; height: 237px;" />
//
// Input: n = 7, edges = [[1,2],[2,3],[3,4],[4,1],[0,1],[5,2],[6,5]]
// Output: [1,0,0,0,0,1,2]
// Explanation:
// The nodes 1, 2, 3, and 4 form the cycle.
// The distance from 0 to 1 is 1.
// The distance from 1 to 1 is 0.
// The distance from 2 to 2 is 0.
// The distance from 3 to 3 is 0.
// The distance from 4 to 4 is 0.
// The distance from 5 to 2 is 1.
// The distance from 6 to 2 is 2.
//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2204.Distance%20to%20a%20Cycle%20in%20Undirected%20Graph/images/image-20220315154634-1.png" style="width: 400px; height: 297px;" />
//
// Input: n = 9, edges = [[0,1],[1,2],[0,2],[2,6],[6,7],[6,8],[1,3],[3,4],[3,5]]
// Output: [0,0,0,1,2,2,1,2,2]
// Explanation:
// The nodes 0, 1, and 2 form the cycle.
// The distance from 0 to 0 is 0.
// The distance from 1 to 1 is 0.
// The distance from 2 to 2 is 0.
// The distance from 3 to 1 is 1.
// The distance from 4 to 1 is 2.
// The distance from 5 to 1 is 2.
// The distance from 6 to 2 is 1.
// The distance from 7 to 2 is 2.
// The distance from 8 to 2 is 2.
//

// Constraints:

//
// 	3 <= n <= 105
// 	edges.length == n
// 	edges[i].length == 2
// 	0 <= node1i, node2i <= n - 1
// 	node1i != node2i
// 	The graph is connected.
// 	The graph has exactly one cycle.
// 	There is at most one edge between any pair of vertices.
//

// def distanceToCycle(self, n, edges):

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
