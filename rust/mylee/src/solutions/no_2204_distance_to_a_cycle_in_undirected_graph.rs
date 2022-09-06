// # [2204. Distance to a Cycle in Undirected Graph](https://leetcode.com/problems/distance-to-a-cycle-in-undirected-graph)

// ## Description

// You are given a positive integer n representing the number of nodes in a connected undirected graph containing exactly one cycle.
// The nodes are numbered from 0 to n - 1 (inclusive).

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

// def distance_to_cycle(self, n, edges):

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn distance_to_cycle(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut degrees = vec![0; n];
        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            degrees[u] += 1;
            degrees[v] += 1;
            g[u].push(v);
            g[v].push(u);
        }
        let mut ans = vec![0; n];
        let mut q: std::collections::VecDeque<usize> = degrees
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| i)
            .collect();

        while let Some(u) = q.pop_front() {
            ans[u] = -1;
            for &v in &g[u] {
                degrees[v] -= 1;
                if degrees[v] == 1 {
                    q.push_back(v);
                }
            }
        }

        let mut q: std::collections::VecDeque<usize> = ans
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 0)
            .map(|(i, _)| i)
            .collect();
        while let Some(u) = q.pop_front() {
            for &v in &g[u] {
                if ans[v] == -1 {
                    ans[v] = ans[u] + 1;
                    q.push_back(v);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_distance_to_cycle_1() {
        assert_eq!(
            vec![1, 0, 0, 0, 0, 1, 2],
            Solution::distance_to_cycle(
                7,
                vec![
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![4, 1],
                    vec![0, 1],
                    vec![5, 2],
                    vec![6, 5]
                ]
            )
        );
    }
    #[test]
    pub fn test_distance_to_cycle_2() {
        assert_eq!(
            vec![0, 0, 0, 1, 2, 2, 1, 2, 2],
            Solution::distance_to_cycle(
                9,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![0, 2],
                    vec![2, 6],
                    vec![6, 7],
                    vec![6, 8],
                    vec![1, 3],
                    vec![3, 4],
                    vec![3, 5]
                ]
            )
        );
    }
}
