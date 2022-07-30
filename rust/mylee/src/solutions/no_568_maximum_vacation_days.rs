// 568\. Maximum Vacation Days
// ===========================

// LeetCode wants to give one of its best employees the option to travel among **N** cities to collect algorithm problems.
// But all work and no play makes Jack a dull boy, you could take vacations in some particular cities and weeks.
// Your job is to schedule the traveling to maximize the number of vacation days you could take, but there are certain rules and restrictions you need to follow.

// **Rules and restrictions:**

// 1.  You can only travel among **N** cities, represented by indexes from 0 to N-1. Initially, you are in the city indexed 0 on **Monday**.
// 2.  The cities are connected by flights. The flights are represented as a **N\*N** matrix (not necessary symmetrical),
// called **flights** representing the airline status from the city i to the city j.
// If there is no flight from the city i to the city j, **flights\[i\]\[j\] = 0**; Otherwise, **flights\[i\]\[j\] = 1**.
// Also, **flights\[i\]\[i\] = 0** for all i.
// 3.  You totally have **K** weeks (**each week has 7 days**) to travel.
// You can only take flights at most once **per day** and can only take flights on each week's **Monday** morning.
// Since flight time is so short, we don't consider the impact of flight time.
// 4.  For each city, you can only have restricted vacation days in different weeks, given an **N\*K** matrix called **days** representing this relationship.
// For the value of **days\[i\]\[j\]**, it represents the maximum days you could take vacation in the city **i** in the week **j**.

// You're given the **flights** matrix and **days** matrix, and you need to output the maximum vacation days you could take during **K** weeks.

// **Example 1:**

// **Input:**flights = \[\[0,1,1\],\[1,0,1\],\[1,1,0\]\], days = \[\[1,3,1\],\[6,0,3\],\[3,3,3\]\]
// **Output:** 12
// **Explanation:**
// Ans = 6 + 3 + 3 = 12.
// One of the best strategies is:
// 1st week : fly from city 0 to city 1 on Monday, and play 6 days and work 1 day.
// (Although you start at city 0, we could also fly to and start at other cities since it is Monday.)
// 2nd week : fly from city 1 to city 2 on Monday, and play 3 days and work 4 days.
// 3rd week : stay at city 2, and play 3 days and work 4 days.

// **Example 2:**

// **Input:**flights = \[\[0,0,0\],\[0,0,0\],\[0,0,0\]\], days = \[\[1,1,1\],\[7,7,7\],\[7,7,7\]\]
// **Output:** 3
// **Explanation:**
// Ans = 1 + 1 + 1 = 3.
// Since there is no flights enable you to move to another city, you have to stay at city 0 for the whole 3 weeks.
// For each week, you only have one day to play and six days to work.
// So the maximum number of vacation days is 3.

// **Example 3:**

// **Input:**flights = \[\[0,1,1\],\[1,0,1\],\[1,1,0\]\], days = \[\[7,0,0\],\[0,7,0\],\[0,0,7\]\]
// **Output:** 21
// **Explanation:**
// Ans = 7 + 7 + 7 = 21
// One of the best strategies is:
// 1st week : stay at city 0, and play 7 days.
// 2nd week : fly from city 0 to city 1 on Monday, and play 7 days.
// 3rd week : fly from city 1 to city 2 on Monday, and play 7 days.

// **Note:**

// 1.  **N and K** are positive integers, which are in the range of \[1, 100\].
// 2.  In the matrix **flights**, all the values are integers in the range of \[0, 1\].
// 3.  In the matrix **days**, all the values are integers in the range \[0, 7\].
// 4.  You could stay at a city beyond the number of vacation days, but you should **work** on the extra days, which won't be counted as vacation days.
// 5.  If you fly from the city A to the city B and take the vacation on that day, the deduction towards vacation days will count towards the vacation days of city B in that week.
// 6.  We don't consider the impact of flight hours towards the calculation of vacation days.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
impl Solution {
    pub fn max_vacation_days(flights: Vec<Vec<i32>>, days: Vec<Vec<i32>>) -> i32 {
        // let clen = days.len();
        // let dlen = if clen == 0 { 0 } else { days[0].len() };
        // if dlen + clen == 0 {
        //     return 0;
        // }
        // let mut dict = vec![vec![]; clen as usize];
        // for j in 0..clen {
        //     dict[j].push(j);
        //     for i in 0..clen {
        //         if flights[i][j] == 1 {
        //             dict[j].push(i);
        //         }
        //     }
        // }
        // let mut dp = vec![vec![i32::min_value(); clen]; 2];
        // dp[1][0] = 0;
        // for i in 0..dlen {
        //     let mod_i = i % 2;
        //     let mod_prev_i = (i + 1) % 2;
        //     for j in 0..clen {
        //         let mut j_v = i32::min_value();
        //         for &k in &dict[j] {
        //             j_v = i32::max(dp[mod_prev_i][k] + days[j][i], j_v);
        //         }
        //         dp[mod_i][j] = j_v;
        //     }
        // }
        // let last_mod_i = (dlen - 1) % 2;
        // dp[last_mod_i].iter().cloned().max().unwrap()
        let (n, k) = (flights.len(), days[0].len());
        let mut dp = vec![i32::MIN; n];
        dp[0] = 0;
        for j in 0..k {
            let mut new_dp = vec![i32::MIN; n];
            for i in 0..n {
                for p in 0..n {
                    if i == p || flights[p][i] > 0 {
                        new_dp[i] = new_dp[i].max(dp[p] + days[i][j]);
                    }
                }
            }
            dp = new_dp;
        }
        dp.into_iter().max().unwrap()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_vacation_days_1() {
        let flights = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
        let days = vec![vec![1, 3, 1], vec![6, 0, 3], vec![3, 3, 3]];
        assert_eq!(Solution::max_vacation_days(flights, days), 12);
    }

    #[test]
    fn test_max_vacation_days_2() {
        let flights = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let days = vec![vec![1, 1, 1], vec![7, 7, 7], vec![7, 7, 7]];
        assert_eq!(Solution::max_vacation_days(flights, days), 3);
    }

    #[test]
    fn test_max_vacation_days_3() {
        let flights = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
        let days = vec![vec![7, 0, 0], vec![0, 7, 0], vec![0, 0, 7]];
        assert_eq!(Solution::max_vacation_days(flights, days), 21);
    }
}
