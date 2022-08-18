// 1256\. Encode Number
// ====================

// Given a non-negative integer `num`, Return its _encoding_ string.

// The encoding is done by converting the integer to a string using a secret function that you should deduce from the following table:

// ![](https://assets.leetcode.com/uploads/2019/06/21/encode_number.png)

// **Example 1:**

// **Input:** num = 23
// **Output:** "1000"

// **Example 2:**

// **Input:** num = 107
// **Output:** "101100"

// **Constraints:**

// *   `0 <= num <= 10^9`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Quora](https://leetcode.ca/tags/#Quora)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn encode(num: i32) -> String {
        let mut num = num + 1;
        let mut ans = String::new();
        while num > 0 {
            ans = (if num & 1 > 0 { 1 } else { 0 }).to_string() + ans.as_str();
            num >>= 1;
        }
        ans[1..].to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_encode_1() {
        assert_eq!(String::from("1000"), Solution::encode(23));
    }
    #[test]
    pub fn test_encode_2() {
        assert_eq!(String::from("101100"), Solution::encode(107));
    }
}
