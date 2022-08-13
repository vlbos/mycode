// 1067\. Digit Count in Range
// ===========================

// Given an integer `d` between `0` and `9`, and two positive integers `low` and `high` as lower and upper bounds, respectively.
// Return the number of times that `d` occurs as a digit in all integers between `low` and `high`, including the bounds `low` and `high`.

// **Example 1:**

// **Input:** d = 1, low = 1, high = 13
// **Output:** 6
// **Explanation:**
// The digit `d=1` occurs `6` times in `1,10,11,12,13`. Note that the digit `d=1` occurs twice in the number `11`.

// **Example 2:**

// **Input:** d = 3, low = 100, high = 250
// **Output:** 35
// **Explanation:**
// The digit `d=3` occurs `35` times in `103,113,123,130,131,...,238,239,243`.

// **Note:**

// 1.  `0 <= d <= 9`
// 2.  `1 <= low <= high <= 2×10^8`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn digits_count(d: i32, low: i32, high: i32) -> i32 {
        let count = |n: i32| {
            let mut cnt = 0;
            let mut i = 1;
            while n / i != 0 {
                let k = n / i;
                let mut high = k / 10;
                if d == 0 {
                    if high != 0 {
                        high -= 1;
                    } else {
                        break;
                    }
                }
                cnt += high * i;
                let cur = k % 10;
                if cur > d {
                    cnt += i;
                } else if cur == d {
                    cnt += n - k * i + 1;
                }
                i *= 10;
            }
            cnt
        };
        count(high) - count(low - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_digits_count_1() {
        assert_eq!(6, Solution::digits_count(1, 1, 13));
    }
    #[test]
    pub fn test_digits_count_2() {
        assert_eq!(35, Solution::digits_count(3, 100, 250));
    }
}
