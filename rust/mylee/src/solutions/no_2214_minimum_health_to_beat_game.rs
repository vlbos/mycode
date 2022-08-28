// [2214\. Minimum Health to Beat Game (Medium)](https://leetcode.com/problems/minimum-health-to-beat-game/)[](https://leetcode.ca/2022-04-16-2214-Minimum-Health-to-Beat-Game/#2214-minimum-health-to-beat-game-medium)
// =====================================================================================================================================================================================================================

// You are playing a game that has `n` levels numbered from `0` to `n - 1`. You are given a **0-indexed** integer array `damage` where `damage[i]` is the amount of health you will lose to complete the `ith` level.

// You are also given an integer `armor`. You may use your armor ability **at most once** during the game on **any** level which will protect you from **at most** `armor` damage.

// You must complete the levels in order and your health must be **greater than** `0` at all times to beat the game.

// Return _the **minimum** health you need to start with to beat the game._

// **Example 1:**

// **Input:** damage = \[2,7,4,3\], armor = 4
// **Output:** 13
// **Explanation:** One optimal way to beat the game starting at 13 health is:
// On round 1, take 2 damage. You have 13 - 2 = 11 health.
// On round 2, take 7 damage. You have 11 - 7 = 4 health.
// On round 3, use your armor to protect you from 4 damage. You have 4 - 0 = 4 health.
// On round 4, take 3 damage. You have 4 - 3 = 1 health.
// Note that 13 is the minimum health you need to start with to beat the game.

// **Example 2:**

// **Input:** damage = \[2,5,3,4\], armor = 7
// **Output:** 10
// **Explanation:** One optimal way to beat the game starting at 10 health is:
// On round 1, take 2 damage. You have 10 - 2 = 8 health.
// On round 2, use your armor to protect you from 5 damage. You have 8 - 0 = 8 health.
// On round 3, take 3 damage. You have 8 - 3 = 5 health.
// On round 4, take 4 damage. You have 5 - 4 = 1 health.
// Note that 10 is the minimum health you need to start with to beat the game.

// **Example 3:**

// **Input:** damage = \[3,3,3\], armor = 0
// **Output:** 10
// **Explanation:** One optimal way to beat the game starting at 10 health is:
// On round 1, take 3 damage. You have 10 - 3 = 7 health.
// On round 2, take 3 damage. You have 7 - 3 = 4 health.
// On round 3, take 3 damage. You have 4 - 3 = 1 health.
// Note that you did not use your armor ability.

// **Constraints:**

// *   `n == damage.length`
// *   `1 <= n <= 105`
// *   `0 <= damage[i] <= 105`
// *   `0 <= armor <= 105`

// **Related Topics**:
// [Array](https://leetcode.com/tag/array/), [Greedy](https://leetcode.com/tag/greedy/), [Prefix Sum](https://leetcode.com/tag/prefix-sum/)

// **Similar Questions**:

// *   [Dungeon Game (Hard)](https://leetcode.com/problems/dungeon-game/)
// *   [Eliminate Maximum Number of Monsters (Medium)](https://leetcode.com/problems/eliminate-maximum-number-of-monsters/)

// Solution 1. DP[](https://leetcode.ca/2022-04-16-2214-Minimum-Health-to-Beat-Game/#solution-1-dp)
// ------------------------------------------------------------------------------------------------

// Let `yes` be the min health needed if we do use armor once.

// Let `no` be the min health needed if we don’t use armor.

// Both of them are initialized as `1`.

// From `A[N-1]` to `A[0]`, at each step, we get `newYes` and `newNo` values:

// *   `newNo` is simply `no + A[i]`.
// *   `newYes` has two options. It’s the min of these two:
//     *   Use the armor in the current level. The min health needed is `no + max(0, A[i] - armor)`.
//     *   Don’t use the armor in the current level but a previous level. The min health needed is `yes + A[i]`.

// In the end, the result is `min(yes, no)`.

//     // OJ: https://leetcode.com/problems/minimum-health-to-beat-game/
//     // Time: O()
//     // Space: O()
//     class Solution {
//     public:
//         long long minimumHealth(vector<int>& A, int armor) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_word_1() {
        assert_eq!(
            "kiran".to_string(),
            Solution::longest_word(
                ["k", "ki", "kir", "kira", "kiran"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_2() {
        assert_eq!(
            "apple".to_string(),
            Solution::longest_word(
                ["a", "banana", "app", "appl", "ap", "apply", "apple"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_3() {
        assert_eq!(
            String::new(),
            Solution::longest_word(
                ["abc", "bc", "ab", "qwe"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
}
