/*
 * @lc app=leetcode id=1190 lang=rust
 *
 * [1190] Reverse Substrings Between Each Pair of Parentheses
 */

// @lc code=start
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = Vec::new();
        let mut pair = vec![0;s.len()];
        for (i,c) in s.chars().enumerate(){
               if c=='('{
                    stack.push(i);
               }else if c==')'{
                    if let Some(j)=stack.pop(){
                        pair[i]=j;
                        pair[j]=i;
                    }
               }
        }
        let mut ans = String::new();
        let mut i =0;
        let mut step=1i32;
        while i<s.len(){
                let c = s.chars().nth(i).unwrap();
                if c=='('||c==')'{
                    i = pair[i];
                    step = -step;
                }else{
                    ans.push(c);
                }
                
                i= (i as i32+step) as usize;
         }
        ans
    }   
}
// @lc code=end

