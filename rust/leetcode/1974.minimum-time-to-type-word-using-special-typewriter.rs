/*
 * @lc app=leetcode id=1974 lang=rust
 *
 * [1974] Minimum Time to Type Word Using Special Typewriter
 */

// @lc code=start
impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut ans = 0;
        let mut pre =('a' as u8) as i8;
        for cc in word.bytes(){
            let c = cc as i8;
            let  d = ((c-pre+26)%26).min((pre-c+26)%26) as i32;
            ans+=d  +1;
            pre = c;
        }
        ans 
    }
}
// @lc code=end

