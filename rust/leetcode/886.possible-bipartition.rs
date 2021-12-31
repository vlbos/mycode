/*
 * @lc app=leetcode id=886 lang=rust
 *
 * [886] Possible Bipartition
 */

// @lc code=start
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n+1];
        for p in &dislikes {
            graph[p[0] as usize].push(p[1] as usize);
            graph[p[1] as usize].push(p[0] as usize);
        }
        let mut color = std::collections::HashMap::new();
        fn dfs(
            graph: &Vec<Vec<usize>>,
            color: &mut std::collections::HashMap<usize, bool>,
            i: usize,
            c: bool,
        )->bool {
            if color.contains_key(&i) {
                return *color.get(&i).unwrap_or(&false) == c;
            }
            color.insert(i, c);
            for node in &graph[i] {
                if !dfs(graph, color, *node, !c) {
                    return false;
                }
            }
            true
        }
        for node in 0..graph.len() {
            if !color.contains_key(&node) && !dfs(&graph, &mut color, node, true) {
                return false;
            }
        }
        true
    }
}
// @lc code=end
