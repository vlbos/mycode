// 1167\. Minimum Cost to Connect Sticks
// =====================================

// You have some `sticks` with positive integer lengths.

// You can connect any two sticks of lengths `X` and `Y` into one stick by paying a cost of `X + Y`.  You perform this action until there is one stick remaining.

// Return the minimum cost of connecting all the given `sticks` into one stick in this way.

// **Example 1:**

// **Input:** sticks = \[2,4,3\]
// **Output:** 14

// **Example 2:**

// **Input:** sticks = \[1,8,3,5\]
// **Output:** 30

// **Constraints:**

// *   `1 <= sticks.length <= 10^4`
// *   `1 <= sticks[i] <= 10^4`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        let mut q: std::collections::BinaryHeap<Reverse<i32>> =
            sticks.into_iter().map(|x| Reverse(x)).collect();
        let mut ans = 0;
        while q.len() > 1 {
            let v = q.pop().unwrap().0 + q.pop().unwrap().0;
            ans += v;
            q.push(Reverse(v));
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_connect_sticks_1() {
        assert_eq!(14, Solution::connect_sticks(vec![2, 4, 3]));
    }
    #[test]
    pub fn test_connect_sticks_2() {
        assert_eq!(30, Solution::connect_sticks(vec![1, 8, 3, 5]));
    }
}
