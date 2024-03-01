// # [2158. Amount of New Area Painted Each Day](https://leetcode.com/problems/amount-of-new-area-painted-each-day)

// ## Description

// There is a long and thin painting that can be represented by a number line.
// You are given a 0-indexed 2D integer array paint of length n, where paint[i] = [starti, endi].
// This means that on the ith day you need to paint the area between starti and endi.

// Painting the same area multiple times will create an uneven painting so you only want to paint each area of the painting at most once.

// Return an integer array worklog of length n, where worklog[i] is the amount of new area that you painted on the ith day.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2158.Amount%20of%20New%20Area%20Painted%20Each%20Day/images/screenshot-2022-02-01-at-17-16-16-diagram-drawio-diagrams-net.png" style="height: 300px; width: 620px;" />
//
// Input: paint = [[1,4],[4,7],[5,8]]
// Output: [3,3,1]
// Explanation:
// On day 0, paint everything between 1 and 4.
// The amount of new area painted on day 0 is 4 - 1 = 3.
// On day 1, paint everything between 4 and 7.
// The amount of new area painted on day 1 is 7 - 4 = 3.
// On day 2, paint everything between 7 and 8.
// Everything between 5 and 7 was already painted on day 1.
// The amount of new area painted on day 2 is 8 - 7 = 1.
//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2158.Amount%20of%20New%20Area%20Painted%20Each%20Day/images/screenshot-2022-02-01-at-17-17-45-diagram-drawio-diagrams-net.png" style="width: 604px; height: 300px;" />
//
// Input: paint = [[1,4],[5,8],[4,7]]
// Output: [3,3,1]
// Explanation:
// On day 0, paint everything between 1 and 4.
// The amount of new area painted on day 0 is 4 - 1 = 3.
// On day 1, paint everything between 5 and 8.
// The amount of new area painted on day 1 is 8 - 5 = 3.
// On day 2, paint everything between 4 and 5.
// Everything between 5 and 7 was already painted on day 1.
// The amount of new area painted on day 2 is 5 - 4 = 1.
//

// Example 3:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2158.Amount%20of%20New%20Area%20Painted%20Each%20Day/images/screenshot-2022-02-01-at-17-19-49-diagram-drawio-diagrams-net.png" style="width: 423px; height: 275px;" />
//
// Input: paint = [[1,5],[2,4]]
// Output: [4,0]
// Explanation:
// On day 0, paint everything between 1 and 5.
// The amount of new area painted on day 0 is 5 - 1 = 4.
// On day 1, paint nothing because everything between 2 and 4 was already painted on day 0.
// The amount of new area painted on day 1 is 0.
//

// Constraints:

//
// 	1 <= paint.length <= 105
// 	paint[i].length == 2
// 	0 <= starti < endi <= 5 * 104
//

//  public int[] amount_painted(int[][] paint) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn amount_painted(paint: Vec<Vec<i32>>) -> Vec<i32> {
        let n = paint.len();
        let mut ans = vec![0; n];
        let (min_day, max_day) = (
            paint.iter().min_by_key(|x| x[0]).unwrap()[0],
            paint.iter().max_by_key(|x| x[1]).unwrap()[1],
        );
        let mut events = Vec::new();
        let mut running_indices = std::collections::BTreeSet::new();
        for (i, p) in paint.iter().enumerate() {
            events.push((p[0], i, 1));
            events.push((p[1], i, -1));
        }
        events.sort_by_key(|x| x.0);
        let mut i = 0;
        for day in min_day..max_day {
            while i < events.len() && events[i].0 == day {
                if events[i].2 == 1 {
                    running_indices.insert(events[i].1);
                } else {
                    running_indices.remove(&events[i].1);
                }
                i += 1;
            }
            if !running_indices.is_empty() {
                ans[*running_indices.iter().next().unwrap()] += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_amount_painted_1() {
        assert_eq!(
            vec![3, 3, 1],
            Solution::amount_painted(vec![vec![1, 4], vec![4, 7], vec![5, 8]])
        );
    }
    #[test]
    pub fn test_amount_painted_2() {
        assert_eq!(
            vec![3, 3, 1],
            Solution::amount_painted(vec![vec![1, 4], vec![5, 8], vec![4, 7]])
        );
    }
    #[test]
    pub fn test_amount_painted_3() {
        assert_eq!(
            vec![4, 0],
            Solution::amount_painted(vec![vec![1, 5], vec![2, 4]])
        );
    }
}
