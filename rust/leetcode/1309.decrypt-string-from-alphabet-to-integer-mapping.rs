/*
 * @lc app=leetcode id=1309 lang=rust
 *
 * [1309] Decrypt String from Alphabet to Integer Mapping
 */

// @lc code=start
impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut ans = String::new();
        let sv = s.chars().collect::<Vec<char>>(); 
        let mut i = 0;
        while i<sv.len(){
            if i+2<sv.len() && sv[i+2]=='#'{
               let  n =  &sv[i..i+2].iter().collect::<String>().parse::<usize>().unwrap();
               let c = ('j' as u8+(n-10) as u8) as char;
               ans.push(c);
               i+=3;
            }else{
               let  n =  sv[i].to_string().parse::<usize>().unwrap();
               let c = ('a' as u8+(n-1) as u8) as char;
               ans.push(c);
               i+=1;
            }
        }
        ans
    }
}
// @lc code=end

