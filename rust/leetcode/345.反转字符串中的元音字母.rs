/*
 * @lc app=leetcode.cn id=345 lang=rust
 *
 * [345] 反转字符串中的元音字母
 */

// @lc code=start
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
            let mut i=0;
            let mut j = s.len()-1;
            let mut c = 'a' as u8;
            let mut e = vec!['a','e','i','o','u','A','E','I','O','U'];
            let mut s = s.into_bytes();
            while i<j{
                while i<j && !e.contains(&(s[i] as char)){
                    i+=1;
                }

                while i<j && !e.contains(&(s[j] as char)){
                    j-=1;
                }
                if i<j{
                    c=s[i];
                    s[i]=s[j];
                    s[j]=c;
                    i+=1;
                    j-=1;
                }
                else{
                break;
                }
            }
            std::str::from_utf8(&s).unwrap().to_string()
    }
}
// @lc code=end

