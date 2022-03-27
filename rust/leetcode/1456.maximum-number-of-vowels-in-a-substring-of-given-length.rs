/*
 * @lc app=leetcode id=1456 lang=rust
 *
 * [1456] Maximum Number of Vowels in a Substring of Given Length
 */

// @lc code=start
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let bs = s.as_bytes();
        let k = k as usize;
        let mut vc=0;
        for i in 0..k{
            if "aeiou".bytes().find(|x|*x==bs[i]).is_some(){
            vc+=1;
            }
        }
        let mut ans = vc;
        for i in k..bs.len(){
            vc+= if "aeiou".bytes().find(|x|*x==bs[i]).is_some(){1}else{0}-if "aeiou".bytes().find(|x|*x==bs[i-k]).is_some(){1}else{0};
            ans=ans.max(vc);
        }
        ans
    }
}
// @lc code=end

