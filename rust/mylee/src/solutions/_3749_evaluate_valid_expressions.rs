// # 3749. Evaluate Valid Expressions 🔒

// Description
// -----------

// You are given a string `expression` that represents a nested mathematical expression in a simplified form.

// A **valid** expression is either an integer **literal** or follows the format `op(a,b)`, where:

// *   `op` is one of `"add"`, `"sub"`, `"mul"`, or `"div"`.
// *   `a` and `b` are each valid expressions.

// The **operations** are defined as follows:

// *   `add(a,b) = a + b`
// *   `sub(a,b) = a - b`
// *   `mul(a,b) = a * b`
// *   `div(a,b) = a / b`

// Return an integer representing the **result** after fully evaluating the expression.

// **Example 1:**

// **Input:** expression = "add(2,3)"

// **Output:** 5

// **Explanation:**

// The operation `add(2,3)` means `2 + 3 = 5`.

// **Example 2:**

// **Input:** expression = "-42"

// **Output:** \-42

// **Explanation:**

// The expression is a single integer literal, so the result is -42.

// **Example 3:**

// **Input:** expression = "div(mul(4,sub(9,5)),add(1,1))"

// **Output:** 8

// **Explanation:**

// *   First, evaluate the inner expression: `sub(9,5) = 9 - 5 = 4`
// *   Next, multiply the results: `mul(4,4) = 4 * 4 = 16`
// *   Then, compute the addition on the right: `add(1,1) = 1 + 1 = 2`
// *   Finally, divide the two main results: `div(16,2) = 16 / 2 = 8`

// Therefore, the entire expression evaluates to 8.

// **Constraints:**

// *   `1 <= expression.length <= 105`
// *   `expression` is valid and consists of digits, commas, parentheses, the minus sign `'-'`, and the lowercase strings `"add"`, `"sub"`, `"mul"`, `"div"`.
// *   All intermediate results fit within the range of a long integer.
// *   All divisions result in integer values.

// // long long evaluate_expression(string expression) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn evaluate_expression(expression: String) -> i64 {
        let m: std::collections::HashMap<_, _> = ["add", "sub", "mul", "div"]
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect();
        let (mut op_st, mut st, mut op, mut num, mut signed) =
            (vec![], vec![], String::new(), 0, false);
        let n = expression.len();
        let exp = expression.as_bytes();
        for (i, c) in expression.chars().enumerate() {
            match c {
                ')' => {
                    let num2 = st.pop().unwrap();
                    match op_st.pop().unwrap() {
                        0 => {
                            let v = st.pop().unwrap() + num2;
                            st.push(v);
                        }
                        1 => {
                            let v = st.pop().unwrap() - num2;
                            st.push(v);
                        }
                        2 => {
                            let v = st.pop().unwrap() * num2;
                            st.push(v);
                        }
                        _ => {
                            let v = st.pop().unwrap() / num2;
                            st.push(v);
                        }
                    }
                }
                ',' => {}
                '(' => {
                    op_st.push(m[&op.as_str()]);
                    op = String::new();
                }
                '-' => {
                    signed = true;
                }
                _ if c.is_ascii_alphabetic() => {
                    op.push(c);
                }
                _ => {
                    // println!("{num}");
                    num = num * 10 + c.to_digit(10).unwrap() as i64;
                    if i + 1 == n || !(exp[i + 1] as char).is_ascii_digit() {
                        if signed {
                            num *= -1;
                        }
                        signed = false;
                        st.push(num);
                        num = 0;
                    }
                }
            }
            // println!("{op_st:?},={c}=={st:?},");
        }
        // while let Some(op) = op_st.pop() {
        //     num = st.pop().unwrap();
        //     match op {
        //         0 => {
        //             let v = st.pop().unwrap() + num;
        //             st.push(v);
        //         }
        //         1 => {
        //             let v = st.pop().unwrap() - num;
        //             st.push(v);
        //         }
        //         2 => {
        //             let v = st.pop().unwrap() * num;
        //             st.push(v);
        //         }
        //         _ => {
        //             let v = st.pop().unwrap() / num;
        //             st.push(v);
        //         }
        //     }
        // }
        if signed {
            num *= -1;
        }
        *st.get(0).unwrap_or(&num)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_evaluate_expression_1() {
        assert_eq!(5, Solution::evaluate_expression("add(2,3)".to_string()));
    }
    #[test]
    pub fn test_evaluate_expression_2() {
        assert_eq!(-42, Solution::evaluate_expression("-42".to_string()));
    }
    #[test]
    pub fn test_evaluate_expression_3() {
        assert_eq!(
            8,
            Solution::evaluate_expression("div(mul(4,sub(9,5)),add(1,1))".to_string())
        );
    }
}
