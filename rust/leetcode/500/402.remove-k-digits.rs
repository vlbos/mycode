/*
 * @lc app=leetcode id=402 lang=rust
 *
 * [402] Remove K Digits
 */

// @lc code=start
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k as usize;
        if num.len() <= k {
            return "0".to_string();
        }
        let mut ans = Vec::new();
        for b in num.bytes() {
            while !ans.is_empty() && b < *(ans.last().unwrap()) && k > 0 {
                ans.pop();
                k -= 1;
            }
            ans.push(b);
        }
        while k>0{
            ans.pop();
            k-=1;
        }

        let mut s = ans
            .iter()
            .map(|x| *x as char)
            .collect::<String>();
          s=  s.trim_start_matches('0').to_string();
        if s.is_empty() {
            "0".to_string()
        } else {
            s.to_string()
        }
    }
}
// @lc code=end
