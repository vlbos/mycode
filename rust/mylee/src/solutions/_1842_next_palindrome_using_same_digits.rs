// 1842\. Next Palindrome Using Same Digits
// ========================================

// You are given a numeric string `num`, representing a very large **palindrome**.

// Return _the **smallest palindrome larger than**_ `num` _that can be created by rearranging its digits. If no such palindrome exists, return an empty string_ `""`.

// A **palindrome** is a number that reads the same backward as forward.

// **Example 1:**

// **Input:** num = "1221"
// **Output:** "2112"
// **Explanation:** The next palindrome larger than "1221" is "2112".

// **Example 2:**

// **Input:** num = "32123"
// **Output:** ""
// **Explanation:** No palindromes larger than "32123" can be made by rearranging the digits.

// **Example 3:**

// **Input:** num = "45544554"
// **Output:** "54455445"
// **Explanation:** The next palindrome larger than "45544554" is "54455445".

// **Constraints:**

// *   `1 <= num.length <= 105`
// *   `num` is a **palindrome**.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

//  public String next_palindrome(String num) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn next_palindrome(num: String) -> String {
        let n = num.len();
        if n < 4 {
            return String::new();
        }
        let next_pal = |bs: &[u8]| -> Option<(usize, usize)> {
            let mut i = bs.len() - 1;
            while i > 0 && bs[i] <= bs[i - 1] {
                i -= 1;
            }
            if i == 0 {
                return None;
            }
            i -= 1;
            let mut j = bs.len() - 1;
            while j > i && bs[j] <= bs[i] {
                j -= 1;
            }
            Some((i, j))
        };
        if let Some((i, j)) = next_pal(num[..n / 2].as_bytes()) {
            let mut s: Vec<char> = num[..n / 2].chars().collect();
            s.swap(i, j);
            s[i + 1..n / 2].reverse();
            let mut ss = s.clone();
            ss.reverse();
            if n % 2 > 0 {
                s.push(num.chars().nth(n / 2).unwrap());
            }
            s.extend(ss);
            s.into_iter().collect()
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_next_palindrome_1() {
        assert_eq!(
            String::from("2112"),
            Solution::next_palindrome(String::from("1221"),)
        );
    }
    #[test]
    pub fn test_next_palindrome_2() {
        assert_eq!(
            String::from(""),
            Solution::next_palindrome(String::from("32123"),)
        );
    }
    #[test]
    pub fn test_next_palindrome_3() {
        assert_eq!(
            String::from("54455445"),
            Solution::next_palindrome(String::from("45544554"),)
        );
    }
}
