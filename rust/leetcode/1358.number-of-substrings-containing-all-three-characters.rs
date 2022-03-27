/*
 * @lc app=leetcode id=1358 lang=rust
 *
 * [1358] Number of Substrings Containing All Three Characters
 */

// @lc code=start
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
       let mut cnt = vec![0;3];
        let bs = s.as_bytes();
        let mut r = 0;
        let mut ans = 0;
        for l in 0..bs.len(){
             while r<bs.len() && cnt.iter().any(|x|*x==0){
                cnt[(bs[r]-b'a') as usize]+=1;
                r+=1;
             }
             if cnt.iter().all(|x|*x>0){
             ans+=bs.len()-r+1;
             }
             cnt[(bs[l]-b'a') as usize]-=1;
        }
        ans as _
    }
}
// @lc code=end

