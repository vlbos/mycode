// 1215\. Stepping Numbers
// =======================

// A _Stepping Number_ is an integer such that all of its adjacent digits have an absolute difference of exactly `1`. For example, `321` is a Stepping Number while `421` is not.

// Given two integers `low` and `high`, find and return a **sorted** list of all the Stepping Numbers in the range `[low, high]` inclusive.

// **Example 1:**

// **Input:** low = 0, high = 21
// **Output:** \[0,1,2,3,4,5,6,7,8,9,10,12,21\]

// **Constraints:**

// *   `0 <= low <= high <= 2 * 10^9`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Epic Systems](https://leetcode.ca/tags/#Epic%20Systems)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn count_stepping_numbers(low: i32, high: i32) -> Vec<i32> {
        let mut q: std::collections::VecDeque<i32> = (1..10).collect();
        let mut ans = Vec::new();
        if low == 0 {
            ans.push(0);
        }
        while let Some(v) = q.pop_front() {
            if v >= low && v <= high {
                ans.push(v);
            }
            if v > high / 10 {
                continue;
            }
            let last = v % 10;
            if last > 0 {
                q.push_back(v * 10 + last - 1);
            }
            if last < 9 {
                q.push_back(v * 10 + last + 1);
            }
        }
        ans
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_stepping_numbers_1() {
        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 21],
            Solution::count_stepping_numbers(0, 21)
        );
    }
}
