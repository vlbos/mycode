// 651\. 4 Keys Keyboard
// =====================

// Imagine you have a special keyboard with the following keys:

// `Key 1: (A)`: Print one 'A' on screen.

// `Key 2: (Ctrl-A)`: Select the whole screen.

// `Key 3: (Ctrl-C)`: Copy selection to buffer.

// `Key 4: (Ctrl-V)`: Print buffer on screen appending it after what has already been printed.

// Now, you can only press the keyboard for **N** times (with the above four keys), find out the maximum numbers of 'A' you can print on screen.

// **Example 1:**

// **Input:** N = 3
// **Output:** 3
// **Explanation:**
// We can at most get 3 A's on screen by pressing following key sequence:
// A, A, A

// **Example 2:**

// **Input:** N = 7
// **Output:** 9
// **Explanation:**
// We can at most get 9 A's on screen by pressing following key sequence:
// A, A, A, Ctrl A, Ctrl C, Ctrl V, Ctrl V

// **Note:**

// 1.  1 <= N <= 50
// 2.  Answers will be in the range of 32-bit signed integer.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft)

// @star
// @lc code=start
impl Solution {
    pub fn max_a(n: i32) -> i32 {
        // let n = n as usize;
        // let mut dp = vec![0usize, 1];
        // for k in 2..=n {
        //     let mut m = usize::min_value();
        //     for x in 0..k - 1 {
        //         m = usize::max(dp[x] * (k - x - 1), m);
        //     }
        //     m = usize::max(dp[dp.len() - 1] + 1, m);
        //     dp.push(m);
        // }
        // dp[n] as i32
        let mut ans = n;
        for i in 1..n - 2 {
            ans = ans.max(Self::max_a(i) * (n - 1 - i));
        }
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_max_a_1() {
        assert_eq!(Solution::max_a(3), 3);
        assert_eq!(Solution::max_a(7), 9);
    }
}
