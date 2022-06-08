/*
 * @lc app=leetcode id=424 lang=rust
 *
 * [424] Longest Repeating Character Replacement
 */

// @lc code=start
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut count=vec![0;26];
        let mut maxn = 0;
        let mut l =0;
        let mut r=0;
        let sv = s.bytes().collect::<Vec<u8>>();
        while r<sv.len(){
              let i = (sv[r]-b'A') as  usize;
              count[i]+=1;
              maxn=maxn.max(count[i]);
              if r-l+1-maxn>k as usize{
                count[(sv[l]-b'A') as  usize]-=1;
                l+=1;
              }
              r+=1;
        }
        (r-l) as i32
    }
}
// @lc code=end

