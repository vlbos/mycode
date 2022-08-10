// 364\. Nested List Weight Sum II
// ===============================

// Given a nested list of integers, return the sum of all integers in the list weighted by their depth.

// Each element is either an integer, or a list -- whose elements may also be integers or other lists.

// Different from the [previous question](https://leetcode.com/problems/nested-list-weight-sum/)
// where weight is increasing from root to leaf, now the weight is defined from bottom up. i.e.,
//  the leaf level integers have weight 1, and the root level integers have the largest weight.

// **Example 1:**

// **Input:** \[\[1,1\],2,\[1,1\]\]
// **Output:** 8
// **Explanation:** Four 1's at depth 1, one 2 at depth 2.

// **Example 2:**

// **Input:** \[1,\[4,\[6\]\]\]
// **Output:** 17
// **Explanation:** One 1 at depth 3, one 4 at depth 2, and one 6 at depth 1; 1\*3 + 4\*2 + 6\*1 = 17.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [DiDi](https://leetcode.ca/tags/#DiDi) [Facebook](https://leetcode.ca/tags/#Facebook) [LinkedIn](https://leetcode.ca/tags/#LinkedIn)

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

// @lc code=start
// use std::collections::VecDeque;

impl Solution {
    pub fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
        // let root = &NestedInteger::List(nested_list);
        // let mut queue: VecDeque<(&NestedInteger, i32)> = VecDeque::new();
        // queue.push_back((root, 0));
        // let mut max_height = 0;
        // while let Some((node, level)) = queue.pop_back() {
        //     match node {
        //         &NestedInteger::List(ref v) => {
        //             for i in v {
        //                 queue.push_back((i, level + 1));
        //             }
        //         }
        //         _ => max_height = i32::max(max_height, level),
        //     }
        // }
        // queue.clear();
        // queue.push_back((root, 0));
        // let mut sum = 0;
        // while let Some((node, level)) = queue.pop_front() {
        //     match node {
        //         &NestedInteger::List(ref v) => {
        //             for i in v {
        //                 queue.push_back((i, level + 1));
        //             }
        //         }
        //         &NestedInteger::Int(v) => {
        //             sum += v * (max_height - level + 1);
        //         }
        //     }
        // }
        // sum
        fn dfsh(nested_list: &Vec<NestedInteger>) -> i32 {
            nested_list.into_iter().fold(0, |acc, x| {
                acc.max(match x {
                    NestedInteger::Int(_) => 1,
                    NestedInteger::List(list) => dfsh(list) + 1,
                })
            })
        }
        fn dfs(nested_list: &Vec<NestedInteger>, level: i32) -> i32 {
            nested_list.into_iter().fold(0, |acc, x| {
                acc + match x {
                    NestedInteger::Int(v) => v * level,
                    NestedInteger::List(list) => dfs(list, level - 1),
                }
            })
        }
        let height = dfsh(&nested_list);
        dfs(&nested_list, height)
    }
}
// @lc code=end

#[allow(dead_code)]
pub  struct Solution;

#[cfg(test)]
mod test {
    use super::NestedInteger::*;
    use super::*;

    #[test]
    fn test_depth_sum_inverse_1() {
        assert_eq!(
            Solution::depth_sum_inverse(vec![
                List(vec![Int(1), Int(1)]),
                Int(2),
                List(vec![Int(1), Int(1)])
            ]),
            8
        );
    }

    #[test]
    fn test_depth_sum_inverse_2() {
        assert_eq!(
            Solution::depth_sum_inverse(vec![Int(1), List(vec![Int(4), List(vec![Int(6)])])]),
            17
        );
    }
}
