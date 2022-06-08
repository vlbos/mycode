/*
 * @lc app=leetcode id=1106 lang=rust
 *
 * [1106] Parsing A Boolean Expression
 */

// @lc code=start
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let (mut opr, mut ops) = (Vec::new(), Vec::new());
        for c in expression.chars() {
            match c {
                '&' | '|' | '!' => opr.push(c),
                't' | 'f' | '(' => ops.push(c),
                ')' => {
                    let op = opr.pop().unwrap();
                    match op {
                        '&' => {
                            let mut v = if ops.pop().unwrap() == 't' {
                                true
                            } else {
                                false
                            };
                            while *ops.last().unwrap() != '(' && v {
                                v &= if ops.pop().unwrap() == 't' {
                                    true
                                } else {
                                    false
                                };
                            }
                            while *ops.last().unwrap() != '(' {
                                ops.pop();
                            }
                            ops.pop();
                            ops.push(if v { 't' } else { 'f' });
                        }
                        '|' => {
                            let mut v = if ops.pop().unwrap() == 't' {
                                true
                            } else {
                                false
                            };
                            while *ops.last().unwrap() != '(' && !v {
                                v |= if ops.pop().unwrap() == 't' {
                                    true
                                } else {
                                    false
                                };
                            }
                            while *ops.last().unwrap() != '(' {
                                ops.pop();
                            }
                            ops.pop();
                            ops.push(if v { 't' } else { 'f' });
                        }
                        _ => {
                            let v = if ops.pop().unwrap() == 't' {
                                true
                            } else {
                                false
                            };
                            ops.pop();
                            ops.push(if !v { 't' } else { 'f' });
                        }
                    }
                },
                _=>{},
            }
        }
        if *ops.last().unwrap() == 't' {
            true
        } else {
            false
        }
    }
}
// @lc code=end
