// [3385\. Minimum Time to Break Locks II 🔒](https://leetcode.com/problems/minimum-time-to-break-locks-ii)
// ========================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// Description
// -----------

// Bob is stuck in a dungeon and must break `n` locks, each requiring some amount of **energy** to break. The required energy for each lock is stored in an array called `strength` where `strength[i]` indicates the energy needed to break the `ith` lock.

// To break a lock, Bob uses a sword with the following characteristics:

// *   The initial energy of the sword is 0.
// *   The initial factor `X` by which the energy of the sword increases is 1.
// *   Every minute, the energy of the sword increases by the current factor `X`.
// *   To break the `ith` lock, the energy of the sword must reach at least `strength[i]`.
// *   After breaking a lock, the energy of the sword resets to 0, and the factor `X` increases by 1.

// Your task is to determine the **minimum** time in minutes required for Bob to break all `n` locks and escape the dungeon.

// Return the **minimum** time required for Bob to break all `n` locks.

// **Example 1:**

// **Input:** strength = \[3,4,1\]

// **Output:** 4

// **Explanation:**

// | Time| Energy| X| Action| Updated X|
// | 0| 0| 1| Nothing| 1|
// | 1| 1| 1| Break 3rd Lock| 2|
// | 2| 2| 2| Nothing| 2|
// | 3| 4| 2| Break 2nd Lock| 3|
// | 4| 3| 3| Break 1st Lock| 3|

// The locks cannot be broken in less than 4 minutes; thus, the answer is 4.

// **Example 2:**

// **Input:** strength = \[2,5,4\]

// **Output:** 6

// **Explanation:**

//| Time| Energy| X| Action| Updated X|
//| 0| 0| 1| Nothing| 1|
//| 1| 1| 1| Nothing| 1|
//| 2| 2| 1| Break 1st Lock| 2|
//| 3| 2| 2| Nothing| 2|
//| 4| 4| 2| Break 3rd Lock| 3|
//| 5| 3| 3| Nothing| 3|
//| 6| 6| 3| Break 2nd Lock| 4

// The locks cannot be broken in less than 6 minutes; thus, the answer is 6.

// **Constraints:**

// *   `n == strength.length`
// *   `1 <= n <= 80`
// *   `1 <= strength[i] <= 106`
// *   `n == strength.length`


    // public int find_minimum_time(int[] strength) {

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn find_minimum_time(strength: Vec<i32>) -> i32 {
       0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_find_minimum_time_1() {
        assert_eq!(4, Solution::find_minimum_time(vec![3,4,1]));
    }
    #[test]
    pub fn test_find_minimum_time_2() {
        assert_eq!(6, Solution::find_minimum_time(vec![2,5,4]));
    }
}
