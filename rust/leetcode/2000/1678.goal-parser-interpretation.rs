/*
 * @lc app=leetcode id=1678 lang=rust
 *
 * [1678] Goal Parser Interpretation
 */

// @lc code=start
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut c = command;
        c = c.replace("(al)", "al");
        c = c.replace("()", "o");
        c
        // let mut ans =Vec::new();
        // let pal = &("(al".chars().collect::<Vec<char>>())[..];
        // let al = "al".chars().collect::<Vec<char>>();
        // for c in command.chars(){
        //    if c==')'{
        //         if !ans.is_empty() && ans[ans.len()-1]=='('{
        //             ans.pop();
        //             ans.push('o');
        //         }else if ans.len()>2 && &ans[ans.len()-3..]==pal{
        //             ans.truncate(ans.len()-3);
        //             ans.extend(al.clone());
        //         }
        //     }else{
        //         ans.push(c);
        //     }
        // }
        // ans.iter().collect()
    }
}
// @lc code=end
