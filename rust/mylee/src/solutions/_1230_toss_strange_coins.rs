// 1230\. Toss Strange Coins
// =========================

// You have some coins.  The `i`\-th coin has a probability `prob[i]` of facing heads when tossed.

// Return the probability that the number of coins facing heads equals `target` if you toss every coin exactly once.

// **Example 1:**

// **Input:** prob = \[0.4\], target = 1
// **Output:** 0.40000

// **Example 2:**

// **Input:** prob = \[0.5,0.5,0.5,0.5,0.5\], target = 0
// **Output:** 0.03125

// **Constraints:**

// *   `1 <= prob.length <= 1000`
// *   `0 <= prob[i] <= 1`
// *   `0 <= target` `<= prob.length`
// *   Answers will be accepted as correct if they are within `10^-5` of the correct answer.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Twitch](https://leetcode.ca/tags/#Twitch)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn probability_of_heads(prob: Vec<f64>, target: i32) -> f64 {
        let n = 1 << target as u32;
        let mut ans = 0.0;
        for i in 0..n {
            if (i as i32).count_ones() != target as u32 {
                continue;
            }
            ans += prob.iter().enumerate().fold(1.0, |a, (j, &p)| {
                a * if i & (1 << j) == 0 { 1.0 - p } else { p }
            });
        }
        ans
    }
}

// impl Solution {
//     pub fn probability_of_heads(prob: Vec<f64>, target: i32) -> f64 {
//        let mut ans = vec![0.0; target as usize + 1];
//         ans[0] = 1.0;
//         for (i, &p) in prob.iter().enumerate() {
//             for j in (1..=(i + 1).min(target as usize)).rev() {
//                 ans[j] = ans[j] * (1.0 - p) + ans[j - 1] * p;
//             }
//             ans[0] = ans[0] * (1.0 - p);
//         }
//         ans[target as usize]
//     }
// }
#[cfg(test)]
mod test {
    use super::*;
    // [0.2,0.8,0,0.3,0.5]
    // 3
    // 输出：
    // 0.00000
    // 预期结果：
    // 0.182
    #[test]
    pub fn test_probability_of_heads_1() {
        assert_eq!(0.4, Solution::probability_of_heads(vec![0.4], 1));
    }
    #[test]
    pub fn test_probability_of_heads_2() {
        assert_eq!(
            0.03125,
            Solution::probability_of_heads(vec![0.5, 0.5, 0.5, 0.5, 0.5], 0)
        );
    }
}
