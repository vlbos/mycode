/*
 * @lc app=leetcode id=2131 lang=rust
 *
 * [2131] Longest Palindrome by Concatenating Two Letter Words
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut m = std::collections::HashMap::<String, i32>::new();
        for w in &words {
            *m.entry(w.clone()).or_insert(0) += 1;
        }
        let mut ans = 0;
        let mut f = false;
        for (k, &v) in &m {
            let b = k.as_bytes();
            let mut bb=b.to_vec();
            bb.reverse();
            if b[0] == b[1]{
                if let Some(n) = m.get(k) {
                    if n % 2 > 0 {
                        f = true;
                    }
                    ans += n / 2 * 4;
                }
            } else  if  b.to_vec()>bb{
                let bb = String::from_utf8(bb).unwrap();
                if let Some(&n) = m.get(&bb) {
                    ans += n.min(v)*4;
                }
            } 
        }
        if f {
            ans += 2;
        }

        ans
    }
}
// @lc code=end
