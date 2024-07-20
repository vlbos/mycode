// # [3032. Count Numbers With Unique Digits II](https://leetcode.com/problems/count-numbers-with-unique-digits-ii)

// <!-- tags:Hash Table,Math,Dynamic Programming -->

// ## Description

// Given two positive integers a and b,
// return the count of numbers having unique digits in the range [a, b] (inclusive).

//
// Example 1:

// Input: a = 1, b = 20
// Output: 19
// Explanation: All the numbers in the range [1, 20] have unique digits except 11.
// Hence, the answer is 19.

// Example 2:

// Input: a = 9, b = 19
// Output: 10
// Explanation: All the numbers in the range [9, 19] have unique digits except 11.
// Hence, the answer is 10.

// Example 3:

// Input: a = 80, b = 120
// Output: 27
// Explanation: There are 41 numbers in the range [80, 120], 27 of which have unique digits.

//
// Constraints:

// 	1  <= a  <= b  <= 1000

// int number_count(int a, int b)
#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn number_count(a: i32, b: i32) -> i32 {
        fn dfs(pos: usize, mask: i32, limit: bool, num: &[u8], f: &mut Vec<Vec<i32>>) -> i32 {
            if pos >= num.len() {
                return if mask > 0 { 1 } else { 0 };
            }
            if !limit && f[pos][mask as usize] != -1 {
                return f[pos][mask as usize];
            }
            let up = if limit { (num[pos] - b'0') as usize } else { 9 };
            let mut ans = 0;
            for i in 0..=up {
                if mask >> i & 1 == 1 {
                    continue;
                }
                let nxt = if mask == 0 && i == 0 {
                    0
                } else {
                    mask | 1 << i
                };
                ans += dfs(pos + 1, nxt, limit && i == up, num, f);
            }
            if !limit {
                f[pos][mask as usize] = ans;
            }
            ans
        }
        let num = (a - 1).to_string();
        let mut f = vec![vec![-1; 1 << 10]; num.len()];
        let x = dfs(0, 0, true, num.as_bytes(), &mut f);
        let num = b.to_string();
        let mut f = vec![vec![-1; 1 << 10]; num.len()];
        let y = dfs(0, 0, true, num.as_bytes(), &mut f);
        y - x
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_number_count_1() {
        assert_eq!(19, Solution::number_count(1, 20));
    }
    #[test]
    pub fn test_number_count_2() {
        assert_eq!(10, Solution::number_count(9, 19));
    }
    #[test]
    pub fn test_number_count_3() {
        assert_eq!(27, Solution::number_count(80, 120));
    }
}
