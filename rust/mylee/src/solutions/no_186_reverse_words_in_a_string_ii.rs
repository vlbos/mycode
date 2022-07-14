/// 186\. Reverse Words in a String II
/// ==================================
/// Given an input string , reverse the string word by word.
/// **Example:**
/// **Input:** \["t","h","e"," ","s","k","y"," ","i","s"," ","b","l","u","e"\]
/// **Output:** \["b","l","u","e"," ","i","s"," ","s","k","y"," ","t","h","e"\]
/// **Note:**
/// *   A word is defined as a sequence of non-space characters.
/// *   The input string does not contain leading or trailing spaces.
/// *   The words are always separated by a single space.
/// **Follow up:** Could you do it _in-place_ without allocating extra space?
/// ### Difficulty:
/// Medium
/// ### Lock:
/// Prime
/// ### Company:
/// [Amazon](https:///leetcode.ca/tags/#Amazon) [Facebook](https:///leetcode.ca/tags/#Facebook) [Microsoft](https:///leetcode.ca/tags/#Microsoft) [Uber](https:///leetcode.ca/tags/#Uber)

struct Solution;
/// @lc code=start

impl Solution {
    pub fn reverse_words(s: &mut Vec<char>) {
        // if s.is_empty() {
        //     return;
        // }
        // let mut i = 0;
        // while i < s.len() {
        //     let mut j = i;
        //     while j + 1 < s.len() && s[j + 1] != ' ' {
        //         j += 1;
        //     }
        //     let new_i = if j + 1 >= s.len() { j + 1 } else { j + 2 };
        //     while i < j {
        //         s.swap(i, j);
        //         i += 1;
        //         j -= 1;
        //     }
        //     i = new_i;
        // }
        // let mut j = 0;
        // let mut k = s.len() - 1;
        // while j < k {
        //     s.swap(j, k);
        //     j += 1;
        //     k -= 1;
        // }
        s.reverse();
        let n = s.len();
        let (mut i, mut j) = (0, 0);
        while j < n {
            while j < n && s[j] != ' ' {
                j += 1;
            }
            if j > i {
                s[i..j].reverse();
            }
            j += 1;
            i = j;
        }
    }
}
/// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let mut src = vec![
            't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
        ];
        let tar = vec![
            'b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e',
        ];
        Solution::reverse_words(&mut src);
        assert_eq!(src, tar);
    }
}
