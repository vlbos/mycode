/*
 * @lc app=leetcode id=882 lang=rust
 *
 * [882] Reachable Nodes In Subdivided Graph
 */

// @lc code=start
impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
       let n = n as usize;
        let mut graph = vec![Vec::new(); n];
        for edge in &edges {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        let mut dist = vec![max_moves + 1; n];
        dist[0] = 0;
        let mut used = std::collections::HashMap::new();
        let mut ans = 0;
        use std::cmp::Reverse;
        let mut bh = std::collections::BinaryHeap::from([Reverse((0, 0))]);
        while let Some(Reverse((d, node))) = bh.pop() {
            if d > dist[node] {
                continue;
            }
            ans += 1;
            for &(nei, weight) in &graph[node] {
                let v = weight.min(max_moves - d);
                used.insert((node, nei), v);
                let d2 = d + weight + 1;
                if d2 < dist[nei] {
                    bh.push(Reverse((d2, nei)));
                    dist[nei] = d2;
                }
            }
        }
        for edge in &edges {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            ans += w.min(*used.get(&(u, v)).unwrap_or(&0) + *used.get(&(v, u)).unwrap_or(&0));
        }
        ans
    }
}
// @lc code=end
