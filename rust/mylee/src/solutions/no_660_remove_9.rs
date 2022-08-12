// 660\. Remove 9
// ==============

// Start from integer 1, remove any integer that contains 9 such as 9, 19, 29...

// So now, you will have a new integer sequence: 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, ...

// Given a positive integer `n`, you need to return the n-th integer after removing. Note that 1 will be the first integer.

// **Example 1:**

// **Input:** 9
// **Output:** 10

// **Hint**: n will not exceed `9 x 10^8`.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google) [Houzz](https://leetcode.ca/tags/#Houzz)

// @lc code=start
impl Solution {
    pub fn   new_integer(mut n: i32) -> i32 {
        // let mut res = 0;
        // let mut times = 0;
        // while n > 0 {
        //     res += (n % 9) * i32::pow(10, times);
        //     n /= 9;
        //     times += 1;
        // }
        // res
        let mut ans = String::new();
        while n > 0 {
            ans = (n % 9).to_string() + ans.as_str();
            n /= 9;
        }
        ans.parse::<i32>().unwrap()
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_new_integer_1() {
        assert_eq!(Solution::new_integer(9), 10);
    }

    #[test]
   pub fn  test_new_integer_2() {
        assert_eq!(Solution::new_integer(82), 101);
    }
}
