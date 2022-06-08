/*
 * @lc app=leetcode id=68 lang=rust
 *
 * [68] Text Justification
 */

// @lc code=start
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut ans = Vec::new();
        let n = words.len();
        let mw = max_width as usize;
        let mut r = 0;
        loop {
            let l = r;
            let mut sum = 0;
            while r < n && sum + words[r].len() + r - l <= mw {
                sum += words[r].len();
                r += 1;
            }
            if r == n {
                let s = words[l..].join(" ");
                let sn = mw - s.len();
                ans.push(s+" ".repeat(sn).as_str());
                break;
            }
            let nw = r - l;
            let ns = mw - sum;
            if nw == 1 {
                ans.push(words[l].clone() + " ".repeat(ns).as_str());
                continue;
            }
            let avgs = ns / (nw - 1);
            let es = ns % (nw - 1);
            let s1 = words[l..l + es + 1].join(" ".repeat(avgs + 1).as_str());
            let s2 = words[l + es + 1..r].join(" ".repeat(avgs).as_str());
            ans.push(s1 + " ".repeat(avgs).as_str() + s2.as_str());
        }
        ans
    }
}
// @lc code=end
