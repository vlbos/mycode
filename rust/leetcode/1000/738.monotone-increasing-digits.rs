/*
 * @lc app=leetcode id=738 lang=rust
 *
 * [738] Monotone Increasing Digits
 */

// @lc code=start
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut ns = n.to_string().bytes().collect::<Vec<u8>>();
        let mut i = 1;
        while i<ns.len() && ns[i-1]<=ns[i]{
                i+=1;
        }
        if i<ns.len(){
            while i>0 && ns[i-1]>ns[i]{
                ns[i-1]-=1;
                i-=1;
            }
            for j in i+1..ns.len(){
                ns[j]=b'9';
            }
        }
        String::from_utf8(ns).unwrap_or(n.to_string()).parse::<i32>().unwrap()
    }
}
// @lc code=end

