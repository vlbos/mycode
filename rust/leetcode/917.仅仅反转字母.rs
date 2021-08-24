/*
 * @lc app=leetcode.cn id=917 lang=rust
 *
 * [917] 仅仅反转字母
 */

// @lc code=start
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
            let mut sc=s.chars().collect::<Vec<char>>();
            let mut i=0;
            let mut j = sc.len()-1;
            while i<j{
                 while i<j && !sc[i].is_ascii_alphabetic(){
                    i+=1;
                }
                if i>=j{
                break;
                }
                while i<j && !sc[j].is_ascii_alphabetic(){
                    j-=1;
                }
                if i>=j{
                break;
                }
                let t = sc[i];
                sc[i]=sc[j];
                sc[j]=t;
                i+=1;
                j-=1;
            }
            sc.iter().collect::<String>()
    }
}
// @lc code=end

