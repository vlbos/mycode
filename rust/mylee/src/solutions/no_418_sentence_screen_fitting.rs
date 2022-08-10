// 418\. Sentence Screen Fitting
// =============================

// Given a `rows x cols` screen and a sentence represented by a list of **non-empty** words, find **how many times** the given sentence can be fitted on the screen.

// **Note:**

// 1.  A word cannot be split into two lines.
// 2.  The order of words in the sentence must remain unchanged.
// 3.  Two consecutive words **in a line** must be separated by a single space.
// 4.  Total words in the sentence won't exceed 100.
// 5.  Length of each word is greater than 0 and won't exceed 10.
// 6.  1 ≤ rows, cols ≤ 20,000.

// **Example 1:**

// **Input:**
// rows = 2, cols = 8, sentence = \["hello", "world"\]

// **Output:**
// 1

// **Explanation:**
// hello---
// world---

// The character '-' signifies an empty space on the screen.

// **Example 2:**

// **Input:**
// rows = 3, cols = 6, sentence = \["a", "bcd", "e"\]

// **Output:**
// 2

// **Explanation:**
// a-bcd-
// e-a---
// bcd-e-

// The character '-' signifies an empty space on the screen.

// **Example 3:**

// **Input:**
// rows = 4, cols = 5, sentence = \["I", "had", "apple", "pie"\]

// **Output:**
// 1

// **Explanation:**
// I-had
// apple
// pie-I
// had--

// The character '-' signifies an empty space on the screen.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google) [Robinhood](https://leetcode.ca/tags/#Robinhood) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
impl Solution {
    pub fn words_typing(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
        // let chars = sentence
        //     .into_iter()
        //     .map(|s| s.chars().collect::<Vec<char>>())
        //     .collect::<Vec<_>>();
        // let len = chars.len();
        // let cols = cols as usize;
        // let rows = rows as usize;
        // if len == 0 || rows == 0 || cols == 0 {
        //     return 0;
        // }
        // let cols = cols + 1;
        // let lens = chars.iter().map(|c| c.len() + 1).collect::<Vec<_>>();
        // let mut count = 0usize;
        // let mut si = 0usize;
        // let mut r = 0usize;
        // let mut dp: Vec<Option<(usize, usize)>> = vec![None; len];
        // while r < rows {
        //     if si == 0 && r != 0 && rows >= r * 2 {
        //         let times = rows / r;
        //         r = times * r;
        //         count = count * times;
        //     } else {
        //         let fsi = si;
        //         let fcount = count;
        //         if let Some((nsi, inc)) = dp[fsi] {
        //             si = nsi;
        //             count += inc;
        //         } else {
        //             let mut c = 0usize;
        //             loop {
        //                 let w = lens[si];
        //                 let nc = c + w;
        //                 if nc > cols {
        //                     break;
        //                 }
        //                 c = nc;
        //                 si += 1;
        //                 if si >= len {
        //                     si = 0;
        //                     count += 1;
        //                 }
        //             }
        //             dp[fsi] = Some((si, count - fcount));
        //         }
        //         r += 1;
        //     }
        // }
        // count as i32
        let sen = sentence.join(" ") + " ";
        let n = sen.len();
        let mut start = 0;
        let bs = sen.as_bytes();
        for _ in 0..rows {
            start += cols as usize;
            if bs[start % n] == b' ' {
                start += 1;
            } else {
                while start > 0 && bs[(start - 1) % n] != b' ' {
                    start -= 1;
                }
            }
        }
        (start / n) as _
    }
}
// @lc code=end

#[allow(dead_code)]
pub  struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_words_typing_1() {
        assert_eq!(
            Solution::words_typing(vec![String::from("hello"), String::from("world")], 2, 8),
            1
        );
    }

    #[test]
    fn test_words_typing_2() {
        assert_eq!(
            Solution::words_typing(
                vec![String::from("a"), String::from("bcd"), String::from("e")],
                3,
                6
            ),
            2
        );
    }

    #[test]
    fn test_words_typing_3() {
        assert_eq!(
            Solution::words_typing(
                vec![
                    String::from("I"),
                    String::from("had"),
                    String::from("apple"),
                    String::from("pie")
                ],
                4,
                5
            ),
            1
        );
    }

    #[test]
    fn test_words_typing_4() {
        assert_eq!(
            Solution::words_typing(
                vec![String::from("f"), String::from("p"), String::from("a")],
                8,
                7
            ),
            10
        );
    }

    #[test]
    fn test_words_typing_5() {
        assert_eq!(Solution::words_typing(vec![String::from("a")], 2, 3), 4);
    }
}
