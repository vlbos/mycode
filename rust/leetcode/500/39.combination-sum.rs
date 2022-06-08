/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 */

// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(
            candidates: &Vec<i32>,
            target: i32,
            mut ans: &mut Vec<Vec<i32>>,
            mut combine: &mut Vec<i32>,
            idx: i32,
        ) {
            if idx == candidates.len() as i32 {
                return;
            }
            if target == 0 {
                ans.push(combine.clone());
                return;
            }
            dfs(candidates, target, &mut ans, combine, idx + 1);
            if target - candidates[idx as usize] >= 0 {
                combine.push(candidates[idx as usize]);
                dfs(candidates, target - candidates[idx  as usize], &mut ans, combine, idx);
                combine.pop();
            }
        }
        let mut combine = Vec::new();
        let mut ans = Vec::new();
        dfs(&candidates, target, &mut ans, &mut combine, 0);
        ans
    }
}
// @lc code=end
