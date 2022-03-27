/*
 * @lc app=leetcode id=1208 lang=rust
 *
 * [1208] Get Equal Substrings Within Budget
 */

// @lc code=start
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        if s==t{
            return s.len() as i32;
        }
        let n = s.len();
        let mut acc_diff=vec![0;n+1];
        for i in 0..n{
            let (si,ti)=(s.bytes().nth(i).unwrap() as i32,t.bytes().nth(i).unwrap() as i32);
            acc_diff[i+1]=acc_diff[i]+ (si-ti).abs();
        }
        let mut max_len = 0;
        for i in 1..=n{
            if let Ok(start)|Err(start) = acc_diff[..i].binary_search(&(acc_diff[i]-max_cost)){
                max_len=max_len.max((i-start) as i32);
            }
        }
        max_len
    }
}
// @lc code=end

