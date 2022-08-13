// 339\. Nested List Weight Sum
// ============================

// Given a nested list of integers, return the sum of all integers in the list weighted by their depth.

// Each element is either an integer, or a list -- whose elements may also be integers or other lists.

// **Example 1:**

// **Input:** \[\[1,1\],2,\[1,1\]\]
// **Output:** 10
// **Explanation:** Four 1's at depth 2, one 2 at depth 1.

// **Example 2:**

// **Input:** \[1,\[4,\[6\]\]\]
// **Output:** 27
// **Explanation:** One 1 at depth 1, one 4 at depth 2, and one 6 at depth 3; 1 + 4\*2 + 6\*3 = 27.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Cloudera](https://leetcode.ca/tags/#Cloudera) [Facebook](https://leetcode.ca/tags/#Facebook) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Uber](https://leetcode.ca/tags/#Uber)
// @lc code=start
// use std::collections::VecDeque;
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
impl Solution {
    pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
        // let mut deque = VecDeque::<(NestedInteger, i32)>::new();
        // deque.push_back((NestedInteger::List(nested_list), 0));
        // let mut sum = 0;
        // while let Some((ni, level)) = deque.pop_front() {
        //     match ni {
        //         NestedInteger::Int(num) => sum += num * level,
        //         NestedInteger::List(list) => {
        //             list.into_iter()
        //                 .for_each(|i| deque.push_back((i, level + 1)));
        //         }
        //     }
        // }
        // sum
        pub fn dfs(nested_list: &Vec<NestedInteger>, level: i32) -> i32 {
            nested_list.into_iter().fold(0, |acc, x| {
                acc + match x {
                    NestedInteger::Int(v) => *v * level,
                    NestedInteger::List(list) => dfs(&list, level + 1),
                }
            })
        }
        dfs(&nested_list, 1)
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use NestedInteger::{Int, List};

    #[test]
    pub fn test_depth_sum_1() {
        assert_eq!(
            Solution::depth_sum(vec![
                List(vec![Int(1), Int(1)]),
                Int(2),
                List(vec![Int(1), Int(1)])
            ]),
            10
        );
    }

    #[test]
    pub fn test_depth_sum_2() {
        assert_eq!(
            Solution::depth_sum(vec![Int(1), List(vec![Int(4), List(vec![Int(6)])])]),
            27
        );
    }

    #[test]
    pub fn test_depth_sum_3() {
        assert_eq!(Solution::depth_sum(vec![Int(0)]), 0);
    }

    #[test]
    pub fn test_depth_sum_4() {
        assert_eq!(Solution::depth_sum(vec![]), 0);
    }
}
