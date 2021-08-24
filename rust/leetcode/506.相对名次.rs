/*
 * @lc app=leetcode.cn id=506 lang=rust
 *
 * [506] 相对名次
 */

// @lc code=start
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut s = vec!["Gold Medal".to_string(), "Silver Medal".to_string(), "Bronze Medal".to_string()];
        let mut r = Vec::<String>::new();
        let mut scores = score.clone();
        scores.sort();
        let mut m = std::collections::HashMap::<i32,String>::new();
        for (i,n) in scores.iter().rev().enumerate(){
            if i<3{
                m.insert(*n,s[i].clone());
            }else{
                 m.insert(*n,(i+1).to_string());
            }
        }
   
        for v in &score{
                r.push((*(m.get(v).unwrap())).clone());
        }
        r
    }
}
// @lc code=end

