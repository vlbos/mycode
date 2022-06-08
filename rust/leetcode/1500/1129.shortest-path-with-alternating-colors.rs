/*
 * @lc app=leetcode id=1129 lang=rust
 *
 * [1129] Shortest Path with Alternating Colors
 */

// @lc code=start
impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut neighbours = vec![vec![Vec::new(); n]; 2];
        for r in &red_edges {
            neighbours[0][r[0] as usize].push(r[1] as usize);
        }
        for b in &blue_edges {
            neighbours[1][b[0] as usize].push(b[1] as usize);
        }
        let mut visited = vec![vec![false; n]; 2];
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0, 0));
        q.push_back((0, 1,0));
        let mut ans = vec![i32::MAX; n];
        while let Some((i, c, s)) = q.pop_front() {
            ans[i] = ans[i].min(s);
            for &j in &neighbours[c][i] {
                if visited[c][j] {
                    continue;
                }
                visited[c][j] = true;
                q.push_back((j, 1-c, s + 1));
            }
        }
        ans.iter()
            .map(|&x| if x == i32::MAX { -1 } else { x })
            .collect::<Vec<i32>>()
    }
}
// @lc code=end
