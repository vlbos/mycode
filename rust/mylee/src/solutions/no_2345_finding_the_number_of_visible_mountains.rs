// # [2345. Finding the Number of Visible Mountains](https://leetcode.com/problems/finding-the-number-of-visible-mountains)

// ## Description

// You are given a 0-indexed 2D integer array peaks where peaks[i] = [xi, yi] states that mountain i has a peak at coordinates (xi, yi).
//  A mountain can be described as a right-angled isosceles triangle, with its base along the x-axis and a right angle at its peak.
//  More formally, the gradients of ascending and descending the mountain are 1 and -1 respectively.

// A mountain is considered visible if its peak does not lie within another mountain (including the border of other mountains).

// Return the number of visible mountains.

// Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2300-2399/2345.Finding%20the%20Number%20of%20Visible%20Mountains/images/ex1.png" style="width: 402px; height: 210px;" />
//
// Input: peaks = [[2,2],[6,3],[5,4]]
// Output: 2
// Explanation: The diagram above shows the mountains.
// - Mountain 0 is visible since its peak does not lie within another mountain or its sides.
// - Mountain 1 is not visible since its peak lies within the side of mountain 2.
// - Mountain 2 is visible since its peak does not lie within another mountain or its sides.
// There are 2 mountains that are visible.

// Example 2:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2300-2399/2345.Finding%20the%20Number%20of%20Visible%20Mountains/images/ex2new1.png" style="width: 300px; height: 180px;" />
//
// Input: peaks = [[1,3],[1,3]]
// Output: 0
// Explanation: The diagram above shows the mountains (they completely overlap).
// Both mountains are not visible since their peaks lie within each other.
//

// Constraints:

//
// 	1 <= peaks.length <= 105
// 	peaks[i].length == 2
// 	1 <= xi, yi <= 105
//
// int visible_mountains(vector<vector<int>>& peaks) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn visible_mountains(peaks: Vec<Vec<i32>>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        for p in &peaks {
            *cnt.entry(p.clone()).or_insert(0) += 1;
        }
        let mut a = Vec::new();
        for p in &peaks {
            if *cnt.get(p).unwrap_or(&0) > 1 {
                continue;
            }
            a.push((p[0] - p[1], p[1], p[0] + p[1]));
        }
        a.sort_by(|x, y| {
            if x.0 == y.0 {
                y.1.cmp(&x.1)
            } else {
                x.0.cmp(&y.0)
            }
        });
        let mut bound = -1;
        let mut ans = 0;
        for &(l, h, r) in &a {
            if bound < r {
                ans += 1;
                bound = r;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_visible_mountains_1() {
        assert_eq!(
            2,
            Solution::visible_mountains(vec![vec![2, 2], vec![6, 3], vec![5, 4]])
        );
    }
    #[test]
    pub fn test_visible_mountains_2() {
        assert_eq!(0, Solution::visible_mountains(vec![vec![1, 3], vec![1, 3]]));
    }
}
