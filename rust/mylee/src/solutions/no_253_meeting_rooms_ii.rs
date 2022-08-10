// 253\. Meeting Rooms II
// ======================

// Given an array of meeting time intervals consisting of start and end times `[[s1,e1],[s2,e2],...]` (si < ei), find the minimum number of conference rooms required.

// **Example 1:**

// **Input:** `[[0, 30],[5, 10],[15, 20]]`
// **Output:** 2

// **Example 2:**

// **Input:** \[\[7,10\],\[2,4\]\]
// **Output:** 1

// **NOTE:** input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Atlassian](https://leetcode.ca/tags/#Atlassian) [Baidu](https://leetcode.ca/tags/#Baidu) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Booking.com](https://leetcode.ca/tags/#Booking.com) [Cisco](https://leetcode.ca/tags/#Cisco) [Citrix](https://leetcode.ca/tags/#Citrix) [Drawbridge](https://leetcode.ca/tags/#Drawbridge) [eBay](https://leetcode.ca/tags/#eBay) [Expedia](https://leetcode.ca/tags/#Expedia) [Facebook](https://leetcode.ca/tags/#Facebook) [GoDaddy](https://leetcode.ca/tags/#GoDaddy) [Goldman Sachs](https://leetcode.ca/tags/#Goldman%20Sachs) [Google](https://leetcode.ca/tags/#Google) [Lyft](https://leetcode.ca/tags/#Lyft) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Nutanix](https://leetcode.ca/tags/#Nutanix) [Oracle](https://leetcode.ca/tags/#Oracle) [Paypal](https://leetcode.ca/tags/#Paypal) [Postmates](https://leetcode.ca/tags/#Postmates) [Quora](https://leetcode.ca/tags/#Quora) [Snapchat](https://leetcode.ca/tags/#Snapchat) [Uber](https://leetcode.ca/tags/#Uber) [Visa](https://leetcode.ca/tags/#Visa) [Walmart Labs](https://leetcode.ca/tags/#Walmart%20Labs) [Yelp](https://leetcode.ca/tags/#Yelp)
// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by_key(|v| v[0]);
        let mut heap = BinaryHeap::<Reverse<i32>>::new();
        for p in &intervals {
            if let Some(Reverse(i)) = heap.peek() {
                if *i <= p[0] {
                    heap.pop();
                }
            }
            heap.push(Reverse(p[1]));
        }
        heap.len() as i32
    }
}

// @lc code=end

#[allow(dead_code)]
pub  struct  Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_meeting_rooms() {
        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]]),
            2
        );
        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![7, 10], vec![2, 4]]),
            1
        );
        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![13, 15], vec![1, 13], vec![6, 9]]),
            2
        );
    }
}
