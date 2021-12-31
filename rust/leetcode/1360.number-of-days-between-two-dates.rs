/*
 * @lc app=leetcode id=1360 lang=rust
 *
 * [1360] Number of Days Between Two Dates
 */

// @lc code=start
impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let mut d1 = date1
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut d2 = date2
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if  date1>date2{
            let d3 = d1;
            d1 = d2;
            d2 = d3;
        }

        let isLeap = |i: i32| -> bool { i % 400 == 0 || i % 4 == 0 && i % 100 != 0 };
        let daysOfMonth = |i: i32, y: i32| -> i32 {
            if i == 2 {
                if isLeap(y) {
                    29
                } else {
                    28
                }
            } else if i == 4 || i == 6 || i == 9 || i == 11 {
                30
            } else {
                31
            }
        };
        let mut days = 0;
        for i in d1[0]..d2[0] {
            days += 365;
            if isLeap(i) {
                days += 1;
            }
        }
        days -= d1[2];
        for i in 1..d1[1] {
            days -= daysOfMonth(i, d1[0]);
        }
        for i in 1..d2[1] {
            days += daysOfMonth(i, d2[0]);
        }
        days += d2[2];

        days
    }
}
// @lc code=end
