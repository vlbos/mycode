/*
 * @lc app=leetcode id=1366 lang=rust
 *
 * [1366] Rank Teams by Votes
 */

// @lc code=start
impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let n = votes[0].len();
        let mut m = std::collections::HashMap::new();
        for v in &votes{
            for (i,c) in v.chars().enumerate(){
                m.entry(c).or_insert(vec![0;n])[i]+=1;
            }
        }
        let mut l:Vec<(char,Vec<i32>)> = m.iter().map(|x|(*x.0,x.1.clone())).collect();
        l.sort_by(|a,b| (b.1.clone(),-((b.0 as u8) as i8)).cmp(&(a.1.clone(),-((a.0 as u8) as i8))));
        let mut ans = String::new();
        for &(c,_) in &l{
            ans.push(c);
        }
        ans
    }
}
// @lc code=end

