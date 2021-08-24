/*
 * @lc app=leetcode.cn id=389 lang=rust
 *
 * [389] 找不同
 */

// @lc code=start
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        // let mut a = vec![0;26];
        // for c in s.chars(){
        //     a[(c as u8-'a' as u8) as usize]+=1;
        // }
        // for c in t.chars(){
        //     let i = (c as u8-'a' as u8) as usize;
        //     a[i]-=1;
        //     if a[i]<0{
        //       return ('a' as u8 + i as u8) as char;
        //     }
        // }
        // 'a'
        let mut r =0;
        for c in s.chars(){
                r ^= c as u8;
        }
        for c in t.chars(){
                r ^= c as u8;
        }
        r as char
    }
}
// @lc code=end

