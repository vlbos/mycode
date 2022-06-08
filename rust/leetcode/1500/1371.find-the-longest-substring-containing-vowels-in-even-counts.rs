/*
 * @lc app=leetcode id=1371 lang=rust
 *
 * [1371] Find the Longest Substring Containing Vowels in Even Counts
 */

// @lc code=start
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut status = 0;
        let mut pos=vec![s.len();(1<<5) as usize];
        pos[0]=0;
        for (i,b) in s.bytes().enumerate(){
            if let Some(j)="aeoiu".bytes().position(|x|x==b){
                status^= 1<<j;
            }
            if pos[status]==s.len(){
                    pos[status]=i+1;
            }else{
                ans = ans.max(i+1-pos[status]);
            }
        }
        ans as _
    }
}
// @lc code=end

