/*
 * @lc app=leetcode id=1807 lang=rust
 *
 * [1807] Evaluate the Bracket Pairs of a String
 */

// @lc code=start
impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        use std::collections::HashMap;
        let mut m: HashMap<String, String> = knowledge
            .iter()
            .map(|x| (x[0].clone(), x[1].clone()))
            .collect();
        let mut ans = String::new();
        let mut key = String::new();
        let mut is_key = false;
        for c in s.chars() {
            if c == '(' {
                is_key = true;
            } else if c == ')' {
                if let Some(v) = m.get(&key) {
                    ans.push_str(v.as_str());
                } else {
                    ans.push('?');
                }
                key = String::new();
                is_key = false;
            } else {
                if is_key {
                    key.push(c);
                } else {
                    ans.push(c);
                }
            }
        }
        ans
    }
}
// @lc code=end
