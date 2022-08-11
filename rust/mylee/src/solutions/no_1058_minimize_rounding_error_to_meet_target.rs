// 1058\. Minimize Rounding Error to Meet Target
// =============================================

// Given an array of prices `[p1,p2...,pn]` and a `target`, round each price `pi` to `Roundi(pi)` so that the rounded array `[Round1(p1),Round2(p2)...,Roundn(pn)]` sums to the given `target`.
// Each operation `Roundi(pi)` could be either `Floor(pi)` or `Ceil(pi)`.

// Return the string `"-1"` if the rounded array is impossible to sum to `target`. Otherwise, return the smallest rounding error, which is defined as Σ |Roundi(pi) - (pi)| for i from 1 to n ,
// as a string with three places after the decimal.

// **Example 1:**

// **Input:** prices = \["0.700","2.800","4.900"\], target = 8
// **Output:** "1.000"
// **Explanation:**
// Use Floor, Ceil and Ceil operations to get (0.7 - 0) + (3 - 2.8) + (5 - 4.9) = 0.7 + 0.2 + 0.1 = 1.0 .

// **Example 2:**

// **Input:** prices = \["1.500","2.500","3.500"\], target = 10
// **Output:** "-1"
// **Explanation:**
// It is impossible to meet the target.

// **Note:**

// 1.  `1 <= prices.length <= 500`.
// 2.  Each string of prices `prices[i]` represents a real number which is between 0 and 1000 and has exactly 3 decimal places.
// 3.  `target` is between 0 and 1000000.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Airbnb](https://leetcode.ca/tags/#Airbnb)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn minimize_error(prices: Vec<String>, target: i32) -> String {
        let prices: Vec<f32> = prices
            .into_iter()
            .map(|x| x.parse::<f32>().unwrap())
            .collect();
        let lg = prices.iter().map(|x| x.ceil() as i32).sum::<i32>();
        let sm = prices.iter().map(|&x| x as i32).sum::<i32>();
        if target < sm || target > lg {
            return "-1".to_string();
        }
        let m = (lg - target) as usize;
        let mut prices: Vec<f32> = prices
            .into_iter()
            .filter(|&x| x - (x as i32 as f32) > 0.0001)
            .collect();
        prices.sort_by_key(|&x| ((x - (x as i32 as f32)) * 1000.0) as i32);
        let ans = prices[..m]
            .iter()
            .map(|&x| x - (x as i32 as f32))
            .sum::<f32>()
            + prices[m..]
                .iter()
                .map(|&x| (x.ceil() as i32 as f32) - x)
                .sum::<f32>();
        format!("{:.3}", ans)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minimize_error_1() {
        assert_eq!(
            String::from("1.000"),
            Solution::minimize_error(
                ["0.700", "2.800", "4.900"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                8
            )
        );
    }
    #[test]
    fn test_minimize_error_2() {
        assert_eq!(
            String::from("-1"),
            Solution::minimize_error(
                ["1.500", "2.500", "3.500"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                10
            )
        );
    }
}
