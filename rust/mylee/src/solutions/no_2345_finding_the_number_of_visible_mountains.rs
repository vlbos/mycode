// # [2345. Finding the Number of Visible Mountains](https://leetcode.com/problems/finding-the-number-of-visible-mountains)

// [中文文档](/solution/2300-2399/2345.Finding%20the%20Number%20of%20Visible%20Mountains/README.md)

// ## Description

// <p>You are given a <strong>0-indexed</strong> 2D integer array <code>peaks</code> where <code>peaks[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> states that mountain <code>i</code> has a peak at coordinates <code>(x<sub>i</sub>, y<sub>i</sub>)</code>. A mountain can be described as a right-angled isosceles triangle, with its base along the <code>x</code>-axis and a right angle at its peak. More formally, the <strong>gradients</strong> of ascending and descending the mountain are <code>1</code> and <code>-1</code> respectively.</p>

// <p>A mountain is considered <strong>visible</strong> if its peak does not lie within another mountain (including the border of other mountains).</p>

// <p>Return <em>the number of visible mountains</em>.</p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2300-2399/2345.Finding%20the%20Number%20of%20Visible%20Mountains/images/ex1.png" style="width: 402px; height: 210px;" />
// <pre>
// <strong>Input:</strong> peaks = [[2,2],[6,3],[5,4]]
// <strong>Output:</strong> 2
// <strong>Explanation:</strong> The diagram above shows the mountains.
// - Mountain 0 is visible since its peak does not lie within another mountain or its sides.
// - Mountain 1 is not visible since its peak lies within the side of mountain 2.
// - Mountain 2 is visible since its peak does not lie within another mountain or its sides.
// There are 2 mountains that are visible.</pre>

// <p><strong>Example 2:</strong></p>
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2300-2399/2345.Finding%20the%20Number%20of%20Visible%20Mountains/images/ex2new1.png" style="width: 300px; height: 180px;" />
// <pre>
// <strong>Input:</strong> peaks = [[1,3],[1,3]]
// <strong>Output:</strong> 0
// <strong>Explanation:</strong> The diagram above shows the mountains (they completely overlap).
// Both mountains are not visible since their peaks lie within each other.
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>1 &lt;= peaks.length &lt;= 10<sup>5</sup></code></li>
// 	<li><code>peaks[i].length == 2</code></li>
// 	<li><code>1 &lt;= x<sub>i</sub>, y<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
// </ul>
// int visibleMountains(vector<vector<int>>& peaks) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(
        words:Vec<String>,
    ) -> String {
       String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
   
    #[test]
    pub fn test_longest_word_1() {
        assert_eq!("kiran".to_string(),Solution::longest_word(
            ["k","ki","kir","kira", "kiran"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_2() {
          assert_eq!("apple".to_string(),Solution::longest_word(
           ["a", "banana", "app", "appl", "ap", "apply", "apple"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_3() {
          assert_eq!(String::new(),Solution::longest_word(
            ["abc", "bc", "ab", "qwe"].into_iter().map(String::from).collect::<Vec<String>>(),
        ));
    }
}
