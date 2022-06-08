/*
 * @lc app=leetcode id=1941 lang=rust
 *
 * [1941] Check if All Characters Have Equal Number of Occurrences
 */

// @lc code=start
impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut a = vec![0;26];
        for c in s.chars(){
            a[(c as u8-'a' as u8) as usize]+=1;
        }
        let mut cnt = 0;
        for i in &a{
            if *i>0 && cnt>0 && *i!=cnt{
                return false;
            }else if *i>0 && cnt==0{
                cnt=*i;
            }
        }
        true
    }
}
// @lc code=end

