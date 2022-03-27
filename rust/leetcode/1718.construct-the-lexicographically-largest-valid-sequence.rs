/*
 * @lc app=leetcode id=1718 lang=rust
 *
 * [1718] Construct the Lexicographically Largest Valid Sequence
 */

// @lc code=start
impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let mut ans = vec![0; n  as usize* 2 - 1];
        let mut j = 0;
        fn dfs(i: usize, cnt: usize, ans: &mut Vec<i32>) -> bool {
            if cnt >= ans.len() {
                return true;
            }
            if ans[i] > 0 {
                return dfs(i + 1, cnt, ans);
            }
            for j in (1..=(ans.len()+1)/2).rev() {
                                let v = j as i32;
                let d = if j == 1 { 0 } else { j };
                if i + d >= ans.len() || ans.contains(&v) || ans[i + d] > 0 {
                    continue;
                }
                ans[i] = v;
                ans[i + d] = v;
                if dfs(i + 1, cnt + 2, ans) {
                    return true;
                }
                ans[i] = 0;
                ans[i + d] = 0;
            }
            false
        }
        dfs(0, 0, &mut ans);
        ans
    }
}
// @lc code=end
