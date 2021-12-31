/*
 * @lc app=leetcode id=1935 lang=rust
 *
 * [1935] Maximum Number of Words You Can Type
 */

// @lc code=start
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut a =vec![0;26];
        for b in broken_letters.chars(){
            a[(b as u8 - 'a' as u8) as usize]+=1;
        }
        let t = text.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut ans = t.len() as i32;
        for w in t{
            for c in w.chars(){
                if a[(c as u8 - 'a' as u8) as usize]>0{
                    ans-=1;
                    break;
                }
            }
        }
        ans 
    }
}
// @lc code=end

