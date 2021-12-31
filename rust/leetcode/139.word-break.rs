/*
 * @lc app=leetcode id=139 lang=rust
 *
 * [139] Word Break
 */

// @lc code=start
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
            let d = word_dict.iter().collect::<std::collections::HashSet<_>>();
            let sv= s.chars().collect::<Vec<char>>();
            let mut dp = vec![false;sv.len()+1];
            dp[0]=true;
            for i in 1..=sv.len(){
                    for j in 0..i{
                        let ss = sv[j..i].iter().collect::<String>();
                        if dp[j]&& d.contains(&ss){
                            dp[i]=true;
                            break;
                        }
                    }
            }
            dp[sv.len()]
    }
}
// @lc code=end

