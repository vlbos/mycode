/*
 * @lc app=leetcode id=851 lang=rust
 *
 * [851] Loud and Rich
 */

// @lc code=start
impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut g = vec![Vec::new(); n];
        for r in &richer {
            g[r[1] as usize].push(r[0] as usize);
        }
        let mut ans = vec![-1; n];
        fn dfs(ans: &mut Vec<i32>, g: &Vec<Vec<usize>>, quiet: &Vec<i32>, i: usize) {
            if ans[i] != -1 {
                return;
            }
            ans[i] = i as i32;
            for v in &g[i] {
                dfs(ans, g, quiet, *v);
                if quiet[ans[*v] as usize] < quiet[ans[i] as usize] {
                    ans[i] = ans[*v] ;
                }
            }
        }
        for i in 0..n {
            dfs(&mut ans, &g, &quiet, i);
        }
        ans
    }
}
// @lc code=end
