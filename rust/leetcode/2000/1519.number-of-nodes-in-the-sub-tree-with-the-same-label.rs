/*
 * @lc app=leetcode id=1519 lang=rust
 *
 * [1519] Number of Nodes in the Sub-Tree With the Same Label
 */

// @lc code=start
impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n as usize];
        fn dfs(
            g: &Vec<Vec<usize>>,
            labels: &String,
            index: usize,
            seen: &mut Vec<bool>,
            ans: &mut Vec<i32>,
        ) -> Vec<i32> {
            seen[index] = true;
            let mut ret = vec![0; 26];
            for &nn in &g[index] {
                if seen[nn] {
                    continue;
                }
                let r = dfs(g, labels, nn, seen, ans);
                for i in 0..26 {
                    ret[i] += r[i];
                }
            }
            let i = (labels.as_bytes()[index] - b'a') as usize;
            ret[i] += 1;
            ans[index] = ret[i];
            ret
        }
        for e in &edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut ans = vec![0; n as usize];
        let mut seen = vec![false; n as usize];
        dfs(&g, &labels, 0, &mut seen, &mut ans);
        ans
    }
}
// @lc code=end
