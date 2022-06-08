/*
 * @lc app=leetcode id=30 lang=rust
 *
 * [30] Substring with Concatenation of All Words
 */

// @lc code=start
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.is_empty() || words.is_empty() || s.len() < words[0].len() {
            return Vec::new();
        }
        let one_len = words[0].len();
        let ws_len = words.len();
        let n = s.len();
        use std::collections::HashMap;
        let mut ws = HashMap::new();
        for w in &words{
            *ws.entry(w.clone()).or_insert(0)+=1;
        }
        let m = ws_len * one_len;
        let bs = s.as_bytes();
        let mut ans = Vec::new();
        for i in 0..one_len {
            let mut l = i;
            let mut r = i;
            let mut curs = HashMap::new();
            while r + one_len <= n {
                let cur = s[r..r + one_len].to_string();
                if !ws.contains_key(&cur) {
                    curs.clear();
                    r += one_len;
                    l = r;
                    continue;
                }
                *curs.entry(cur.clone()).or_insert(0)+=1;

                while *curs.get(&cur).unwrap_or(&0)> *ws.get(&cur).unwrap_or(&0){
                    let cur_l = s[l..l + one_len].to_string();
                    if *curs.get(&cur_l).unwrap_or(&0)<2{
                            curs.remove(&cur_l);
                    }else{
                         *curs.entry(cur_l.clone()).or_insert(0)-=1;
                    }
                    l += one_len;
                }
                if  curs==ws  {
                    ans.push(l as i32);
                    let cur_l = s[l..l + one_len].to_string();
                    if *curs.get(&cur_l).unwrap_or(&0)<2{
                            curs.remove(&cur_l);
                    }else{
                         *curs.entry(cur_l.clone()).or_insert(0)-=1;
                    }
                    l += one_len;
                }
                r += one_len;
            }
        }
        ans
    }
}
// @lc code=end
