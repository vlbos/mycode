/*
 * @lc app=leetcode id=1896 lang=rust
 *
 * [1896] Minimum Cost to Change the Final Value of Expression
 */

// @lc code=start
impl Solution {
    pub fn min_operations_to_flip(expression: String) -> i32 {
        let mut stack_num = Vec::new();
        let mut stack_op = Vec::new();
        let op_and = |x1: i32, y1: i32, x2: i32, y2: i32| {
            (*[x1 + x2, x1 + y2, x2 + y1].iter().min().unwrap(), y1 + y2)
        };
        let op_or = |x1: i32, y1: i32, x2: i32, y2: i32| {
            (x1 + x2, *[y1 + y2, x1 + y2, x2 + y1].iter().min().unwrap())
        };
        let calc = |stack_num: &mut Vec<(i32, i32)>, stack_op: &mut Vec<char>| {
            if stack_num.len() < 2 || stack_op[stack_op.len() - 1] == '(' {
                return;
            }
            let (x1, y1) = stack_num.pop().unwrap();
            let (x2, y2) = stack_num.pop().unwrap();
            let (x_and, y_and) = op_and(x1, y1, x2, y2);
            let (x_or, y_or) = op_or(x1, y1, x2, y2);
            if stack_op[stack_op.len() - 1] == '&' {
                stack_num.push((x_and.min(x_or + 1), y_and.min(y_or + 1)));
            } else {
                stack_num.push((x_or.min(x_and + 1), y_or.min(y_and + 1)));
            }
            stack_op.pop();
        };
        for ch in expression.chars() {
            match ch {
                '(' | '|' | '&' => stack_op.push(ch),
                '0' | '1' => {
                    let x = (ch as u8 - b'0') as i32;
                    stack_num.push(((x, 1 - x)));
                    calc(&mut stack_num, &mut stack_op);
                }
                _ => {
                    stack_op.pop();
                    calc(&mut stack_num, &mut stack_op);
                }
            }
        }
        stack_num[0].0.max(stack_num[0].1)
    }
}
// @lc code=end
