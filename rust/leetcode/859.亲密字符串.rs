/*
 * @lc app=leetcode.cn id=859 lang=rust
 *
 * [859] 亲密字符串
 */

// @lc code=start
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len()!=goal.len(){
        return false;
        }
        if s==goal{
         for (i,c) in s.chars().enumerate(){
              if s[i+1..].contains(&c.to_string()){
                    return true;
                }
         }
        }
        let mut ii =Vec::<usize>::new();
        for (i,c) in s.chars().enumerate(){
            if c!= goal.chars().nth(i).unwrap(){
                    ii.push(i);
                    if ii.len()>2{
                    return false;
                    }
            }
        }
        if ii.len()==2 && s.chars().nth(ii[0])==goal.chars().nth(ii[1]) && s.chars().nth(ii[1])==goal.chars().nth(ii[0]) {
            return true;
        }
        false
    }
}
// @lc code=end

