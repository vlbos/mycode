/*
 * @lc app=leetcode id=1125 lang=rust
 *
 * [1125] Smallest Sufficient Team
 */

// @lc code=start
impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let n = req_skills.len();
        use std::collections::HashMap;
        let skill_id: HashMap<String, usize> = req_skills
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();
        let mut people_state = Vec::new();
        for skills in &people {
            let mut state = 0;
            for skill in skills {
                state |= 1<<*skill_id.get(skill).unwrap();
            }
            people_state.push(state);
        }
        let n1 = 1<<n;
        let mut team:HashMap<usize,Vec<i32>> = HashMap::new();
        let mut dp = vec![i32::MAX / 3; n1];
        dp[0] = 0;
        for (pi, &p) in people_state.iter().enumerate() {
            for state in (0..n1).rev() {
                let next_state = state | p;
                if dp[state] + 1 < dp[next_state] {
                    dp[next_state] = dp[state] + 1;
                    let mut nt = team.get(&state).unwrap_or(&Vec::new()).clone();
                    nt.push(pi as i32);
                    team.insert(next_state, nt);
                }
            }
        }
        team.remove(&(n1 - 1)).unwrap()
    }
}
// @lc code=end
