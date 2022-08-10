// 1065\. Index Pairs of a String
// ==============================

// Given a `text` string and `words` (a list of strings), return all index pairs `[i, j]` so that the substring `text[i]...text[j]` is in the list of `words`.

// **Example 1:**

// **Input:** text = "thestoryofleetcodeandme", words = \["story","fleet","leetcode"\]
// **Output:** \[\[3,7\],\[9,13\],\[10,17\]\]

// **Example 2:**

// **Input:** text = "ababa", words = \["aba","ab"\]
// **Output:** \[\[0,1\],\[0,2\],\[2,3\],\[2,4\]\]
// **Explanation:**
// Notice that matches can overlap, see "aba" is found in \[0,2\] and \[2,4\].

// **Note:**

// 1.  All strings contains only lowercase English letters.
// 2.  It's guaranteed that all strings in `words` are different.
// 3.  `1 <= text.length <= 100`
// 4.  `1 <= words.length <= 20`
// 5.  `1 <= words[i].length <= 50`
// 6.  Return the pairs `[i,j]` in sorted order (i.e. sort them by their first coordinate in case of ties sort them by their second coordinate).

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

#[allow(dead_code)]
pub  struct Solution {}
impl Solution {
    pub fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        for i in 0..text.len() {
            for word in &words {
                if &text[i..word.len()] == &word[..] {
                    ans.push(vec![i as i32, (i + word.len()) as i32 - 1]);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_index_pairs_1() {
        assert_eq!(
            vec![vec![3, 7], vec![9, 13], vec![10, 17]],
            Solution::index_pairs(
                String::from("thestoryofleetcodeandme"),
                ["story", "fleet", "leetcode"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }
    #[test]
    fn test_index_pairs_2() {
        assert_eq!(
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
            Solution::index_pairs(
                String::from("ababa"),
                ["aba", "ab"].into_iter().map(String::from).collect()
            )
        );
    }
}
