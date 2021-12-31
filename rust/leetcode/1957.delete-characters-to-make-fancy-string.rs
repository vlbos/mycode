/*
 * @lc app=leetcode id=1957 lang=rust
 *
 * [1957] Delete Characters to Make Fancy String
 */

// @lc code=start
impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut ans =String::new();
        let mut cnt = 0;
        let mut cc=' ';
        for c in s.chars(){
            if c!=cc{
                ans.push(c);
                cnt=1;
                cc=c;
            }else{
                cnt+=1;
                if cnt<3{
                  ans.push(c);
                }
            }
        }
        ans
    }
}
// @lc code=end

