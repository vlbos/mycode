/*
 * @lc app=leetcode id=809 lang=rust
 *
 * [809] Expressive Words
 */

// @lc code=start
impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let rle = |s: &String| -> (String,Vec<i32>) {
            let mut ans = String::new();
            let mut cnt = Vec::new();
            let mut pre = -1;
            for (i, c) in s.chars().enumerate() {
                if i == s.len() - 1 || c != s.chars().nth(i + 1).unwrap() {
                    ans.push(c);
                    cnt.push(i as i32- pre);
                    pre = i as i32;
                }
            }
            (ans, cnt)
        };
        let sr = rle(&s);
        let mut ans = 0;
        for w in &words {
            let wr = rle(w);
            if sr.0 != wr.0 {
                continue;
            }
            let mut flag = true;
            for i in 0..wr.1.len() {
                let c1 = sr.1[i];
                let c2 = wr.1[i];
                if c1 < 3 && c1 != c2 || c1 < c2 {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
