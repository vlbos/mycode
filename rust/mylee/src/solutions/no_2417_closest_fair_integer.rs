// # [2417. Closest Fair Integer](https://leetcode.com/problems/closest-fair-integer)
// ## Description

//  You are given a  positive  integer  n .

//  We call an integer  k  fair if the number of  even  digits in  k  is equal to the number of  odd  digits in it.

//  Return  the  smallest  fair integer that is  greater than or equal  to   n .

//   Example 1:
//  Input:  n = 2
//  Output:  10
//  Explanation:  The smallest fair integer that is greater than or equal to 2 is 10.
// 10 is fair because it has an equal number of even and odd digits (one odd digit and one even digit).

//   Example 2:
//  Input:  n = 403
//  Output:  1001
//  Explanation:  The smallest fair integer that is greater than or equal to 403 is 1001.
// 1001 is fair because it has an equal number of even and odd digits (two odd digits and two even digits).

//   Constraints:

// 	  1 <= n <= 10^9

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn closest_fair(n: i32) -> i32 {
        let ns = n.to_string();
        let a = ns
            .chars()
            .filter(|c| c.to_digit(10).unwrap() % 2 == 0)
            .count() as usize;
        let k = ns.len();
        if a * 2 == k {
            n
        } else if k % 2 > 0 {
            10i32.pow(k as u32)
                + if k > 1 {
                    vec!['1'; k / 2]
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap()
                } else {
                    0
                }
        } else {
            Self::closest_fair(n + 1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_closest_fair_1() {
        assert_eq!(10, Solution::closest_fair(2));
    }
    #[test]
    pub fn test_closest_fair_2() {
        assert_eq!(1001, Solution::closest_fair(403));
    }
}
