/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        fn back_track(n:i32,mut ans: &mut Vec<String>,mut curr:&mut Vec<char>,open:i32,close:i32){
            if curr.len()==(2*n as usize){
                ans.push(curr.iter().collect());
                return;
            }
            if open<n{
               curr.push('(');      
               back_track(n,&mut ans,&mut curr,open+1,close);
               curr.pop();
            }
            if close<open{
               curr.push(')');      
               back_track(n,&mut ans,&mut curr,open,close+1);
               curr.pop();
            }
        }
        let mut curr = Vec::<char>::new();
        back_track(n,&mut ans,&mut curr,0,0);
        ans
    }
}
// @lc code=end

