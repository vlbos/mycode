/*
 * @lc app=leetcode id=301 lang=rust
 *
 * [301] Remove Invalid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        use std::collections::HashSet;
        let mut ans = Vec::new();
        let mut curr = HashSet::new();
        curr.insert(s.clone());
        let is_valid = |s: &String| -> bool {
            let mut count = 0;
            for c in s.chars() {
                if c == '(' {
                    count += 1;
                } else if c == ')' {
                    if count == 0 {
                        return false;
                    }
                    count -= 1;
                }
            }
            count == 0
        };
        loop {
            for str in &curr {
                if is_valid(str) {
                    ans.push(str.clone());
                }
            }
            if !ans.is_empty() {
                return ans;
            }
            let mut next = HashSet::new();
            for str in &curr {
                for (i, c) in str.chars().enumerate() {
                    if i > 0 && c == str.chars().nth(i - 1).unwrap() {
                        continue;
                    }
                    if c == '(' || c == ')' {
                        next.insert((str[..i].to_string() + &str[i + 1..]).clone());
                    }
                }
            }
            curr = next;
        }
        ans
    }
}
// @lc code=end
