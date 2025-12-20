// ## [3632\. Subarrays with XOR at Least K 🔒](https://leetcode.com/problems/subarrays-with-xor-at-least-k)

// ## Description

// Given an array of positive integers `nums` of length `n` and a non‑negative integer `k`.

// Return the number of **contiguous subarrays** whose bitwise XOR of all elements is **greater** than or **equal** to `k`.

// **Example 1:**

// **Input:** nums = \[3,1,2,3\], k = 2

// **Output:** 6

// **Explanation:**

// The valid subarrays with `XOR >= 2` are `[3]` at index 0, `[3, 1]` at indices 0 - 1,
// `[3, 1, 2, 3]` at indices 0 - 3, `[1, 2]` at indices 1 - 2, `[2]` at index 2, and `[3]` at index 3; there are 6 in total.

// **Example 2:**

// **Input:** nums = \[0,0,0\], k = 0

// **Output:** 6

// **Explanation:**

// Every contiguous subarray yields `XOR = 0`, which meets `k = 0`. There are 6 such subarrays in total.

// **Constraints:**

// +   `1 <= nums.length <= 105`
// +   `0 <= nums[i] <= 109`
// +   `0 <= k <= 109`

//  long long countXorSubarrays(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_xor_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        struct Trie {
            bit_length: i32,
            nodes: Vec<Vec<i32>>,
            cnts: Vec<i32>,
        }
        impl Trie {
            fn new(bit_length: i32) -> Self {
                Self {
                    bit_length,
                    nodes: vec![vec![-1; 2]],
                    cnts: vec![0],
                }
            }
            fn add(&mut self, num: i32) {
                let mut cur = 0;
                for i in (0..self.bit_length).rev() {
                    let x = (num >> i) & 1;
                    if self.nodes[cur as usize][x as usize] == -1 {
                        self.nodes[cur as usize][x as usize] = self.nodes.len() as i32;
                        self.nodes.push(vec![-1; 2]);
                        self.cnts.push(0);
                    }
                    cur = self.nodes[cur as usize][x as usize];
                    self.cnts[cur as usize] += 1;
                }
            }
            fn query(&self, prefix: i32, k: i32) -> i32 {
                let (mut cur, mut ans) = (0, 0);
                for i in (0..self.bit_length).rev() {
                    let (t, x) = ((k >> i) & 1, (prefix >> i) & 1);
                    if t == 0 {
                        let tmp = self.nodes[cur as usize][(1 ^ x) as usize];
                        if tmp != -1 {
                            ans += self.cnts[tmp as usize];
                        }
                    }

                    cur = self.nodes[cur as usize][(t ^ x) as usize];
                    if cur == -1 {
                        break;
                    }
                }
                if cur != -1 {
                    ans += self.cnts[cur as usize];
                }
                ans
            }
        }
        let mx = k.max(1).max(*nums.iter().max().unwrap());
        let mut trie = Trie::new(32 - mx.leading_zeros() as i32);
        let (mut ans, mut pre) = (0, 0);
        trie.add(pre);
        for x in nums {
            pre ^= x;
            ans += trie.query(pre, k) as i64;
            trie.add(pre);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_count_xor_subarrays_1() {
        assert_eq!(6, Solution::count_xor_subarrays(vec![3, 1, 2, 3], 2));
    }
    #[test]
    pub fn test_count_xor_subarrays_2() {
        assert_eq!(6, Solution::count_xor_subarrays(vec![0, 0, 0], 0));
    }
}
