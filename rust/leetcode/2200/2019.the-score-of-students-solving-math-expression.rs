/*
 * @lc app=leetcode id=2019 lang=rust
 *
 * [2019] The Score of Students Solving Math Expression
 */

// @lc code=start
impl Solution {
    pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
         let mut count = std::collections::HashMap::new();
        for &ans in &answers {
            *count.entry(ans).or_insert(0) += 1;
        }
        let n = s.len();
        let mut st = Vec::new();
        let bs = s.as_bytes();
        st.push((bs[0] - b'0') as i32);
        for i in (1..n).step_by(2) {
            if bs[i] == b'+' {
                st.push((bs[i + 1] - b'0') as i32);
            } else {
                let last = st.len() - 1;
                st[last] *= (bs[i + 1] - b'0') as i32;
            }
        }
        let right = st.iter().sum::<i32>();
        let mut ans = *count.get(&right).unwrap_or(&0) * 5;
        use std::collections::HashSet;
        let mut dp = vec![vec![HashSet::new(); n + 2]; n + 2];
        for i in (0..n).step_by(2) {
            dp[i][i].insert((bs[i] - b'0') as i32);
        }
        for step in (2..n).step_by(2) {
            for i in (0..n - step).step_by(2) {
                for t in (0..step).step_by(2) {
                    for x in dp[i][i + t].clone() {
                        for y in dp[i + t + 2][i + step].clone() {
                            if bs[i + t + 1] == b'+' {
                                if x + y <= 1000 {
                                    dp[i][i + step].insert(x + y);
                                }
                            } else {
                                if x * y <= 1000 {
                                    dp[i][i + step].insert(x * y);
                                }
                            }
                        }
                    }
                }
            }
        }
        ans + dp[0][n - 1]
            .iter()
            .filter(|&x| *x != right)
            .map(|x|  *count.get(x).unwrap_or(&0) * 2)
            .sum::<i32>()
    }
}
// @lc code=end
