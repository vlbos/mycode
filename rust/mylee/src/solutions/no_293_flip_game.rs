// 293\. Flip Game
// ===============

// You are playing the following Flip Game with your friend: Given a string that contains only these two characters: `+` and `-`,
// you and your friend take turns to flip two **consecutive** `"++"` into `"--"`.
// The game ends when a person can no longer make a move and therefore the other person will be the winner.

// Write a function to compute all possible states of the string after one valid move.

// **Example:**

// **Input:** `s = "++++"`
// **Output:**
// \[
//   "--++",
//   "+--+",
//   "++--"
// \]

// **Note:** If there is no valid move, return an empty list `[]`.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
impl Solution {
    pub fn generate_possible_next_moves(s: String) -> Vec<String> {
        // let chars = s.chars().collect::<Vec<char>>();
        // let mut res = vec![];
        // for i in 1..chars.len() {
        //     let curr = chars[i];
        //     let prev = chars[i - 1];
        //     if curr == '+' && prev == '+' {
        //         let mut new_chars = chars.clone();
        //         new_chars[i - 1] = '-';
        //         new_chars[i] = '-';
        //         res.push(new_chars.into_iter().collect::<String>());
        //     }
        // }
        // res
        let bs = s.as_bytes();
        let mut ans = Vec::new();
        for (i, w) in bs.windows(2).enumerate() {
            if w == &[b'+', b'+'] {
                ans.push(format!("{}--{}", &s[..i], &s[i + 2..]));
            }
        }
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::solutions::util::test_tools::{assert_equivalent, map_to_string};

    #[test]
    pub fn test_generate_possible_next_moves() {
        let tar = map_to_string(&["--++", "+--+", "++--"]);
        assert_equivalent(
            &Solution::generate_possible_next_moves(String::from("++++")),
            &tar,
        );
    }
}
