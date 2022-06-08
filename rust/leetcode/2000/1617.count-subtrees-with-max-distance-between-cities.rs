/*
 * @lc app=leetcode id=1617 lang=rust
 *
 * [1617] Count Subtrees With Max Distance Between Cities
 */

// @lc code=start
impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut graph = HashMap::new();
        for e in &edges {
            let (u, v) = (e[0] - 1, e[1] - 1);
            graph.entry(u).or_insert(HashSet::new()).insert(v);
            graph.entry(v).or_insert(HashSet::new()).insert(u);
        }
        let n = n as usize;
        let bfs = |state: i32, cur: i32| {
            let mut st = HashSet::from([cur]);
            let mut q = std::collections::VecDeque::from([(cur, 0)]);
            let (mut f, mut m) = (-1, 0);
            while let Some((farthest_node, max_dist)) = q.pop_front() {
                f = farthest_node;
                m = max_dist;
                for &ne in graph.get(&farthest_node).unwrap_or(&HashSet::new()) {
                    if !st.contains(&ne) && ((state >> (ne as u32)) & 1) > 0 {
                        st.insert(ne);
                        q.push_back((ne, max_dist + 1));
                    }
                }
            }
            vec![f, m, st.len() as i32]
        };
        let get_max_dist = |state: i32| {
            let mut city_cnt = 0;
            let mut most_left_city = -1;
            for i in 0..n {
                if state & (1 << i) > 0 {
                    city_cnt += 1;
                    most_left_city = i as i32;
                }
            }
            let ans = bfs(state, most_left_city);
            if ans[2] < city_cnt {
                return 0;
            }
            bfs(state, ans[0])[1]
        };
        let mut ans = vec![0; n - 1];
        for i in 1..(1 << n) {
            let d = get_max_dist(i);
            if d > 0 {
                ans[d as usize - 1] += 1;
            }
        }
        ans
    }
}
// @lc code=end
