/*
 * @lc app=leetcode.cn id=482 lang=rust
 *
 * [482] 密钥格式化
 */

// @lc code=start
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut r = String::new();
        let mut cnt =0;
        for c in s.chars().rev(){
            if c!='-'{
               if cnt>0 && cnt%k==0{
                    r.insert(0,'-');
                }
                if c.is_ascii_lowercase(){
                    r.insert(0,c.to_ascii_uppercase());
                }
                else {
                    r.insert(0,c.to_ascii_uppercase());
                }
                cnt+=1;
             }
        }
        r
    }
}
// @lc code=end

