// 252\. Meeting Rooms
// ===================

// Given an array of meeting time intervals consisting of start and end times `[[s1,e1],[s2,e2],...]` (si < ei), determine if a person could attend all meetings.

// **Example 1:**

// **Input:** `[[0,30],[5,10],[15,20]]`
// **Output:** false

// **Example 2:**

// **Input:** \[\[7,10\],\[2,4\]\]
// **Output:** true

// **NOTE:** input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Twitter](https://leetcode.ca/tags/#Twitter)
// @lc code=start

impl Solution {
    pub fn   can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        // intervals.sort_by_key(|v| v[0]);
        // if intervals.is_empty() {
        //     return true;
        // }
        // let mut all_period = intervals[0].clone();
        // for i in 1..intervals.len() {
        //     let period = &intervals[i];
        //     if period[0] < all_period[1] {
        //         return false;
        //     } else {
        //         all_period[1] = period[1];
        //     }
        // }
        // true
        let mut intervals = intervals;
        intervals.sort_by_key(|x| x[0]);
        if intervals.len() < 2 {
            return true;
        }
        for v in intervals.windows(2) {
            if v[1][0] < v[0][1] {
                return false;
            }
        }
        true
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
   pub fn  test_can_attend_meetings_1() {
        let src = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert!(!Solution::can_attend_meetings(src));
    }

    #[test]
   pub fn  test_can_attend_meetings_2() {
        let src = vec![vec![7, 10], vec![2, 4]];
        assert!(Solution::can_attend_meetings(src));
    }
}
