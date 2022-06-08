/*
 * @lc app=leetcode id=1627 lang=rust
 *
 * [1627] Graph Connectivity With Threshold
 */

// @lc code=start
impl Solution {
    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut fa: Vec<usize> = (0..=n).collect();
        let mut sz = vec![1; n + 1];
        fn find(x: usize, fa: &mut Vec<usize>) -> usize {
            if fa[x] == x {
                return x;
            }
            fa[x] = find(fa[x], fa);
            fa[x]
        }
        let merge = |x: usize, y: usize, fa: &mut Vec<usize>, sz: &mut Vec<i32>| {
            let (fx, fy) = (find(x, fa), find(y, fa));
            if fx == fy {
                return;
            }
            let (fx, fy) = if sz[fx] > sz[fy] { (fy, fx) } else { (fx, fy) };
            fa[fx] = fy;
            sz[fy] += sz[fx];
        };
        for i in threshold as usize + 1..n {
            for j in (i * 2..=n).step_by(i) {
                merge(i, j, &mut fa, &mut sz);
            }
        }
        queries
            .iter()
            .map(|q| find(q[0] as usize, &mut fa) == find(q[1] as usize, &mut fa))
            .collect()
    }
}
// @lc code=end
