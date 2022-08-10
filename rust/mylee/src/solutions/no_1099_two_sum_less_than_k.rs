// 1099\. Two Sum Less Than K
// ==========================

// Given an array `A` of integers and integer `K`, return the maximum `S` such that there exists `i < j` with `A[i] + A[j] = S` and `S < K`.
// If no `i, j` exist satisfying this equation, return -1.

// **Example 1:**

// **Input:** A = \[34,23,1,24,75,33,54,8\], K = 60
// **Output:** 58
// **Explanation:**
// We can use 34 and 24 to sum 58 which is less than 60.

// **Example 2:**

// **Input:** A = \[10,20,30\], K = 15
// **Output:** \-1
// **Explanation:**
// In this case it's not possible to get a pair sum less that 15.

// **Note:**

// 1.  `1 <= A.length <= 100`
// 2.  `1 <= A[i] <= 1000`
// 3.  `1 <= K <= 2000`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google)
 
#[allow(dead_code)] 
 pub struct Solution {}
impl Solution {
    pub fn two_sum_less_than_k(a: Vec<i32>, k: i32) -> i32 {
        let mut ans = -1;
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                let s = a[i] + a[j];
                if s < k {
                    ans = ans.max(s);
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
    fn test_two_sum_less_than_k_1() {
        assert_eq!(
            58,
            Solution::two_sum_less_than_k(vec![34, 23, 1, 24, 75, 33, 54, 8], 60)
        );
    }
    #[test]
    fn test_two_sum_less_than_k_2() {
        assert_eq!(-1, Solution::two_sum_less_than_k(vec![10, 20, 30], 15));
    }
}
