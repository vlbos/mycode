/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = std::collections::VecDeque::new();
        for c in s.chars() {
            if let Some(i) = String::from(")}]").find(|ch: char| ch == c) {
                if let Some(l) = stack.front() {
                    if let Some(j) = String::from("({[").find(|ch: char| ch == *l) {
                        if i != j {
                            return false;
                        }
                        stack.pop_front();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                stack.push_front(c)
            }
        }

        stack.is_empty()
    }
}
// @lc code=end
