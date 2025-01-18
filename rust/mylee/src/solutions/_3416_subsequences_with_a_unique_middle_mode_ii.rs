// [3416\. Subsequences with a Unique Middle Mode II 🔒](https://leetcode.com/problems/subsequences-with-a-unique-middle-mode-ii)
// ==============================================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// Description
// -----------

// Given an integer array `nums`, find the number of subsequences of size 5 of `nums` with a **unique middle mode**.

// Since the answer may be very large, return it **modulo** `109 + 7`.

// A **mode** of a sequence of numbers is defined as the element that appears the **maximum** number of times in the sequence.

// A sequence of numbers contains a **unique mode** if it has only one mode.

// A sequence of numbers `seq` of size 5 contains a **unique middle mode** if the _middle element_ (`seq[2]`) is a **unique mode**.

// **Example 1:**

// **Input:** nums = \[1,1,1,1,1,1\]

// **Output:** 6

// **Explanation:**

// `[1, 1, 1, 1, 1]` is the only subsequence of size 5 that can be formed from this list, and it has a unique middle mode of 1.

// **Example 2:**

// **Input:** nums = \[1,2,2,3,3,4\]

// **Output:** 4

// **Explanation:**

// `[1, 2, 2, 3, 4]` and `[1, 2, 3, 3, 4]` have unique middle modes because the number at index 2 has the greatest frequency in the subsequence. `[1, 2, 2, 3, 3]` does not have a unique middle mode because 2 and 3 both appear twice in the subsequence.

// **Example 3:**

// **Input:** nums = \[0,1,2,3,4,5,6,7,8\]

// **Output:** 0

// **Explanation:**

// There does not exist a subsequence of length 5 with a unique middle mode.

// **Constraints:**

// *   `5 <= nums.length <= 105`
// *   `-109 <= nums[i] <= 109`




#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn subsequences_with_middle_mode(mut nums: Vec<i32>) -> i32 {
       0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_subsequences_with_middle_mode_1() {
        assert_eq!(6, Solution::subsequences_with_middle_mode(vec![1,1,1,1,1,1]));
    }
    #[test]
    pub fn test_subsequences_with_middle_mode_2() {
        assert_eq!(4, Solution::subsequences_with_middle_mode(vec![1,2,2,3,3,4]));
    }
}




// int (vector<int>& nums) {
// Time:  O(n)
// Space: O(n)

// freq table, prefix sum, combinatorics
class Solution {
public:
    int subsequencesWithMiddleMode(vector<int>& nums) {
        typedef __int128 int128_t;

        const auto& nC2 = [](int128_t x) {
            return x * (x - 1) / 2;
        };

        static const int MOD = 1e9 + 7;
        int128_t result = 0;
        unordered_map<int, int128_t> left, right;
        for (const auto& x : nums) {
            ++right[x];
        }

        int128_t left_x_sq = 0; // sum(left[x]^2 for x != v)
        int128_t right_x_sq = 0; // sum(right[x]^2f or x != v)
        int128_t left_x_right_x = 0;  // sum(left[x]*right[x] for x != v)
        int128_t left_x_sq_right_x = 0;  // sum(left[x]^2*right[x] for x != v)
        int128_t left_x_right_x_sq = 0;  //sum(left[x]*right[x]^2 for x != v)
        for (const auto& [_, v] : right) {
            right_x_sq += v * v;
        }
        for (int i = 0; i < size(nums); ++i) {
            const int v = nums[i];
            left_x_sq -= left[v] * left[v];
            right_x_sq -= right[v]* right[v];
            left_x_right_x -= left[v] * right[v];
            left_x_sq_right_x -= left[v] * left[v] * right[v];
            left_x_right_x_sq -= left[v] * right[v] * right[v];
            --right[v];

            const int l = i;
            const int r = size(nums) - (i + 1);
            // all possibles
            result += nC2(l) * nC2(r);
            // only mid is a
            result -= nC2(l - left[v]) * nC2(r - right[v]);
            // bb/a/ac
            // sum((left[x]*(left[x]-1)//2)*right[v]*((r-right[v])-right[x]) for x != v)
            result -= ((left_x_sq - (l - left[v])) * (r - right[v]) - (left_x_sq_right_x - left_x_right_x)) * right[v] / 2;
            // ac/a/bb
            // sum(left[v]*((l-left[v])-left[x])*(right[x]*(right[x]-1)//2) for x != v)
            result -= ((right_x_sq - (r - right[v])) * (l - left[v]) - (left_x_right_x_sq - left_x_right_x) ) *left[v] / 2;
            // ab/a/bc
            // sum(left[v]*left[x]*right[x]*((r-right[v])-right[x]) for x != v)
            result -= left[v] * left_x_right_x * (r - right[v]) - left[v] * left_x_right_x_sq;
            // bc/a/ab
            // sum(left[x]*((l-left[v])-left[x])*right[v]*right[x] for x != v)
            result -= right[v] * left_x_right_x * (l - left[v]) - right[v] * left_x_sq_right_x;
            // bb/a/ab
            // sum((left[x]*(left[x]-1)//2)*right[v]*right[x] for x != v)
            result -= right[v] * (left_x_sq_right_x - left_x_right_x) / 2;
            // ab/a/bb
            // sum((right[x]*(right[x]-1)//2)*left[v]*left[x] for x != v)
            result -= left[v] * (left_x_right_x_sq - left_x_right_x) / 2;

            ++left[v];
            left_x_sq += left[v] * left[v];
            right_x_sq += right[v] * right[v];
            left_x_right_x += left[v] * right[v];
            left_x_sq_right_x += left[v] * left[v] * right[v];
            left_x_right_x_sq += left[v] * right[v] * right[v];
        }

        return result % MOD;
    }
};

