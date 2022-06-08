/*
 * @lc app=leetcode id=473 lang=rust
 *
 * [473] Matchsticks to Square
 */

// @lc code=start
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        if matchsticks.len() < 4 {
            return false;
        }

        let s = matchsticks.iter().sum::<i32>();
        if s % 4 != 0 {
            return false;
        }
        let ss = s / 4;
        let f = matchsticks.iter().filter(|x| **x > ss).count();
        if f > 0 {
            return false;
        }
        let mut matchsticks = matchsticks;
        matchsticks.sort_by(|a, b| b.cmp(a));

        let mut sums = vec![0; 4];
        fn dfs(idx: usize, sl: i32, matchsticks: &Vec<i32>, sums: &mut Vec<i32>) -> bool {
            if idx == matchsticks.len() {
                for n in sums.iter().skip(1) {
                    if *sums.first().unwrap() != *n {
                        return false;
                    }
                }
                return true;
            }
            for i in 0..sums.len() {
                if sums[i] + matchsticks[idx] <= sl {
                    sums[i] += matchsticks[idx];
                    if dfs(idx + 1, sl, matchsticks, sums) {
                        return true;
                    }
                    sums[i] -= matchsticks[idx];
                }
            }
            false
        }

        dfs(0, ss, &matchsticks, &mut sums)
    }
}
// @lc code=end
