// 3860. Unique Email Groups
// ## Description
// You are given an array of strings `emails`, where each string is a valid email address.
// Two email addresses belong to the same group if **both** their **normalized** local names and **normalized** domain names are **identical**.
// The normalization rules are as follows:
// * The local name is the part **before** the `'@'` symbol.
// * Ignore all dots `'.'`.
// * Ignore everything after the first `'+'`, if present.
// * Convert to lowercase.
// * The domain name is the part **after** the `'@'` symbol.
// * Convert to lowercase.
// Return an integer denoting the number of **unique** email groups after normalization.

// **Example 1:**
// **Input:** emails = ["test.email+alex@leetcode.com", "test.e.mail+bob.cathy@leetcode.com", "testemail+david@lee.tcode.com"]
// **Output:** 2
// **Explanation:**
// |Email|Local|Normalized Local|Domain|Normalized Domain|Final Email|
// |test.email+alex@leetcode.com|test.email+alex|testemail|leetcode.com|leetcode.com|testemail@leetcode.com|
// |test.e.mail+bob.cathy@leetcode.com|test.e.mail+bob.cathy|testemail|leetcode.com|leetcode.com|testemail@leetcode.com|
// |testemail+david@lee.tcode.com|testemail+david|testemail|lee.tcode.com|lee.tcode.com|testemail@lee.tcode.com|
// Unique emails are [`"testemail@leetcode.com"`, `"testemail@lee.tcode.com"`]. Thus, the answer is 2.
// **Example 2:**
// **Input:** emails = ["A@B.com", "a@b.com", "ab+xy@b.com", "a.b@b.com"]
// **Output:** 2
// **Explanation:**
// |Email|Local|Normalized Local|Domain|Normalized Domain|Final Email|
// |A@B.com|A|a|B.com|b.com|a@b.com|
// |a@b.com|a|a|b.com|b.com|a@b.com|
// |ab+xy@b.com|ab+xy|ab|b.com|b.com|ab@b.com|
// |a.b@b.com|a.b|ab|b.com|b.com|ab@b.com|
// Unique emails are [`"a@b.com"`, `"ab@b.com"`]. Thus, the answer is 2.
// **Example 3:**
// **Input:** emails = ["a.b+c.d+e@DoMain.com", "ab+xyz@domain.com", "ab@domain.com"]
// **Output:** 1
// **Explanation:**
// |Email|Local|Normalized Local|Domain|Normalized Domain|Final Email|
// |a.b+c.d+e@DoMain.com|a.b+c.d+e|ab|DoMain.com|domain.com|ab@domain.com|
// |ab+xyz@domain.com|ab+xyz|ab|domain.com|domain.com|ab@domain.com|
// |ab@domain.com|ab|ab|domain.com|domain.com|ab@domain.com|
// All emails normalize to `"ab@domain.com"`. Thus, the answer is 1.

// **Constraints:**
// * `1 \<= emails.length \<= 1000`
// * `1 \<= emails[i].length \<= 100`
// * `emails[i]` consists of lowercase and uppercase English letters, digits, and the characters `'.'`, `'+'`, and `'@'`.
// * Each `emails[i]` contains **exactly** one `'@'` character.
// * All local and domain names are non-empty; local names do not start with `'+'`.
// * Domain names end with the `".com"` suffix and contain at least one character before `".com"`.

// int unique_email_groups(vector<string>& emails) {

pub struct Solution;
impl Solution {
    pub fn unique_email_groups(emails: Vec<String>) -> i32 {0}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_vec_s;
    #[test]
    pub fn test_unique_email_groups_1() {
        assert_eq!(
            2,
            Solution::unique_email_groups(lc_vec_s!["test.email+alex@leetcode.com", "test.e.mail+bob.cathy@leetcode.com", "testemail+david@lee.tcode.com"])
        );
    }
    #[test]
    pub fn test_unique_email_groups_2() {
        assert_eq!(
            2,
            Solution::unique_email_groups(lc_vec_s!["A@B.com", "a@b.com", "ab+xy@b.com", "a.b@b.com"])
        );
    }
    #[test]
    pub fn test_unique_email_groups_3() {
        assert_eq!(1, Solution::unique_email_groups(lc_vec_s!["a.b+c.d+e@DoMain.com", "ab+xyz@domain.com", "ab@domain.com"]));
    }
}
