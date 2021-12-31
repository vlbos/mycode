/*
 * @lc app=leetcode id=856 lang=rust
 *
 * [856] Score of Parentheses
 */

// @lc code=start
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut ans = 0;
        let mut bal = 0;
        for (i,v) in s.chars().enumerate(){
            if v=='('{
                bal+=1;
            }else{
                bal-=1;
                if s.chars().nth(i-1).unwrap()=='('{
                    ans+=1<<bal;
                }
            }
        }
        ans
    }
}
// @lc code=end

