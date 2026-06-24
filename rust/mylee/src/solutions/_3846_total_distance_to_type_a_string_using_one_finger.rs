// 3846. Total Distance to Type a String Using One Finger
// ## Description
// There is a special keyboard where keys are arranged in a rectangular grid as follows.
// |q|w|e|r|t|y|u|i|o|p|
// |a|s|d|f|g|h|j|k|l| |
// |z|x|c|v|b|n|m| | | |
// You are given a string `s` that consists of lowercase English letters only.
// Return an integer denoting the total **distance** to type `s` using only one finger.
// Your finger starts on the key `'a'`.
// The **distance** between two keys at `(r1, c1)` and `(r2, c2)` is `|r1 - r2| + |c1 - c2|`.

// **Example 1:**
// **Input:** s = "hello"
// **Output:** 17
// **Explanation:**
// * Your finger starts at `'a'`, which is at `(1, 0)`.
// * Move to `'h'`, which is at `(1, 5)`. The distance is `|1 - 1| + |0 - 5| = 5`.
// * Move to `'e'`, which is at `(0, 2)`. The distance is `|1 - 0| + |5 - 2| = 4`.
// * Move to `'l'`, which is at `(1, 8)`. The distance is `|0 - 1| + |2 - 8| = 7`.
// * Move to `'l'`, which is at `(1, 8)`. The distance is `|1 - 1| + |8 - 8| = 0`.
// * Move to `'o'`, which is at `(0, 8)`. The distance is `|1 - 0| + |8 - 8| = 1`.
// * Total distance is `5 + 4 + 7 + 0 + 1 = 17`.
// **Example 2:**
// **Input:** s = "a"
// **Output:** 0
// **Explanation:**
// * Your finger starts at `'a'`, which is at `(1, 0)`.
// * Move to `'a'`, which is at `(1, 0)`. The distance is `|1 - 1| + |0 - 0| = 0`.
// * Total distance is 0.

// **Constraints:**
// * `1 \<= s.length \<= 104`
// * `s` consists of lowercase English letters only.

//  int total_distance(string s) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn total_distance(s: String) -> i32 {
        const ROWS: [i32; 26] = [
            1, 2, 2, 1, 0, 1, 1, 1, 0, 1, 1, 1, 2, 2, 0, 0, 0, 0, 1, 0, 0, 2, 0, 2, 0, 2,
        ];
        const COLS: [i32; 26] = [
            0, 4, 2, 2, 2, 3, 4, 5, 7, 6, 7, 8, 6, 5, 8, 9, 0, 3, 1, 4, 6, 3, 1, 1, 5, 0,
        ];
        let (mut i, mut j, mut ans) = (1, 0, 0);
        for b in s.bytes() {
            let k = (b - b'a') as usize;
            ans += (i - ROWS[k]).abs() + (j - COLS[k]).abs();
            (i, j) = (ROWS[k], COLS[k]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_total_distance_1() {
        assert_eq!(Solution::total_distance(String::from("hello")), 17);
    }
    #[test]
    pub fn test_total_distance_2() {
        assert_eq!(Solution::total_distance(String::from("a")), 0);
    }
}
