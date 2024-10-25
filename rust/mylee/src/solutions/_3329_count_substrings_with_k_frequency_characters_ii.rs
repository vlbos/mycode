// [3329\. Count Substrings With K-Frequency Characters II 🔒](https://leetcode.com/problems/count-substrings-with-k-frequency-characters-ii)
// ==========================================================================================================================================


// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// Description
// -----------

// Given a string `s` and an integer `k`, return the total number of substrings of `s` where **at least one** character appears **at least** `k` times.

// **Example 1:**

// **Input:** s = "abacb", k = 2

// **Output:** 4

// **Explanation:**

// The valid substrings are:

// *   "`aba"` (character `'a'` appears 2 times).
// *   `"abac"` (character `'a'` appears 2 times).
// *   `"abacb"` (character `'a'` appears 2 times).
// *   `"bacb"` (character `'b'` appears 2 times).

// **Example 2:**

// **Input:** s = "abcde", k = 1

// **Output:** 15

// **Explanation:**

// All substrings are valid because every character appears at least once.

// **Constraints:**

// *   `1 <= s.length <= 3 * 105`
// *   `1 <= k <= s.length`
// *   `s` consists only of lowercase English letters.


//  long long number_of_substrings(string s, int k) 

// @lc code=end

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String,  k: i32) -> i64 {
        let mut cnt=[0;26];
        let (mut ans,mut l)=(0,0);
        let bs=s.as_bytes();
        for b in s.bytes(){
            let i=(b-b'a') as usize;
            cnt[i]+=1;
            while cnt[i]==k{
                let j=(bs[l]-b'a') as usize;
                cnt[j]-=1;
                l+=1;
            }
            ans+=l as i64;
        }
        ans
    }
}
#[cfg(test)]
mod test {
    use super::*;
   
    #[test]
    pub fn test_number_of_substrings_1() {
        assert_eq!(Solution::number_of_substrings(String::from("abacb"), 2), 4);
    }

    #[test]
    pub fn test_number_of_substrings_2() {
        assert_eq!(Solution::number_of_substrings(String::from("abcde"), 1), 15);
    }

}
