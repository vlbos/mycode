// ## [3476\. Maximize Profit from Task Assignment 🔒](https://leetcode.com/problems/maximize-profit-from-task-assignment)

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// ## Description

// You are given an integer array `workers`, where `workers[i]` represents the skill level of the `ith` worker. 
// You are also given a 2D integer array `tasks`, where:

// +   `tasks[i][0]` represents the skill requirement needed to complete the task.
// +   `tasks[i][1]` represents the profit earned from completing the task.

// Each worker can complete **at most** one task, and they can only take a task if their skill level is **equal** to the task's skill requirement. 
// An **additional** worker joins today who can take up *any* task, **regardless** of the skill requirement.

// Return the **maximum** total profit that can be earned by optimally assigning the tasks to the workers.

// **Example 1:**

// **Input:** workers = \[1,2,3,4,5\], tasks = \[\[1,100\],\[2,400\],\[3,100\],\[3,400\]\]

// **Output:** 1000

// **Explanation:**

// +   Worker 0 completes task 0.
// +   Worker 1 completes task 1.
// +   Worker 2 completes task 3.
// +   The additional worker completes task 2.

// **Example 2:**

// **Input:** workers = \[10,10000,100000000\], tasks = \[\[1,100\]\]

// **Output:** 100

// **Explanation:**

// Since no worker matches the skill requirement, only the additional worker can complete task 0.

// **Example 3:**

// **Input:** workers = \[7\], tasks = \[\[3,3\],\[3,3\]\]

// **Output:** 3

// **Explanation:**

// The additional worker completes task 1. Worker 0 cannot work since no task has a skill requirement of 7.

// **Constraints:**

// +   `1 <= workers.length <= 105`
// +   `1 <= workers[i] <= 109`
// +   `1 <= tasks.length <= 105`
// +   `tasks[i].length == 2`
// +   `1 <= tasks[i][0], tasks[i][1] <= 109`

//  long long max_profit(vector<int>& workers, vector<vector<int>>& tasks) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_profit(workers: Vec<i32>, tasks: Vec<Vec<i32>>) -> i64 {
        use std::collections::{HashMap,BinaryHeap};
        let mut hb=tasks.iter().fold(HashMap::new(),|mut s,t| {s.entry(t[0]).or_insert(BinaryHeap::new()).push(t[1]);s});
        let mut ans=0;
        for w in workers{
            if let Some(v)=hb.get_mut(&w){
                    if let Some(s)=v.pop(){
                        ans+=s as i64;
                    }
            }
        }
        ans+hb.values().map(|v|*v.peek().unwrap_or(&0)).max().unwrap_or(0) as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_max_profit_1() {
        assert_eq!(
            1000,
            Solution::max_profit(
                vec![1, 2, 3, 4, 5],
                lc_matrix![[1, 100], [2, 400], [3, 100], [3, 400]]
            )
        );
    }
    #[test]
    pub fn test_max_profit_2() {
        assert_eq!(
            100,
            Solution::max_profit(vec![10, 10000, 100000000], lc_matrix![[1, 100]])
        );
    }
    #[test]
    pub fn test_max_profit_3() {
        assert_eq!(3, Solution::max_profit(vec![7], lc_matrix![[3, 3], [3, 3]]));
    }
}
