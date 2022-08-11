// 634\. Find the Derangement of An Array
// ======================================

// In combinatorial mathematics, a derangement is a permutation of the elements of a set, such that no element appears in its original position.

// There's originally an array consisting of `n` integers from 1 to `n` in ascending order, you need to find the number of derangement it can generate.

// Also, since the answer may be very large, you should return the output mod 109 \+ 7.

// **Example 1:**

// **Input:** 3
// **Output:** 2
// **Explanation:** The original array is \[1,2,3\]. The two derangements are \[2,3,1\] and \[3,1,2\].

// **Note:**
// `n` is in the range of \[1, 106\].

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [IXL](https://leetcode.ca/tags/#IXL)

// @lc code=start
// const MOD: i64 = (1e9 as i64) + 7;

impl Solution {
    pub fn find_derangement(n: i32) -> i32 {
        // let n = n as i64;
        // if n <= 1 {
        //     return 0;
        // }
        // let mut mul = 1i64;
        // let mut sum = 0i64;
        // for i in (0..=n).rev() {
        //     sum = (sum + MOD + mul * (if i % 2 == 0 { 1 } else { -1 })) % MOD;
        //     mul = (mul * i) % MOD;
        // }
        // sum as i32
        if n < 2 {
            return 0;
        }
        let n = n as i64;
        let mut ans = 1;
        for i in 1..=n {
            ans = (i * ans + if i % 2 == 0 { 1 } else { -1 }) % 1_000_000_007;
        }
        ans as _
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_derangement_1() {
        assert_eq!(Solution::find_derangement(3), 2);
    }

    #[test]
    fn test_find_derangement_2() {
        assert_eq!(Solution::find_derangement(1), 0);
    }

    #[test]
    fn test_find_derangement_3() {
        assert_eq!(Solution::find_derangement(4), 9);
    }
}
