// 163. Missing Ranges
// Given a sorted integer array nums, where the range of elements are in the inclusive range [lower, upper], return its missing ranges.

// Example:

// Input: nums = [0, 1, 3, 50, 75], lower = 0 and upper = 99,
// Output: ["2", "4->49", "51->74", "76->99"]
// Difficulty:
// Medium
// Lock:
// Prime
// Company:
// Amazon Facebook Google Oracle
struct Solution;
// @lc code=start

impl Solution {
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
        let mut new_nums = vec![lower as i64 - 1];
        new_nums.extend(nums.into_iter().map(|i| i as i64).collect::<Vec<i64>>());
        new_nums.push(upper as i64 + 1);
        let mut res = vec![];
        for i in 0..new_nums.len() - 1 {
            let curr = new_nums[i];
            let next = new_nums[i + 1];
            match next - curr {
                sub if sub == 2 => {
                    res.push(format!("{}", curr + 1));
                }
                sub if sub > 2 => {
                    res.push(format!("{}->{}", curr + 1, next - 1));
                }
                _ => {}
            }
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::util::test_tools::map_to_string;

    #[test]
    fn test_find_missing_ranges() {
        let nums = vec![0, 1, 3, 50, 75];
        let res = map_to_string(&["2", "4->49", "51->74", "76->99"]);
        assert_eq!(Solution::find_missing_ranges(nums, 0, 99), res);
    }
}
