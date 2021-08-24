/*
 * @lc app=leetcode.cn id=383 lang=rust
 *
 * [383] 赎金信
 */

// @lc code=start
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut r = vec![0;26];
        let mut m = vec![0;26];
        for c in ransom_note.chars(){
            r[(c as u8 -'a' as u8) as usize]+=1;
        }
        for c in magazine.chars(){
            m[(c as u8 -'a' as u8 ) as usize]+=1;
        }
        for i in 0..26{
           if r[i]>m[i]{
            return false;
            }
        }
        true
    }
}
// @lc code=end

