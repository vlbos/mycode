/*
Implement a basic calculator to evaluate a simple expression string.

The expression string may contain open ( and closing parentheses ), the plus + or minus sign -, non-negative integers and empty spaces  .

The expression string contains only non-negative integers, +, -, *, / operators , open ( and closing parentheses ) and empty spaces  . The integer division should truncate toward zero.

You may assume that the given expression is always valid. All intermediate results will be in the range of [-2147483648, 2147483647].

Some examples:

"1 + 1" = 2
" 6-4 / 2 " = 4
"2*(5+5*2)/3+(6/2+8)" = 21
"(2+6* 3+5- (3*14/7+2)*5)+3"=-12



Note: Do not use the eval built-in library function.

*/
pub struct Solution {}
impl Solution {
    pub fn calculate(s: String) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_1() {
        assert_eq!(0, Solution::calculate(String::new()));
    }
}
