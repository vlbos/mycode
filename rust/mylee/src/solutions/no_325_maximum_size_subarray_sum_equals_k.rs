// 325\. Maximum Size Subarray Sum Equals k
// ========================================

// Given an array _nums_ and a target value _k_, find the maximum length of a subarray that sums to _k_.
// If there isn't one, return 0 instead.

// **Note:**
// The sum of the entire _nums_ array is guaranteed to fit within the 32-bit signed integer range.

// **Example 1:**

// **Input:** _nums_ = `[1, -1, 5, -2, 3]`, _k_ = `3`
// **Output:** 4
// **Explanation:** The subarray `[1, -1, 5, -2]` sums to 3 and is the longest.

// **Example 2:**

// **Input:** _nums_ = `[-2, -1, 2, 1]`, _k_ = `1`
// **Output:** 2 **Explanation:** The subarray `[-1, 2]` sums to 1 and is the longest.

// **Follow Up:**
// Can you do it in O(_n_) time?

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Palantir Technologies](https://leetcode.ca/tags/#Palantir%20Technologies)

// @lc code=start

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        // use std::collections::HashMap;
        // let from_to_here_sum: Vec<i32> = (0..=nums.len())
        //     .into_iter()
        //     .scan(0, |acc, curr| {
        //         *acc += if curr == 0 { 0 } else { nums[curr - 1] };
        //         Some(*acc)
        //     })
        //     .collect();
        // let first_sum_to_here_index = from_to_here_sum
        //     .iter()
        //     .enumerate()
        //     .rev()
        //     .map(|(i, sum)| (*sum, i))
        //     .collect::<HashMap<i32, usize>>();
        // from_to_here_sum
        //     .iter()
        //     .enumerate()
        //     .skip(1)
        //     .map(|(to, to_sum)| {
        //         let from_sum = to_sum - k;
        //         let map = &first_sum_to_here_index;
        //         if map.contains_key(&from_sum) && to > map[&from_sum] {
        //             Some(to - map[&from_sum])
        //         } else {
        //             None
        //         }
        //     })
        //     .fold(0usize, |acc, curr| {
        //         usize::max(
        //             acc,
        //             match curr {
        //                 Some(len) => len,
        //                 None => 0,
        //             },
        //         )
        //     }) as i32
        let mut sum = 0;
        let mut ans = 0;
        let mut m = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let i = i as i32;
            sum += num;
            if sum == k {
                ans = i + 1;
            }
            m.entry(sum).or_insert(i);
            if let Some(j) = m.get(&(sum - k)) {
                if ans < i - j {
                    ans = i - j;
                }
            }
        }
        ans
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_sub_array_len_1() {
        assert_eq!(Solution::max_sub_array_len(vec![1, -1, 5, -2, 3], 3), 4);
    }

    #[test]
    fn test_max_sub_array_len_2() {
        assert_eq!(Solution::max_sub_array_len(vec![-2, -1, 2, 1], 1), 2);
    }
}
