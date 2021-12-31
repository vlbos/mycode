/*
 * @lc app=leetcode id=785 lang=rust
 *
 * [785] Is Graph Bipartite?
 */

// @lc code=start
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        fn dfs(graph: &Vec<Vec<i32>>, color: &mut Vec<i32>, cc: i32, i: usize, flag: &mut bool) {
            color[i] = cc;
            for j in 0..graph[i].len() {
                let k = graph[i][j] as usize;
                if color[k] == 0 {
                    dfs(graph, color, if cc == 1 { 2 } else { 1 }, k, flag);
                    if !*flag {
                        return;
                    }
                } else if color[k] == cc {
                    *flag = false;
                    return;
                }
            }
        }
        let mut color = vec![0; graph.len()];
        let mut flag = true;
        for i in 0..color.len() {
            if color[i] == 0 {
                dfs(&graph, &mut color, 1, i, &mut flag);
                if !flag {
                    return false;
                }
            }
        }
        if color.iter().any(|x| *x == 0) {
            return false;
        }
        flag
    }
}
// @lc code=end
