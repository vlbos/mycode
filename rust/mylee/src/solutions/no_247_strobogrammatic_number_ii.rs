// 247\. Strobogrammatic Number II
// ===============================

// A strobogrammatic number is a number that looks the same when rotated 180 degrees (looked at upside down).

// Find all strobogrammatic numbers that are of length = n.

// **Example:**

// **Input:**  n = 2
// **Output:** `["11","69","88","96"]`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Cisco](https://leetcode.ca/tags/#Cisco) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)
#[allow(dead_code)]
pub struct Solution;
// @lc code=start
// const CENTER_CHAR_SIZE: usize = 3;
// const PAIR_CHAR_SIZE: usize = 5;
// const CENTER_CHARS: [char; CENTER_CHAR_SIZE] = ['1', '8', '0'];
// const PAIR_CHARS: [(char, char); PAIR_CHAR_SIZE] =
//     [('0', '0'), ('6', '9'), ('9', '6'), ('1', '1'), ('8', '8')];

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn   find_strobogrammatic(n: i32) -> Vec<String> {
        // if n <= 0 {
        //     return vec![];
        // }
        // let mid = (n / 2) as usize;
        // let size = if n & 1 == 1 {
        //     usize::pow(PAIR_CHAR_SIZE, mid as u32) * CENTER_CHAR_SIZE
        // } else {
        //     usize::pow(PAIR_CHAR_SIZE, mid as u32)
        // };
        // let mut result: Vec<Vec<char>> = vec![vec![' '; n as usize]; size];
        // for i in 0..size {
        //     let mut k = i;
        //     for j in 0..mid {
        //         let kind = k % PAIR_CHAR_SIZE;
        //         result[i][j] = PAIR_CHARS[kind].0;
        //         result[i][n as usize - 1 - j] = PAIR_CHARS[kind].1;
        //         k /= PAIR_CHAR_SIZE;
        //     }
        //     if n & 1 == 1 {
        //         let kind = k % CENTER_CHAR_SIZE;
        //         result[i][mid] = CENTER_CHARS[kind];
        //     }
        // }
        // result
        //     .iter()
        //     .filter(|s| s.len() == 1 || s[0] != '0')
        //     .map(|s| s.iter().collect::<String>())
        //     .collect()
        if n == 0 {
            return Vec::new();
        }
        let mut ans = if n % 2 == 0 {
            vec![String::new()]
        } else {
            ["0", "1", "8"].into_iter().map(String::from).collect()
        };
        for i in 0..n / 2 {
            let mut tmp = Vec::new();
            for a in &ans {
                for (j, d) in [['0', '0'], ['1', '1'], ['8', '8'], ['6', '9'], ['9', '6']]
                    .iter()
                    .enumerate()
                {
                    if j == 0 && i == n / 2 - 1 {
                        continue;
                    }
                    tmp.push(format!("{}{}{}", d[0], a, d[1]));
                }
            }

            ans = tmp;
        }
        ans
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::util::test_tools::{assert_equivalent, map_to_string};
    #[test]
   pub fn  test_find_strobogrammatic() {
        let tar = Solution::find_strobogrammatic(2);
        let src = map_to_string(&["11", "69", "88", "96"]);
        assert_equivalent(&tar, &src);
    }
}
