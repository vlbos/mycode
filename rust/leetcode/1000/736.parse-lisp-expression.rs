/*
 * @lc app=leetcode id=736 lang=rust
 *
 * [736] Parse Lisp Expression
 */

// @lc code=start
impl Solution {
    pub fn evaluate(expression: String) -> i32 {
         use std::collections::HashMap;
        let mut scope = Vec::new();
        scope.push(HashMap::new());
        fn evaluate_outer(expression: &String, scope: &mut Vec<HashMap<String, i32>>) -> i32 {
            scope.push(HashMap::new());
            let ans = evaluate_inner(expression, scope);
            scope.pop();
            ans
        }
        fn evaluate_inner(expression: &String, scope: &mut Vec<HashMap<String, i32>>) -> i32 {
            let first = expression.chars().nth(0).unwrap();
            if first != '(' {
                if first.is_ascii_digit() || first == '-' {
                    return expression.parse::<i32>().unwrap();
                }
                for m in scope.iter().rev() {
                    if m.contains_key(expression) {
                        return *m.get(expression).unwrap();
                    }
                }
            }
            let i = if expression.chars().nth(1).unwrap_or(' ') == 'm' {
                6
            } else {
                5
            };
            let tokens = parse(expression[i..expression.len()-1].to_string());

            if expression[1..].starts_with("add") {
                return evaluate_outer(&tokens[0],scope) + evaluate_outer(&tokens[1],scope);
            }
            if expression[1..].starts_with("mult") {
                return evaluate_outer(&tokens[0],scope) * evaluate_outer(&tokens[1],scope);
            }

            for j in (1..tokens.len()).step_by(2) {
                let v = evaluate_outer(&tokens[j], scope);
                let last = scope.len() - 1;
                scope[last].insert(tokens[j - 1].clone(), v);
            }
            evaluate_outer(&tokens[tokens.len() - 1], scope)
        }
        fn parse(expression: String) -> Vec<String> {
            let mut ans = Vec::new();
            let mut buf = String::new();
            let mut bal = 0;
            for token in expression.split_ascii_whitespace() {
                for c in token.chars(){
                    if c == '(' {
                        bal += 1;
                    }
                    if c == ')' {
                        bal -= 1;
                    }
                }
                if !buf.is_empty() {
                    buf.push(' ');
                }
                buf.push_str(token);
                if bal == 0 {
                    ans.push(buf.clone());
                    buf = String::new();
                }
            }
            if !buf.is_empty() {
                ans.push(buf.clone());
            }
            ans
        }
        evaluate_outer(&expression, &mut scope)
    }
}
// @lc code=end
