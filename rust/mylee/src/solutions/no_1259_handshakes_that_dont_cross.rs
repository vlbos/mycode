// 1259\. Handshakes That Don't Cross
// ==================================

// You are given an **even** number of people `num_people` that stand around a circle and each person shakes hands with someone else, so that there are `num_people / 2` handshakes total.

// Return the number of ways these handshakes could occur such that none of the handshakes cross.

// Since this number could be very big, return the answer **mod `10^9 + 7`**

// **Example 1:**

// **Input:** num\_people = 2
// **Output:** 1

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2019/07/11/5125_example_2.png)

// **Input:** num\_people = 4
// **Output:** 2
// **Explanation:** There are two ways to do it, the first way is \[(1,2),(3,4)\] and the second one is \[(2,3),(4,1)\].

// **Example 3:**

// ![](https://assets.leetcode.com/uploads/2019/07/11/5125_example_3.png)

// **Input:** num\_people = 6
// **Output:** 5

// **Example 4:**

// **Input:** num\_people = 8
// **Output:** 14

// **Constraints:**

// *   `2 <= num_people <= 1000`
// *   `num_people % 2 == 0`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn number_of_ways(num_people: i32) -> i32 {
        let n = num_people as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in (2..=n).step_by(2) {
            for j in (2..=i).step_by(2) {
                dp[i] = (dp[i] + dp[j - 2] * dp[i - j]) % 1_000_000_007;
            }
        }
        dp[n]
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_number_of_ways_1() {
        assert_eq!(1, Solution::number_of_ways(2));
    }
    #[test]
    pub fn test_number_of_ways_2() {
        assert_eq!(2, Solution::number_of_ways(4));
    }
    #[test]
    pub fn test_number_of_ways_3() {
        assert_eq!(5, Solution::number_of_ways(6));
    }
    #[test]
    pub fn test_number_of_ways_4() {
        assert_eq!(14, Solution::number_of_ways(8));
    }
}
