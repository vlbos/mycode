/*
 * @lc app=leetcode id=1782 lang=rust
 *
 * [1782] Count Pairs Of Nodes
 */

// @lc code=start
impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut deg = vec![0; n as usize + 1];
        let mut overlap = std::collections::HashMap::new();
        let mut distinct_edges = Vec::new();
        let encode = |a: i32, b: i32| a.max(b) * (n + 1) + a.min(b);
        for edge in &edges {
            let (p, q) = (edge[0], edge[1]);
            deg[p as usize] += 1;
            deg[q as usize] += 1;
            let idx = encode(p, q);
            if !overlap.contains_key(&idx) {
                distinct_edges.push(vec![p, q]);
            }
            *overlap.entry(idx).or_insert(0) += 1;
        }
        let mut sorted_deg = deg[1..].to_vec();
        sorted_deg.sort();
        let mut ans = vec![0; queries.len()];
        for (i,&query) in queries.iter().enumerate() {
            let (mut l, mut r) = (0, n- 1);
            let mut cnt = 0;
            while l < n {
                while l < r && sorted_deg[l  as usize ] + sorted_deg[r  as usize ] > query {
                    r -= 1;
                }
                cnt += n - l.max(r)- 1;
                l += 1;
            }
            for edge in &distinct_edges {
                let (p, q) = (edge[0] as usize, edge[1] as usize);
                let idx = encode(edge[0], edge[1]);
                if deg[p] + deg[q] > query
                    && deg[p] + deg[q] - *overlap.get(&idx).unwrap_or(&0) <= query
                {
                    cnt -= 1;
                }
            }
            ans[i] = cnt;
        }
        ans
    }
}
// @lc code=end
