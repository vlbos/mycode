/*
 * @lc app=leetcode.cn id=168 lang=rust
 *
 * [168] Excel表列名称
 */

// @lc code=start
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
         let mut n = column_number;
         let al = 26;
         let mut r = String::new();
         let a = 'A' as u8;
         while n>0{
            r.insert(0,(a+(((n-1)%al) as u8)) as char);
            n=n/al-if n%al==0{1}else{0};
        }
        r
    }
}
// @lc code=end

