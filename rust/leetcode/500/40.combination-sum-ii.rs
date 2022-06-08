/*
 * @lc app=leetcode id=40 lang=rust
 *
 * [40] Combination Sum II
 */

// @lc code=start
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
         fn dfs(
            candidates: &Vec<Vec<i32>>,
            target: i32,
            mut ans: &mut Vec<Vec<i32>>,
            mut combine: &mut Vec<i32>,
            idx: usize,
        ) {

            if target == 0 {
                    ans.push(combine.clone());
                return;
            }
            if idx == candidates.len() || target< candidates[idx][0]{
                return;
            }

            dfs(candidates, target, &mut ans, combine, idx + 1);
            let most = (target/candidates[idx][0]).min(candidates[idx][1]);
            for i in 1..=most {
                combine.push(candidates[idx][0]);
                dfs(candidates, target - i*candidates[idx][0], &mut ans, combine, idx+1);
            }
            for i in 1..=most {
                combine.pop();
            }
        }
        let mut combine = Vec::new();
        let mut ans = Vec::new();
        let mut candidates =  candidates;
        candidates.sort();
        let mut freq:Vec<Vec<i32>> = Vec::new();
        for c in &candidates{
                if freq.is_empty()||*(freq.last().unwrap().first().unwrap())!=*c{
                    freq.push(vec![*c,1]);
                }else{
                    *(freq.last_mut().unwrap().last_mut().unwrap())+=1;
                }
        }
        dfs(&freq, target, &mut ans, &mut combine, 0);
        ans
    }
}
// @lc code=end

