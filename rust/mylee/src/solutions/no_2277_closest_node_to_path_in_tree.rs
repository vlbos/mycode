// # [2277. Closest Node to Path in Tree](https://leetcode.com/problems/closest-node-to-path-in-tree)

// ## Description

// You are given a positive integer n representing the number of nodes in a tree, numbered from 0 to n - 1 (inclusive).
//  You are also given a 2D integer array edges of length n - 1, where edges[i] = [node1i, node2i] denotes that there is a bidirectional edge connecting node1i and node2i in the tree.

// You are given a 0-indexed integer array query of length m where query[i] = [starti, endi, nodei] means that for the ith query,
// you are tasked with finding the node on the path from starti to endi that is closest to nodei.

// Return an integer array answer of length m, where answer[i] is the answer to the ith query.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2277.Closest%20Node%20to%20Path%20in%20Tree/images/image-20220514132158-1.png" style="width: 300px; height: 211px;" />
//
// Input: n = 7, edges = [[0,1],[0,2],[0,3],[1,4],[2,5],[2,6]], query = [[5,3,4],[5,3,6]]
// Output: [0,2]
// Explanation:
// The path from node 5 to node 3 consists of the nodes 5, 2, 0, and 3.
// The distance between node 4 and node 0 is 2.
// Node 0 is the node on the path closest to node 4, so the answer to the first query is 0.
// The distance between node 6 and node 2 is 1.
// Node 2 is the node on the path closest to node 6, so the answer to the second query is 2.
//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2277.Closest%20Node%20to%20Path%20in%20Tree/images/image-20220514132318-2.png" style="width: 300px; height: 89px;" />
//
// Input: n = 3, edges = [[0,1],[1,2]], query = [[0,1,2]]
// Output: [1]
// Explanation:
// The path from node 0 to node 1 consists of the nodes 0, 1.
// The distance between node 2 and node 1 is 1.
// Node 1 is the node on the path closest to node 2, so the answer to the first query is 1.
//

// Example 3:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2277.Closest%20Node%20to%20Path%20in%20Tree/images/image-20220514132333-3.png" style="width: 300px; height: 89px;" />
//
// Input: n = 3, edges = [[0,1],[1,2]], query = [[0,0,0]]
// Output: [0]
// Explanation:
// The path from node 0 to node 0 consists of the node 0.
// Since 0 is the only node on the path, the answer to the first query is 0.

// Constraints:

//
// 	1 <= n <= 1000
// 	edges.length == n - 1
// 	edges[i].length == 2
// 	0 <= node1i, node2i <= n - 1
// 	node1i != node2i
// 	1 <= query.length <= 1000
// 	query[i].length == 3
// 	0 <= starti, endi, nodei <= n - 1
// 	The graph is a tree.
//

//  vector<int> closest_node(int n, vector<vector<int>>& edges,
//                           vector<vector<int>>& query) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn closest_node(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let mut n = n as usize;
        let mut g = vec![Vec::new(); n];
        let mut dist = vec![vec![-1; n]; n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }
        fn fill_dist(
            start: usize,
            u: usize,
            d: i32,
            g: &Vec<Vec<usize>>,
            dist: &mut Vec<Vec<i32>>,
        ) {
            dist[start][u] = d;
            for &v in &g[u] {
                if dist[start][v] == -1 {
                    fill_dist(start, v, d + 1, g, dist);
                }
            }
        }
        fn find_closest(
            u: usize,
            end: usize,
            node: usize,
            ans: usize,
            g: &Vec<Vec<usize>>,
            dist: &Vec<Vec<i32>>,
        ) -> i32 {
            for &v in &g[u] {
                if dist[v][end] < dist[u][end] {
                    return find_closest(
                        v,
                        end,
                        node,
                        if dist[ans][node] < dist[v][node] {
                            ans
                        } else {
                            v
                        },
                        g,
                        dist,
                    );
                }
            }
            ans as i32
        }
        for i in 0..n {
            fill_dist(i, i, 0, &g, &mut dist);
        }
        let mut ans = Vec::new();
        for q in &query {
            ans.push(find_closest(
                q[0] as usize,
                q[1] as usize,
                q[2] as usize,
                q[0] as usize,
                &g,
                &dist,
            ));
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_closest_node_1() {
        assert_eq!(
            vec![0, 2],
            Solution::closest_node(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![1, 4],
                    vec![2, 5],
                    vec![2, 6]
                ],
                vec![vec![5, 3, 4], vec![5, 3, 6]]
            )
        );
    }
    #[test]
    pub fn test_closest_node_2() {
        assert_eq!(
            vec![1],
            Solution::closest_node(3, vec![vec![0, 1], vec![1, 2]], vec![vec![0, 1, 2]])
        );
    }
    #[test]
    pub fn test_closest_node_3() {
        assert_eq!(
            vec![0],
            Solution::closest_node(3, vec![vec![0, 1], vec![1, 2]], vec![vec![0, 0, 0]])
        );
    }
}
