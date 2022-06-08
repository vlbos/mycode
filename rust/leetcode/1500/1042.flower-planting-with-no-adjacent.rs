/*
 * @lc app=leetcode id=1042 lang=rust
 *
 * [1042] Flower Planting With No Adjacent
 */

// @lc code=start
impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
       let n = n as usize;
        let mut t = vec![Vec::new();n];
        for p in &paths{
            t[p[0].max(p[1]) as usize-1].push(p[0].min(p[1]) as usize-1);
        }
        let mut ans = vec![1;n];
        for i in 1..n{
             let mut s = [1,2,3,4].iter().cloned().collect::<std::collections::BTreeSet<i32>>();
             for j in 0..t[i].len(){
                   s.remove(&ans[t[i][j]]);
             }
             ans[i]=*s.iter().next().unwrap_or(&1);
        }
        ans
    }
}
// @lc code=end

