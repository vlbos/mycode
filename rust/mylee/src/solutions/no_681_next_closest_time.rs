// 681\. Next Closest Time
// =======================

// Given a time represented in the format "HH:MM", form the next closest time by reusing the current digits. There is no limit on how many times a digit can be reused.

// You may assume the given input string is always valid. For example, "01:34", "12:09" are all valid. "1:34", "12:9" are all invalid.

// **Example 1:**

// **Input:** "19:34"
// **Output:** "19:39"
// **Explanation:** The next closest time choosing from digits **1**, **9**, **3**, **4**, is **19:39**, which occurs 5 minutes later.
// It is not **19:33**, because this occurs 23 hours and 59 minutes later.

// **Example 2:**

// **Input:** "23:59"
// **Output:** "22:22"
// **Explanation:** The next closest time choosing from digits **2**, **3**, **5**, **9**, is **22:22**.
// It may be assumed that the returned time is next day's time since it is smaller than the input time numerically.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
// use std::collections::BTreeSet;
// use std::ops::Bound::{Excluded, Unbounded};

const ZERO_CHARCODE: i32 = '0' as i32;

impl Solution {
    pub fn next_closest_time(time: String) -> String {
        // let mut digits = BTreeSet::<i32>::new();
        // let current_time = Solution::parse_time(&time);
        // for d in &current_time {
        //     digits.insert(*d);
        // }
        // let min_num = **digits.iter().take(1).collect::<Vec<_>>().first().unwrap();
        // let mut res = format!("{}{}:{}{}", min_num, min_num, min_num, min_num);
        // for i in (0..4).rev() {
        //     let d = current_time[i];
        //     let next_num_vec = digits
        //         .range((Excluded(&d), Unbounded))
        //         .take(1)
        //         .collect::<Vec<_>>();
        //     let next = next_num_vec.first();
        //     if let Some(&&next_digit) = next {
        //         let mut may_next_time = current_time.clone();
        //         may_next_time[i] = next_digit;
        //         for j in (i + 1)..4 {
        //             may_next_time[j] = min_num;
        //         }
        //         if Solution::valid_time(&may_next_time) {
        //             res = format!(
        //                 "{}{}:{}{}",
        //                 may_next_time[0], may_next_time[1], may_next_time[2], may_next_time[3]
        //             );
        //             break;
        //         }
        //     }
        // }
        // res
        let (h, m) = (
            time[..2].parse::<i32>().unwrap(),
            time[3..].parse::<i32>().unwrap(),
        );
        let cur = h * 60 + m;
        let mut ans = String::new();
        use std::collections::HashSet;
        let ts = time.chars().collect::<HashSet<_>>();
        for i in 1..1441 {
            let t = (i + cur) % 1440;
            let (h, m) = (t / 60, t % 60);
            ans = format!("{:02}:{:02}", h, m);
            if ans.chars().collect::<HashSet<_>>().is_subset(&ts) {
                break;
            }
        }
        ans
    }

    // fn parse_time(time: &str) -> Vec<i32> {
    //     let chars = time.chars().collect::<Vec<_>>();
    //     vec![chars[0], chars[1], chars[3], chars[4]]
    //         .into_iter()
    //         .map(Solution::parse_digit)
    //         .collect()
    // }

    // fn parse_digit(ch: char) -> i32 {
    //     return ch as i32 - ZERO_CHARCODE;
    // }

    // fn valid_time(time: &[i32]) -> bool {
    //     let hours = time[0] * 10 + time[1];
    //     let minutes = time[2] * 10 + time[3];
    //     hours >= 0 && hours <= 23 && minutes >= 0 && minutes <= 59
    // }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_closest_time_1() {
        assert_eq!(
            Solution::next_closest_time(String::from("19:34")),
            String::from("19:39")
        );
    }

    #[test]
    fn test_next_closest_time_2() {
        assert_eq!(
            Solution::next_closest_time(String::from("23:59")),
            String::from("22:22")
        );
    }

    #[test]
    fn test_next_closest_time_3() {
        assert_eq!(
            Solution::next_closest_time(String::from("13:55")),
            String::from("15:11")
        );
    }

    #[test]
    fn test_next_closest_time_4() {
        assert_eq!(
            Solution::next_closest_time(String::from("20:48")),
            String::from("22:00")
        );
    }
}
