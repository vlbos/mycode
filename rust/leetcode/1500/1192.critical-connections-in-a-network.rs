/*
 * @lc app=leetcode id=1192 lang=rust
 *
 * [1192] Critical Connections in a Network
 */

// @lc code=start
impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
         let n = n as usize;
        let mut dfn = vec![0; n];
        let mut low = vec![0; n];
        let mut visited = vec![false; n];
        let mut edges = vec![Vec::new(); n];

        for connection in &connections {
            let (u, v) = (connection[0] as usize, connection[1] as usize);
            edges[u].push(v);
            edges[v].push(u);
        }
        fn tarjan(
            node: usize,
            parent: usize,
            dfn: &mut Vec<i32>,
            low: &mut Vec<i32>,
            visited: &mut Vec<bool>,
            ans: &mut Vec<Vec<i32>>,
            times: &mut i32,
            edges: &Vec<Vec<usize>>,
        ) {
            *times += 1;
            dfn[node] = *times;
            low[node] = *times;
            visited[node] = true;
            for &e in &edges[node] {
                if e == parent {
                    continue;
                }
                if visited[e] {
                    low[node] = low[node].min(dfn[e]);
                } else {
                    tarjan(e, node, dfn, low, visited, ans, times, edges);
                    low[node] = low[node].min(low[e]);
                    if low[e] > dfn[node] {
                        ans.push(vec![node as i32, e as i32]);
                    }
                }
            }
        }
        let mut ans =Vec::new();
        let mut times=0;
        tarjan(
            0,
            n,
            &mut dfn,
            &mut low,
            &mut visited,
            &mut ans,
            &mut times,
            &edges,
        );
        ans
    }
}
// @lc code=end
