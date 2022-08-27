// # [2323. Find Minimum Time to Finish All Jobs II](https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs-ii)

// [中文文档](/solution/2300-2399/2323.Find%20Minimum%20Time%20to%20Finish%20All%20Jobs%20II/README.md)

// ## Description

// <p>You are given two <strong>0-indexed</strong> integer arrays <code>jobs</code> and <code>workers</code> of <strong>equal</strong> length, where <code>jobs[i]</code> is the amount of time needed to complete the <code>i<sup>th</sup></code> job, and <code>workers[j]</code> is the amount of time the <code>j<sup>th</sup></code> worker can work each day.</p>

// <p>Each job should be assigned to <strong>exactly</strong> one worker, such that each worker completes <strong>exactly</strong> one job.</p>

// <p>Return <em>the <strong>minimum</strong> number of days needed to complete all the jobs after assignment.</em></p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>

// <pre>
// <strong>Input:</strong> jobs = [5,2,4], workers = [1,7,5]
// <strong>Output:</strong> 2
// <strong>Explanation:</strong>
// - Assign the 2<sup>nd</sup> worker to the 0<sup>th</sup> job. It takes them 1 day to finish the job.
// - Assign the 0<sup>th</sup> worker to the 1<sup>st</sup> job. It takes them 2 days to finish the job.
// - Assign the 1<sup>st</sup> worker to the 2<sup>nd</sup> job. It takes them 1 day to finish the job.
// It takes 2 days for all the jobs to be completed, so return 2.
// It can be proven that 2 days is the minimum number of days needed.
// </pre>

// <p><strong>Example 2:</strong></p>

// <pre>
// <strong>Input:</strong> jobs = [3,18,15,9], workers = [6,5,1,3]
// <strong>Output:</strong> 3
// <strong>Explanation:</strong>
// - Assign the 2<sup>nd</sup> worker to the 0<sup>th</sup> job. It takes them 3 days to finish the job.
// - Assign the 0<sup>th</sup> worker to the 1<sup>st</sup> job. It takes them 3 days to finish the job.
// - Assign the 1<sup>st</sup> worker to the 2<sup>nd</sup> job. It takes them 3 days to finish the job.
// - Assign the 3<sup>rd</sup> worker to the 3<sup>rd</sup> job. It takes them 3 days to finish the job.
// It takes 3 days for all the jobs to be completed, so return 3.
// It can be proven that 3 days is the minimum number of days needed.
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>n == jobs.length == workers.length</code></li>
// 	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
// 	<li><code>1 &lt;= jobs[i], workers[i] &lt;= 10<sup>5</sup></code></li>
// </ul>

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def minimumTime(self, jobs: List[int], workers: List[int]) -> int:


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
