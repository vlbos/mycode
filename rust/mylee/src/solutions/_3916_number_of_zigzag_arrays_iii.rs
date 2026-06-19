// 3916. Number of ZigZag Arrays III
// ## Description
// You are given three integers `n`, `l`, and `r`.
// A **ZigZag** array of length `n` is defined as follows:
// * Each element lies in the range `[l, r]`.
// * No **two** adjacent elements are equal.
// * No **three** consecutive elements form a **strictly increasing** or **strictly decreasing** sequence.
// Return the total number of valid **ZigZag** arrays.
// Since the answer may be large, return it **modulo** `109 + 7`.

// **Example 1:**
// **Input:** n = 3, l = 4, r = 5
// **Output:** 2
// **Explanation:**
// There are only 2 valid ZigZag arrays of length `n = 3` using values in the range `[4, 5]`:
// * `[4, 5, 4]`
// * `[5, 4, 5]`
// **Example 2:**
// **Input:** n = 3, l = 1, r = 3
// **Output:** 10
// **Explanation:**
// There are 10 valid ZigZag arrays of length `n = 3` using values in the range `[1, 3]`:
// * `[1, 2, 1]`, `[1, 3, 1]`, `[1, 3, 2]`
// * `[2, 1, 2]`, `[2, 1, 3]`, `[2, 3, 1]`, `[2, 3, 2]`
// * `[3, 1, 2]`, `[3, 1, 3]`, `[3, 2, 3]`
// All arrays meet the ZigZag conditions.

// **Constraints:**
// * `3 \<= n \<= 200`
// * `1 \<= l \< r \<= 10​​​​​​​9`

// int zig_zag_arrays(int n, int l, int r) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_zig_zag_arrays_1() {
        assert_eq!(2, Solution::zig_zag_arrays(3, 4, 5));
    }
    #[test]
    pub fn test_zig_zag_arrays_2() {
        assert_eq!(10, Solution::zig_zag_arrays(3, 1, 3));
    }
   
}
