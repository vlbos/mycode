// # [2297. Jump Game VIII](https://leetcode.com/problems/jump-game-viii)

// [中文文档](/solution/2200-2299/2297.Jump%20Game%20VIII/README.md)

// ## Description

// <p>You are given a <strong>0-indexed</strong> integer array <code>nums</code> of length <code>n</code>. You are initially standing at index <code>0</code>. You can jump from index <code>i</code> to index <code>j</code> where <code>i &lt; j</code> if:</p>

// <ul>
// 	<li><code>nums[i] &lt;= nums[j]</code> and <code>nums[k] &lt; nums[i]</code> for all indexes <code>k</code> in the range <code>i &lt; k &lt; j</code>, or</li>
// 	<li><code>nums[i] &gt; nums[j]</code> and <code>nums[k] &gt;= nums[i]</code> for all indexes <code>k</code> in the range <code>i &lt; k &lt; j</code>.</li>
// </ul>

// <p>You are also given an integer array <code>costs</code> of length <code>n</code> where <code>costs[i]</code> denotes the cost of jumping <strong>to</strong> index <code>i</code>.</p>

// <p>Return <em>the <strong>minimum</strong> cost to jump to the index </em><code>n - 1</code>.</p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>

// <pre>
// <strong>Input:</strong> nums = [3,2,4,4,1], costs = [3,7,6,4,2]
// <strong>Output:</strong> 8
// <strong>Explanation:</strong> You start at index 0.
// - Jump to index 2 with a cost of costs[2] = 6.
// - Jump to index 4 with a cost of costs[4] = 2.
// The total cost is 8. It can be proven that 8 is the minimum cost needed.
// Two other possible paths are from index 0 -&gt; 1 -&gt; 4 and index 0 -&gt; 2 -&gt; 3 -&gt; 4.
// These have a total cost of 9 and 12, respectively.
// </pre>

// <p><strong>Example 2:</strong></p>

// <pre>
// <strong>Input:</strong> nums = [0,1,2], costs = [1,1,1]
// <strong>Output:</strong> 2
// <strong>Explanation:</strong> Start at index 0.
// - Jump to index 1 with a cost of costs[1] = 1.
// - Jump to index 2 with a cost of costs[2] = 1.
// The total cost is 2. Note that you cannot jump directly from index 0 to index 2 because nums[0] &lt;= nums[1].
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>n == nums.length == costs.length</code></li>
// 	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
// 	<li><code>0 &lt;= nums[i], costs[i] &lt;= 10<sup>5</sup></code></li>
// </ul>
//  long long minCost(vector<int>& nums, vector<int>& costs) {

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
