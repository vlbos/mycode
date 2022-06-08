/*
 * @lc app=leetcode id=1417 lang=rust
 *
 * [1417] Reformat The String
 */

// @lc code=start
impl Solution {
    pub fn reformat(s: String) -> String {
       let d = s.chars().filter(|x|x.is_ascii_digit()).collect::<Vec<char>>().len() as i32;
       let l = s.chars().filter(|x|x.is_ascii_lowercase()).collect::<Vec<char>>().len() as i32;
       if (d-l).abs()>1{
        return "".to_string();
       }
       let mut sv = s.chars().collect::<Vec<char>>();
       let mut i = 0;
       let mut j = 1;
       if d<l{
          i=1;
          j=0;
       }
       while i<sv.len() && j<sv.len(){
           while i<sv.len() && sv[i].is_ascii_digit(){
                i+=2;
            }
             while j<sv.len() && sv[j].is_ascii_lowercase(){
                j+=2;
            }
            if i<sv.len() && j<sv.len(){
                let a = sv[i];
                sv[i]=sv[j];
                sv[j]=a;
            }
       }
       sv.iter().collect::<String>()
    }
}
// @lc code=end

