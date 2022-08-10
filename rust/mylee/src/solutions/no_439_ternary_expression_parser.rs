// 439\. Ternary Expression Parser
// ===============================

// Given a string representing arbitrarily nested ternary expressions, calculate the result of the expression.
// You can always assume that the given expression is valid and only consists of digits `0-9`, `?`, `:`, `T` and `F` (`T` and `F` represent True and False respectively).

// **Note:**

// 1.  The length of the given string is ≤ 10000.
// 2.  Each number will contain only one digit.
// 3.  The conditional expressions group right-to-left (as usual in most languages).
// 4.  The condition will always be either `T` or `F`. That is, the condition will never be a digit.
// 5.  The result of the expression will always evaluate to either a digit `0-9`, `T` or `F`.

// **Example 1:**

// **Input:** "T?2:3"

// **Output:** "2"

// **Explanation:** If true, then result is 2; otherwise result is 3.

// **Example 2:**

// **Input:** "F?1:T?4:5"

// **Output:** "4"

// **Explanation:** The conditional expressions group right-to-left. Using parenthesis, it is read/evaluated as:

//              "(F ? 1 : (T ? 4 : 5))"                   "(F ? 1 : (T ? 4 : 5))"
//           -> "(F ? 1 : 4)"                 or       -> "(T ? 4 : 5)"
//           -> "4"                                    -> "4"

// **Example 3:**

// **Input:** "T?T?F:5:3"

// **Output:** "F"

// **Explanation:** The conditional expressions group right-to-left. Using parenthesis, it is read/evaluated as:

//              "(T ? (T ? F : 5) : 3)"                   "(T ? (T ? F : 5) : 3)"
//           -> "(T ? F : 3)"                 or       -> "(T ? F : 5)"
//           -> "F"                                    -> "F"

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Snapchat](https://leetcode.ca/tags/#Snapchat)

// @lc code=start
impl Solution {
    pub fn parse_ternary(expression: String) -> String {
        // let chars = expression.chars().collect::<Vec<_>>();
        // if chars.len() <= 0 {
        //     return String::from("");
        // }
        // let mut stack = vec![];
        // let mut i = (chars.len() - 1) as i32;
        // while i >= 0 {
        //     let add = match chars[i as usize] {
        //         '?' => {
        //             let condition = chars[(i - 1) as usize] == 'T';
        //             i -= 1;
        //             let left = stack.pop().unwrap();
        //             stack.pop();
        //             let right = stack.pop().unwrap();
        //             if condition {
        //                 left
        //             } else {
        //                 right
        //             }
        //         }
        //         ch => ch,
        //     };
        //     stack.push(add);
        //     i -= 1;
        // }
        // String::from(stack[0])
        let mut ans = expression;
        while ans.len() > 1 {
            if let Some(i) = ans.rfind("?") {
                let ba = ans.as_bytes();
                ans = format!(
                    "{}{}{}",
                    &ans[..i - 1],
                    ba[i + if ba[i - 1] == b'T' { 1 } else { 3 }] as char,
                    &ans[i + 4..]
                );
            }
        }
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub  struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_ternary_1() {
        assert_eq!(Solution::parse_ternary(String::from("T?2:3")), "2");
    }

    #[test]
    fn test_parse_ternary_2() {
        assert_eq!(Solution::parse_ternary(String::from("F?1:T?4:5")), "4");
    }

    #[test]
    fn test_parse_ternary_3() {
        assert_eq!(Solution::parse_ternary(String::from("T?T?F:5:3")), "F");
    }
}
