// 3958. Minimum Cost to Split into Ones II
// ## Description
// You are given an integer `n`.
// In one operation, you may split an integer `x` into two positive integers `a` and `b` such that `a + b = x`.
// The cost of this operation is `a \* b`.
// Return the **minimum** total cost required to split the integer `n` into `n` ones.

// **Example 1:**
// **Input:** n = 3
// **Output:** 3
// **Explanation:**
// One optimal set of operations is:
// |`x`|`a`|`b`|`a + b`|`a \* b`|Cost|
// |3|1|2|3|2|2|
// |2|1|1|2|1|1|
// Thus, the minimum total cost is `2 + 1 = 3`.
// **Example 2:**
// **Input:** n = 4
// **Output:** 6
// **Explanation:​​​​​​​**
// One optimal set of operations is:
// |`x`|`a`|`b`|`a + b`|`a \* b`|Cost|
// |4|2|2|4|4|4|
// |2|1|1|2|1|1|
// Thus, the minimum total cost is `4 + 1 + 1 = 6`.

// **Constraints:**
// * `1 \<= n \<= 5 \* 107`

// long long min_cost(int n) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_cost(n: i64) -> i64 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_min_cost_1() {
        assert_eq!(3, Solution::min_cost(3));
    }
    #[test]
    pub fn test_min_cost_2() {
        assert_eq!(6, Solution::min_cost(4));
    }
}
