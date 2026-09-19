// [3383\. Minimum Runes to Add to Cast Spell 🔒](https://leetcode.com/problems/minimum-runes-to-add-to-cast-spell)
// ================================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// Description
// -----------

// Alice has just graduated from wizard school, and wishes to cast a magic spell to celebrate. The magic spell contains certain **focus points** where magic needs to be concentrated, and some of these focus points contain **magic crystals** which serve as the spell's energy source. Focus points can be linked through **directed runes**, which channel magic flow from one focus point to another.

// You are given a integer `n` denoting the _number_ of focus points and an array of integers `crystals` where `crystals[i]` indicates a focus point which holds a magic crystal. You are also given two integer arrays `flowFrom` and `flowTo`, which represent the existing **directed runes**. The `ith` rune allows magic to freely flow from focus point `flowFrom[i]` to focus point `flowTo[i]`.

// You need to find the number of directed runes Alice must add to her spell, such that _each_ focus point either:

// *   **Contains** a magic crystal.
// *   **Receives** magic flow _from_ another focus point.

// Return the **minimum** number of directed runes that she should add.

// **Example 1:**

// **Input:** n = 6, crystals = \[0\], flowFrom = \[0,1,2,3\], flowTo = \[1,2,3,0\]

// **Output:** 2

// **Explanation:**

// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3383.Minimum%20Runes%20to%20Add%20to%20Cast%20Spell/images/runesexample0.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3383.Minimum%20Runes%20to%20Add%20to%20Cast%20Spell/images/runesexample0.png)

// Add two directed runes:

// *   From focus point 0 to focus point 4.
// *   From focus point 0 to focus point 5.

// **Example 2:**

// **Input:** n = 7, crystals = \[3,5\], flowFrom = \[0,1,2,3,5\], flowTo = \[1,2,0,4,6\]

// **Output:** 1

// **Explanation:**

// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3383.Minimum%20Runes%20to%20Add%20to%20Cast%20Spell/images/runesexample1.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3383.Minimum%20Runes%20to%20Add%20to%20Cast%20Spell/images/runesexample1.png)

// Add a directed rune from focus point 4 to focus point 2.

// **Constraints:**

// *   `2 <= n <= 105`
// *   `1 <= crystals.length <= n`
// *   `0 <= crystals[i] <= n - 1`
// *   `1 <= flowFrom.length == flowTo.length <= min(2 * 105, (n * (n - 1)) / 2)`
// *   `0 <= flowFrom[i], flowTo[i] <= n - 1`
// *   `flowFrom[i] != flowTo[i]`
// *   All pre-existing directed runes are **distinct**.

// int min_runes_to_add(int n, vector<int>& crystals, vector<int>& flow_from, vector<int>& flow_to) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_runes_to_add(
        n: i32,
        crystals: Vec<i32>,
        flow_from: Vec<i32>,
        flow_to: Vec<i32>,
    ) -> i32 {
        use std::collections::VecDeque;
        let n = n as usize;
        let (mut con, mut rcon) = (vec![vec![]; n], vec![vec![]; n]);
        for (&a, &b) in flow_from.iter().zip(&flow_to) {
            con[a as usize].push(b as usize);
            rcon[b as usize].push(a as usize);
        }

        fn dfs(x: usize, con: &Vec<Vec<usize>>, mark: &mut Vec<bool>, all: &mut Vec<usize>) {
            if mark[x] {
                return;
            }
            mark[x] = true;

            for &y in &con[x] {
                dfs(y, con, mark, all);
            }
            all.push(x);
        }
        fn dfs2(x: usize, m: i32, con: &Vec<Vec<usize>>, id: &mut Vec<i32>) {
            if id[x] >= 0 {
                return;
            }
            id[x] = m;

            for &y in &con[x] {
                dfs2(y, m, con, id);
            }
        }

        let (mut mark, mut all) = (vec![false; n], vec![]);
        for i in 0..n {
            dfs(i, &con, &mut mark, &mut all);
        }

        let mut id = vec![-1; n];
        let mut m = 0;
        for i in (0..n).rev() {
            let x = all[i];
            if id[x] < 0 {
                dfs2(x, m, &rcon, &mut id);
                m += 1;
            }
        }

        let mut to = vec![false; m as usize];
        for i in 0..flow_from.len() {
            if id[flow_from[i] as usize] != id[flow_to[i] as usize] {
                to[id[flow_to[i] as usize] as usize] = true;
            }
        }

        let mut special = vec![false; m as usize];
        for &x in &crystals {
            special[id[x as usize] as usize] = true;
        }

        (0..m as usize).filter(|&i| !to[i] && !special[i]).count() as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_min_runes_to_add_1() {
        assert_eq!(
            2,
            Solution::min_runes_to_add(6, vec![0], vec![0, 1, 2, 3], vec![1, 2, 3, 0])
        );
    }
    #[test]
    pub fn test_min_runes_to_add_2() {
        assert_eq!(
            1,
            Solution::min_runes_to_add(7, vec![3, 5], vec![0, 1, 2, 3, 5], vec![1, 2, 0, 4, 6])
        );
    }
}

// Intuition
// Find strongly connected components, turn them into single nodes, and then find all nodes without a crystal and with an in-degree of zero

