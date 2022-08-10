// 772\. Basic Calculator III
// ==========================

// Implement a basic calculator to evaluate a simple expression string.

// The expression string may contain open `(` and closing parentheses `)`, the plus `+` or minus sign `-`, **non-negative** integers and empty spaces .

// The expression string contains only non-negative integers, `+`, `-`, `*`, `/` operators , open `(` and closing parentheses `)` and empty spaces .
// The integer division should truncate toward zero.

// You may assume that the given expression is always valid. All intermediate results will be in the range of `[-2147483648, 2147483647]`.

// Some examples:

// "1 + 1" = 2
// " 6-4 / 2 " = 4
// "2\*(5+5\*2)/3+(6/2+8)" = 21
// "(2+6\* 3+5- (3\*14/7+2)\*5)+3"=-12

// **Note:** **Do not** use the `eval` built-in library function.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [DoorDash](https://leetcode.ca/tags/#DoorDash) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Houzz](https://leetcode.ca/tags/#Houzz) [Hulu](https://leetcode.ca/tags/#Hulu) [Jingchi](https://leetcode.ca/tags/#Jingchi) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Pinterest](https://leetcode.ca/tags/#Pinterest) [Pocket Gems](https://leetcode.ca/tags/#Pocket%20Gems) [Salesforce](https://leetcode.ca/tags/#Salesforce) [Snapchat](https://leetcode.ca/tags/#Snapchat) [Uber](https://leetcode.ca/tags/#Uber)
 
#[allow(dead_code)] 
 pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn calculate(s: String) -> i32 {
        let n = s.len();
        let (mut num, mut cur_res) = (0, 0);
        let mut ans = 0;
        let mut i = 0;
        let bs = s.as_bytes();
        let mut op = '+';
        while i < n {
            let c = bs[i] as char;
            if c.is_ascii_digit() {
                num *= 10;
                num += (c as u8 - b'0') as i64;
            } else if c == '(' {
                let j = i;
                let mut cnt = 0;
                while i < n {
                    if bs[i] == b'(' {
                        cnt += 1;
                    } else if bs[i] == b')' {
                        cnt -= 1;
                    }
                    if cnt == 0 {
                        break;
                    }

                    i += 1;
                }
                num = Self::calculate(s[j + 1..i].to_string()) as i64;
            }
            if i == n - 1 || "+-*/".chars().any(|x| x == c) {
                match op {
                    '-' => cur_res -= num,
                    '*' => cur_res *= num,
                    '/' => cur_res /= num,
                    _ => cur_res += num,
                };
                if i == n - 1 || c == '+' || c == '-' {
                    ans += cur_res;
                    cur_res = 0;
                }
                num = 0;
                op = c;
            }

            i += 1;
        }
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_1() {
        assert_eq!(2, Solution::calculate(String::from("1 + 1")));
    }
    #[test]
    fn test_calculate_2() {
        assert_eq!(4, Solution::calculate(String::from(" 6-4 / 2 ")));
    }
    #[test]
    fn test_calculate_3() {
        assert_eq!(21, Solution::calculate(String::from("2*(5+5*2)/3+(6/2+8)")));
    }
    #[test]
    fn test_calculate_4() {
        assert_eq!(
            -12,
            Solution::calculate(String::from("(2+6* 3+5- (3*14/7+2)*5)+3"))
        );
    }
    #[test]
    fn test_calculate_5() {
        assert_eq!(
            -2147483648,
            Solution::calculate(String::from("0-2147483648"))
        );
    }
}
