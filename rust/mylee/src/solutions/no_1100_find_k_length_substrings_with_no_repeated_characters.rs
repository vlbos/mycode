// 1100\. Find K-Length Substrings With No Repeated Characters
// ===========================================================

// Given a string `S`, return the number of substrings of length `K` with no repeated characters.

// **Example 1:**

// **Input:** S = "havefunonleetcode", K = 5
// **Output:** 6
// **Explanation:**
// There are 6 substrings they are : 'havef','avefu','vefun','efuno','etcod','tcode'.

// **Example 2:**

// **Input:** S = "home", K = 5
// **Output:** 0
// **Explanation:**
// Notice K can be larger than the length of S. In this case is not possible to find any substring.

// **Note:**

// 1.  `1 <= S.length <= 10^4`
// 2.  All characters of S are lowercase English letters.
// 3.  `1 <= K <= 10^4`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut cnt = std::collections::HashMap::new();
        let k = k as usize;

        if s.len() < k {
            return 0;
        }
        let bs = s.as_bytes();
        for b in s[..k].bytes() {
            *cnt.entry(b).or_insert(0) += 1;
        }
        if cnt.len() == k {
            ans += 1;
        }
        for i in k..bs.len() {
            let pre = bs[i - k];
            *cnt.entry(pre).or_insert(0) -= 1;
            if *cnt.get(&pre).unwrap() == 0 {
                cnt.remove(&pre);
            }
            *cnt.entry(bs[i]).or_insert(0) += 1;

            if cnt.len() == k {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_k_len_substr_no_repeats_1() {
        assert_eq!(
            6,
            Solution::num_k_len_substr_no_repeats(String::from("havefunonleetcode"), 5)
        );
    }
    #[test]
    fn test_num_k_len_substr_no_repeats_2() {
        assert_eq!(
            0,
            Solution::num_k_len_substr_no_repeats(String::from("home"), 5)
        );
    }
}
