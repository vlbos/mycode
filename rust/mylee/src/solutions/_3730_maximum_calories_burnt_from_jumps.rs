// # 3730. Maximum Calories Burnt from Jumps 🔒

// Description
// -----------

// You are given an integer array `heights` of size `n`, where `heights[i]` represents the height of the `ith` block in an exercise routine.

// You start on the ground (height 0) and **must** jump onto each block **exactly once** in any order.

// *   The **calories burned** for a jump from a block of height `a` to a block of height `b` is `(a - b)2`.
// *   The **calories burned** for the first jump from the ground to the chosen first block `heights[i]` is `(0 - heights[i])2`.

// Return the **maximum** total calories you can burn by selecting an optimal jumping sequence.

// **Note:** Once you jump onto the first block, you cannot return to the ground.

// **Example 1:**

// **Input:** heights = \[1,7,9\]

// **Output:** 181

// **Explanation:**​​​​​​​

// The optimal sequence is `[9, 1, 7]`.

// *   Initial jump from the ground to `heights[2] = 9`: `(0 - 9)2 = 81`.
// *   Next jump to `heights[0] = 1`: `(9 - 1)2 = 64`.
// *   Final jump to `heights[1] = 7`: `(1 - 7)2 = 36`.

// Total calories burned = `81 + 64 + 36 = 181`.

// **Example 2:**

// **Input:** heights = \[5,2,4\]

// **Output:** 38

// **Explanation:**

// The optimal sequence is `[5, 2, 4]`.

// *   Initial jump from the ground to `heights[0] = 5`: `(0 - 5)2 = 25`.
// *   Next jump to `heights[1] = 2`: `(5 - 2)2 = 9`.
// *   Final jump to `heights[2] = 4`: `(2 - 4)2 = 4`.

// Total calories burned = `25 + 9 + 4 = 38`.

// **Example 3:**

// **Input:** heights = \[3,3\]

// **Output:** 9

// **Explanation:**

// The optimal sequence is `[3, 3]`.

// *   Initial jump from the ground to `heights[0] = 3`: `(0 - 3)2 = 9`.
// *   Next jump to `heights[1] = 3`: `(3 - 3)2 = 0`.

// Total calories burned = `9 + 0 = 9`.

// **Constraints:**

// *   `1 <= n == heights.length <= 105`
// *   `1 <= heights[i] <= 105`

// // long long max_calories_burnt(vector<int>& heights) {

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn max_calories_burnt(mut heights: Vec<i32>) -> i64 {
        heights.sort_unstable();
        let (mut l, mut r, mut pre, mut ans) = (0, heights.len() - 1, 0, 0);
        while l < r {
            let (a, b) = (heights[l] as i64, heights[r] as i64);
            ans += (pre - b) * (pre - b) + (a - b) * (a - b);
            pre = a;
            l += 1;
            r -= 1;
        }
        let b = heights[r] as i64;
        ans + (b - pre) * (b - pre)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_max_calories_burnt_1() {
        assert_eq!(181, Solution::max_calories_burnt(vec![1, 7, 9]));
    }
    #[test]
    pub fn test_max_calories_burnt_2() {
        assert_eq!(38, Solution::max_calories_burnt(vec![5, 2, 4]));
    }
    #[test]
    pub fn test_max_calories_burnt_3() {
        assert_eq!(9, Solution::max_calories_burnt(vec![3, 3]));
    }
}
