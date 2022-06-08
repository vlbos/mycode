/*
 * @lc app=leetcode id=816 lang=rust
 *
 * [816] Ambiguous Coordinates
 */

// @lc code=start
impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        let x: &[_] = &['(', ')'];
        let s = s.trim_matches(x);
        let dot_split = |s: String| -> Vec<String> {
            let mut ans = Vec::new();
            if s.len() == 1 {
                ans.push(s);
                return ans;
            }
            for i in 1..=s.len() {
                let (left, right) = s.split_at(i);
                if (left != "0" && left.starts_with("0")) || right.ends_with("0") {
                    continue;
                }
                ans.push(format!(
                    "{}{}{}",
                    left,
                    if right.is_empty() { "" } else { "." },
                    right
                ));
            }
            ans
        };
        for i in 1..s.len() {
            let (left, right) = s.split_at(i);
            let lv = dot_split(left.to_string());
            let rv = dot_split(right.to_string());
            for l in &lv {
                for r in &rv {
                    ans.push(format!("({}, {})", l, r));
                }
            }
        }
        ans
    }
}
// @lc code=end
