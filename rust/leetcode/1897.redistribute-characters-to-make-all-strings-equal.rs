/*
 * @lc app=leetcode id=1897 lang=rust
 *
 * [1897] Redistribute Characters to Make All Strings Equal
 */

// @lc code=start
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        if words.len()==1{
            return true;
        }
        let mut a =vec![0;26];
        for w in &words{
            for c in w.chars(){
                a[(c as u8-'a' as u8) as usize]+=1;
            }
        }
        for n in &a{
            if *n>0 && (*n)%words.len()!=0{
                return false;
            }
        }
        true
    }
}
// @lc code=end

