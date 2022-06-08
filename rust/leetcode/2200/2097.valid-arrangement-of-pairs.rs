/*
 * @lc app=leetcode id=2097 lang=rust
 *
 * [2097] Valid Arrangement of Pairs
 */

// @lc code=start
impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut edges = HashMap::new();
        let (mut indeg, mut outdeg) = (HashMap::new(), HashMap::new());
        for p in &pairs {
            let (u, v) = (p[0], p[1]);
            edges.entry(u).or_insert(Vec::new()).push(v);
            *indeg.entry(v).or_insert(0) += 1;
            *outdeg.entry(u).or_insert(0) += 1;
        }
        let start = *outdeg
            .iter()
            .find(|x| *x.1 == *indeg.get(&x.0).unwrap_or(&0) + 1)
            .unwrap_or((&pairs[0][0], &0))
            .0;
        let mut ans = Vec::new();
        fn dfs(u: i32, edges: &mut HashMap<i32, Vec<i32>>, ans: &mut Vec<Vec<i32>>) {
            while let Some(v) = edges.get_mut(&u).unwrap_or(&mut Vec::new()).pop() {
                dfs(v, edges, ans);
                ans.push(vec![u, v]);
            }
        }
        dfs(start,&mut edges,&mut ans);
        ans.reverse();
        ans
    }
}
// @lc code=end
