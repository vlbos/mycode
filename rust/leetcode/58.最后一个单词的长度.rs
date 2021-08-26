/*
 * @lc app=leetcode.cn id=58 lang=rust
 *
 * [58] 最后一个单词的长度
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        // if s.is_empty() || s ==" "{
        //     return 0;
        // }
        // let mut pre = 0;
        // let mut n = 0;
        // for (i,c) in s.chars().enumerate(){
        //     if c== ' ' {
        //        if(n>0)
        //         {
        //          pre=n;
        //         }
        //        n = 0;
        //     }
        //     else {
        //         n+=1;
        //     }
        // }
        // if n>0 {n}else{pre}
        let mut len = 0;
        for c in s.chars().rev() {
            if c == ' ' {
                if (len > 0) {
                    break;
                }
            } else {
                len += 1;
            }
        }
        len
    }
}
// @lc code=end
