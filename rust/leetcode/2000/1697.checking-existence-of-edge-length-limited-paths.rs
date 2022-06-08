/*
 * @lc app=leetcode id=1697 lang=rust
 *
 * [1697] Checking Existence of Edge Length Limited Paths
 */

// @lc code=start
impl Solution {
    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
         let mut fa: Vec<i32> = (0..n).collect();
        let mut sz = vec![1; n as usize];
        let mut comp_cnt = n;
        fn find_set(x: i32, fa: &mut Vec<i32>) -> i32 {
            let xi = x as usize;
            if fa[xi] == x {
                return x;
            }
            fa[xi] = find_set(fa[xi], fa);
            fa[xi]
        }
        let unite = |x: i32, y: i32, comp_cnt: &mut i32, fa: &mut Vec<i32>, sz: &mut Vec<i32>| {
            let (x, y) = (find_set(x, fa), find_set(y, fa));
            if x == y {
                return false;
            }
            let (x, y) = if sz[x as usize] < sz[y as usize] {
                (y, x)
            } else {
                (x, y)
            };
            fa[y as usize] = x;
            sz[x as usize] += sz[y as usize];
            *comp_cnt -= 1;
            true
        };
        let connected = |x: i32, y: i32, fa: &mut Vec<i32>| find_set(x, fa) == find_set(y, fa);
        let mut qid: Vec<usize> = (0..queries.len()).collect();
        qid.sort_by_key(|&x| queries[x][2]);
        let mut edge_list = edge_list;
        edge_list.sort_by_key(|x| x[2]);
        let mut i = 0;
        let mut ans = vec![false; queries.len()];
        for &id in &qid {
            while i < edge_list.len() && edge_list[i][2] < queries[id][2] {
                unite(
                    edge_list[i][0],
                    edge_list[i][1],
                    &mut comp_cnt,
                    &mut fa,
                    &mut sz,
                );
                i += 1;
            }
            ans[id] = connected(queries[id][0], queries[id][1],&mut fa);
        }
        ans
    }
}
// @lc code=end
