/*
 * @lc app=leetcode id=150 lang=rust
 *
 * [150] Evaluate Reverse Polish Notation
 */

// @lc code=start
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut s = Vec::new();
        for t in &tokens {
            let (l, r) = match t.as_str() {
                "*" | "/" | "+" | "-" => {
                    let r = s.pop().unwrap();
                    let l = s.pop().unwrap();
                    (l, r)
                }
                _ => (0, 0),
            };
            let n = match t.as_str() {
                "*" => l * r,
                "/" => l / r,
                "+" => l + r,
                "-" => l - r,
                _ => t.parse::<i32>().unwrap(),
            };
            s.push(n);
        }
        s[0]
    }
}
// @lc code=end
