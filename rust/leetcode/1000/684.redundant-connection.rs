/*
 * @lc app=leetcode id=684 lang=rust
 *
 * [684] Redundant Connection
 */

// @lc code=start
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut p = vec![0; n + 1];
        for i in 0..p.len() {
            p[i] = i;
        }
        for e in &edges {
            let n1 = e[0] as usize;
            let n2 = e[1] as usize;
            if find(&mut p, n1) != find(&mut p, n2) {
                union(&mut p, n1, n2);
            } else {
                return e.clone();
            }
        }

        fn find(p: &mut Vec<usize>, index: usize) -> usize {
            if p[index] != index {
                p[index] = find(p, p[index]);
            }
            p[index]
        }
        fn union(p: &mut Vec<usize>, index1: usize, index2: usize) {
            let p1 = find(p, index1);
            p[p1] = find(p, index2);
        }
        Vec::new()
    }
}
// @lc code=end
