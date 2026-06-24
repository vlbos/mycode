// 3837. Delayed Count of Equal Elements
// ## Description
// You are given an integer array `nums` of length `n` and an integer `k`.
// For each index `i`, define the **delayed count** as the number of indices `j` such that:
// * `i + k \< j \<= n - 1`, and
// * `nums[j] == nums[i]`
// Return an array `ans` where `ans[i]` is the **delayed count** of index `i`.

// **Example 1:**
// **Input:** nums = [1,2,1,1], k = 1
// **Output:** [2,0,0,0]
// **Explanation:**
// |`i`|`nums[i]`|possible `j`|`nums[j]`|satisfying
// `nums[j] == nums[i]`|`ans[i]`|
// |0|1|[2, 3]|[1, 1]|[2, 3]|2|
// |1|2|[3]|[1]|[]|0|
// |2|1|[]|[]|[]|0|
// |3|1|[]|[]|[]|0|
// Thus, `ans = [2, 0, 0, 0]`​​​​​​​.
// **Example 2:**
// **Input:** nums = [3,1,3,1], k = 0
// **Output:** [1,1,0,0]
// **Explanation:**
// |`i`|`nums[i]`|possible `j`|`nums[j]`|satisfying
// `nums[j] == nums[i]`|`ans[i]`|
// |0|3|[1, 2, 3]|[1, 3, 1]|[2]|1|
// |1|1|[2, 3]|[3, 1]|[3]|1|
// |2|3|[3]|[1]|[]|0|
// |3|1|[]|[]|[]|0|
// Thus, `ans = [1, 1, 0, 0]`​​​​​​​.

// **Constraints:**
// * `1 \<= n == nums.length \<= 105`
// * `1 \<= nums[i] \<= 105`
// * `0 \<= k \<= n - 1`

// vector<int> delayed_count(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn delayed_count(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut cnt = std::collections::HashMap::new();
        for (i, &x) in nums.iter().enumerate() {
            cnt.entry(x).or_insert(vec![]).push(i);
        }
        let mut ans = vec![0; nums.len()];
        for (i, &x) in nums.iter().enumerate() {
            let v = cnt.get(&x).unwrap();
            let j = v.partition_point(|&y| y <= i + k);
            if j < v.len() {
                ans[i] = (v.len() - j) as i32;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_delayed_count_1() {
        assert_eq!(
            vec![2, 0, 0, 0],
            Solution::delayed_count(vec![1, 2, 1, 1], 1)
        );
    }
    #[test]
    pub fn test_delayed_count_2() {
        assert_eq!(
            vec![1, 1, 0, 0],
            Solution::delayed_count(vec![3, 1, 3, 1], 0)
        );
    }
}
