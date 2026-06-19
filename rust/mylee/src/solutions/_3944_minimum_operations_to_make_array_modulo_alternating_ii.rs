// 3944. Minimum Operations to Make Array Modulo Alternating II
// ## Description
// You are given an integer array `nums` and an integer `k`.
// In one operation, you can **increase** or **decrease** any element of `nums` by 1.
// An array is called **modulo alternating** if there exist two **distinct** integers `x` and `y` (`0 \<= x, y \< k`) such that:
// * For every **even** index `i`, `nums[i] % k == x`
// * For every **odd** index `i`, `nums[i] % k == y`
// Return the **minimum** number of operations required to make `nums` **modulo alternating**.

// **Example 1:**
// **Input:** nums = [1,4,2,8], k = 3
// **Output:** 2
// **Explanation:**
// * Let's choose `x = 1` for even indices and `y = 2` for odd indices.
// * Perform the following operations:
// * Increment `nums[1] = 4` by 1, giving `nums = [1, 5, 2, 8]`.
// * Decrement `nums[2] = 2` by 1, giving `nums = [1, 5, 1, 8]`.
// * Now, for even indices, `nums[i] % k = 1`, and for odd indices, `nums[i] % k = 2`.
// * Thus, the total number of operations required is 2.
// **Example 2:**
// **Input:** nums = [1,1,1], k = 3
// **Output:** 1
// **Explanation:**
// * Incrementing `nums[1]` by 1 gives `nums = [1, 2, 1]`, which satisfies the condition with `x = 1` and `y = 2`.
// * Thus, the total number of operations required is 1.
// **Example 3:**
// **Input:** nums = [6,7,8], k = 2
// **Output:** 0
// **Explanation:**
// The array already satisfies the condition with `x = 0` and `y = 1`. Thus, no operations are required.

// **Constraints:**
// * `1 \<= nums.length \<= 105`
// * `1 \<= nums[i] \<= 109`
// * `2 \<= k \<= 105`

//  long long min_operations(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i64 {0}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_operations_1() {
        assert_eq!(2, Solution::min_operations(vec![1,4,2,8], 3));
    }
    #[test]
    pub fn test_min_operations_2() {
        assert_eq!(1, Solution::min_operations(vec![1,1,1], 3));
    }
    #[test]
    pub fn test_min_operations_3() {
        assert_eq!(0, Solution::min_operations(vec![6,7,8], 2));
    }
}
