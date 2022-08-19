// 1272\. Remove Interval
// ======================

// Given a **sorted** list of disjoint `intervals`, each interval `intervals[i] = [a, b]` represents the set of real numbers `x` such that `a <= x < b`.

// We remove the intersections between any interval in `intervals` and the interval `toBeRemoved`.

// Return a **sorted** list of `intervals` after all such removals.

// **Example 1:**

// **Input:** intervals = \[\[0,2\],\[3,4\],\[5,7\]\], toBeRemoved = \[1,6\]
// **Output:** \[\[0,1\],\[6,7\]\]

// **Example 2:**

// **Input:** intervals = \[\[0,5\]\], toBeRemoved = \[2,3\]
// **Output:** \[\[0,2\],\[3,5\]\]

// **Constraints:**

// *   `1 <= intervals.length <= 10^4`
// *   `-10^9 <= intervals[i][0] < intervals[i][1] <= 10^9`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn remove_interval(intervals: Vec<Vec<i32>>, to_be_removed: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut to_be_removed = to_be_removed;
        for interval in &intervals {
            if to_be_removed[0] <= interval[0] && interval[1] <= to_be_removed[1] {
                continue;
            }
            if interval[0] > to_be_removed[1] || interval[1] < to_be_removed[0] {
                ans.push(interval.clone());
                continue;
            }
            if interval[0] <= to_be_removed[0] && to_be_removed[1] <= interval[1] {
                ans.push(vec![interval[0], to_be_removed[0]]);
                ans.push(vec![to_be_removed[1], interval[1]]);
                continue;
            }
            if interval[1] >= to_be_removed[0] && interval[1] <= to_be_removed[1] {
                ans.push(vec![interval[0], to_be_removed[0]]);
                to_be_removed[0] = interval[1];
            } else if interval[0] >= to_be_removed[0] && interval[0] <= to_be_removed[1] {
                ans.push(vec![to_be_removed[1], interval[1]]);
                to_be_removed[1] = interval[0];
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_remove_interval_1() {
        assert_eq!(
            vec![vec![0, 1], vec![6, 7]],
            Solution::remove_interval(vec![vec![0, 2], vec![3, 4], vec![5, 7]], vec![1, 6])
        );
    }

    #[test]
    pub fn test_remove_interval_2() {
        assert_eq!(
            vec![vec![0, 2], vec![3, 5]],
            Solution::remove_interval(vec![vec![0, 5]], vec![2, 3])
        );
    }
}