// Approach
// Tarjan Algo to find SCC's, then mark all nodes with crystals or incoming connections, and count the unmarked nodes

// Code
// class Solution {
//     private int time = 1, current = 0, top = 0;
//     public int minRunesToAdd(int n, int[] crystals, int[] flowFrom, int[] flowTo) {
//         int m = flowFrom.length;
//         int[] id = new int[n]; //stores the id of the strongly connected component for each node

//         //create adj list
//         ArrayList<Integer>[] adj = new ArrayList[n];
//         for(int i = 0; i < n; i++) adj[i] = new ArrayList<>();
//         for(int i = 0; i < m; i++) adj[flowFrom[i]].add(flowTo[i]);

//         //identify strongly connected components
//         int[] stack = new int[n], dist = new int[n], min = new int[n];
//         boolean[] seen = new boolean[n];
//         for(int i = 0; i < n; i++) {
//             if(dist[i] == 0) tarjan(i, seen, dist, min, stack, id, adj);
//         }
//         //we now can treat strongly connected components as singular nodes

//         //mark all nodes with a non-zero in-degree
//         for(int i = 0; i < m; i++) {
//             int val = id[flowTo[i]];
//             if(id[flowFrom[i]] != val) seen[val] = true;
//         }
//         //mark all crystal nodes
//         for(int x : crystals) seen[id[x]] = true;

//         //count the nodes with a zero in-degree and arent crystal nodes, as these must be connected
//         int count = 0;
//         for(int i = 0; i < current; i++) {
//             if(!seen[i]) count++;
//         }
//         return count;
//     }
//     private void tarjan(int index, boolean[] seen, int[] dist, int[] min, int[] stack, int[] id, ArrayList<Integer>[] adj) {
//         dist[index] = min[index] = time++;
//         seen[index] = true;
//         stack[top++] = index;
//         for(int next : adj[index]) {
//             if(dist[next] == 0) {
//                 tarjan(next, seen, dist, min, stack, id, adj);
//                 min[index] = Math.min(min[index], min[next]);
//             }else if(seen[next]) min[index] = Math.min(min[index], dist[next]);
//         }
//         if(min[index] == dist[index]) {
//             int root = -1;
//             while(root != index) {
//                 root = stack[--top];
//                 id[root] = current; //give all nodes in the current component the same id
//                 seen[root] = false;
//             }
//             current++;
//         }
//     }
// }

// Intuition
// Run kosa raju's algo to find all strongly connected components
// Build a simplified graph (DAG) with all the strongly connected components
// Identify what components the crystal nodes belong to
// For each node in the new DAG, if this component does not contain a crystal node, and its indegree is zero, we need to add a new rune.
// Complexity
// Time complexity:
// O(E+V)

// Space complexity:
// O(E+V)

// Code
// from collections import defaultdict
// from typing import List

// class Solution:
//     def minRunesToAdd(self, n: int, crystals: List[int], flowFrom: List[int], flowTo: List[int]) -> int:
//         adjMap = {i: [] for i in range(n)}
//         transposeMap = {i: [] for i in range(n)}
//         stack = []

//         # Build the original graph and its transpose
//         for src, dsn in zip(flowFrom, flowTo):
//             adjMap[src].append(dsn)
//             transposeMap[dsn].append(src)

//         visited = set()
//         count = 0
//         sccs = []

//         # First DFS pass to fill the stack based on finishing times
//         def dfs(node):
//             visited.add(node)
//             for nei in adjMap[node]:
//                 if nei not in visited:
//                     dfs(nei)
//             stack.append(node)

//         # Second DFS pass on the transposed graph
//         def transposedfs(node, scc):
//             visited.add(node)
//             scc.append(node)
//             for nei in transposeMap[node]:  # Corrected to use transposeMap
//                 if nei not in visited:
//                     transposedfs(nei, scc)

//         # Step 1: Perform DFS on the original graph to fill the stack
//         for node in range(n):
//             if node not in visited:
//                 dfs(node)

//         # Step 2: Perform DFS on the transposed graph in reverse finishing time order
//         visited = set()
//         while stack:
//             node = stack.pop()
//             if node not in visited:
//                 scc = []
//                 transposedfs(node, scc)
//                 sccs.append(scc)

//         # Step 3: Map each node to its SCC index
//         scc_map = {}
//         for i, scc in enumerate(sccs):
//             for node in scc:
//                 scc_map[node] = i

//         # Step 4: Build the DAG of SCCs
//         dag = {node:[]  for node in range(len(sccs))}
//         for node in adjMap:
//             for nei in adjMap[node]:
//                 if scc_map[node] != scc_map[nei]:  # Only add edges between different SCCs
//                     dag[scc_map[node]].append(scc_map[nei])

//         # Step 5: Calculate indegree for each SCC in the DAG
//         indegree = [0] * n
//         for node in dag:
//             for nei in dag[node]:
//                 indegree[nei] += 1

//         # Step 6: Track which SCCs contain crystals
//         crystalComponents = set()
//         for crystal in crystals:
//             crystalComponents.add(scc_map[crystal])

//         # Step 7: Count the number of SCCs that need new runes
//         for node in dag:
//             if indegree[node] == 0 and node not in crystalComponents:
//                 count += 1

//         return count
