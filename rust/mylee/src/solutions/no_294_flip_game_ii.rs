// 294\. Flip Game II
// ==================

// You are playing the following Flip Game with your friend:
// Given a string that contains only these two characters: `+` and `-`,
// you and your friend take turns to flip two **consecutive** `"++"` into `"--"`.
// The game ends when a person can no longer make a move and therefore the other person will be the winner.

// Write a function to determine if the starting player can guarantee a win.

// **Example:**

// **Input:** `s = "++++"`
// **Output:** true
// **Explanation:** The starting player can guarantee a win by flipping the middle `"++"` to become `"+--+"`.

// **Follow up:**
// Derive your algorithm's runtime complexity.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
// use std::collections::HashMap;

impl Solution {
    pub fn can_win(s: String) -> bool {
        // let mut memo = HashMap::new();
        // Solution::can_win_rec(&mut memo, s)
        for (i, _) in s.as_bytes().windows(2).enumerate().filter(|x| x.1 == b"++") {
            if !Self::can_win(format!(
                "{}--{}",
                &s[..i],
                if i + 2 < s.len() { &s[i + 2..] } else { "" }
            )) {
                return true;
            }
        }
        false
    }

    // fn can_win_rec(memo: &mut HashMap<String, bool>, s: String) -> bool {
    //     if let Some(res) = memo.get(&s) {
    //         return *res;
    //     }
    //     let len = s.len();
    //     for i in 1..len {
    //         let chars = s.chars().collect::<Vec<char>>();
    //         let prev = chars[i - 1];
    //         let curr = chars[i];
    //         if prev == '+' && curr == '+' {
    //             let mut new_target = chars.to_vec();
    //             new_target[i - 1] = '-';
    //             new_target[i] = '-';
    //             let new_target = new_target.into_iter().collect::<String>();
    //             if !Solution::can_win_rec(memo, new_target) {
    //                 memo.insert(s, true);
    //                 return true;
    //             }
    //         }
    //     }
    //     memo.insert(s, false);
    //     false
    // }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_can_win() {
        assert!(Solution::can_win(String::from("++++")));
    }
}
