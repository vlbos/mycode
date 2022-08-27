// 1908\. Game of Nim[](https://leetcode.ca/2021-07-29-1908-Game-of-Nim/#1908-game-of-nim)
// =======================================================================================

// Level[](https://leetcode.ca/2021-07-29-1908-Game-of-Nim/#level)
// ---------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-07-29-1908-Game-of-Nim/#description)
// ---------------------------------------------------------------------------

// Alice and Bob take turns playing a game with **Alice starting first**.

// In this game, there are n piles of stones. On each player’s turn, the player should remove any **positive** number of stones from a non-empty pile **of his or her choice**. The first player who cannot make a move loses, and the other player wins.

// Given an integer array `piles`, where `piles[i]` is the number of stones in the `i-th` pile, return _`true` if Alice wins, or `false` if Bob wins_.

// Both Alice and Bob play **optimally**.

// **Example 1:**

// **Input:** piles = \[1\]

// **Output:** true

// _\*Explanation:8_ There is only one possible scenario:

// *   On the first turn, Alice removes one stone from the first pile. piles = \[0\].
// *   On the second turn, there are no stones left for Bob to remove. Alice wins.

// **Example 2:**

// **Input:** piles = \[1,1\]

// **Output:** false

// **Explanation:** It can be proven that Bob will always win. One possible scenario is:

// *   On the first turn, Alice removes one stone from the first pile. piles = \[0,1\].
// *   On the second turn, Bob removes one stone from the second pile. piles = \[0,0\].
// *   On the third turn, there are no stones left for Alice to remove. Bob wins.

// **Example 3:**

// **Input:** piles = \[1,2,3\]

// **Output:** false

// **Explanation:** It can be proven that Bob will always win. One possible scenario is:

// *   On the first turn, Alice removes three stones from the third pile. piles = \[1,2,0\].
// *   On the second turn, Bob removes one stone from the second pile. piles = \[1,1,0\].
// *   On the third turn, Alice removes one stone from the first pile. piles = \[0,1,0\].
// *   On the fourth turn, Bob removes one stone from the second pile. piles = \[0,0,0\].
// *   On the fifth turn, there are no stones left for Alice to remove. Bob wins.

// **Constraints:**

// *   `n == piles.length`
// *   `1 <= n <= 7`
// *   `1 <= piles[i] <= 7`

// **Follow-up:** Could you find a linear time solution?

// Solution[](https://leetcode.ca/2021-07-29-1908-Game-of-Nim/#solution)
// ---------------------------------------------------------------------

// For the game of Nim, where Alice starts first, Alice can win if and only if the bitwise xor results of all the elements in `piles` is not 0.

//     class Solution {
//         public boolean nimGame(int[] piles) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(
        words:Vec<String>,
    ) -> String {
       String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
   
    #[test]
    pub fn test_longest_word_1() {
        assert_eq!("kiran".to_string(),Solution::longest_word(
            ["k","ki","kir","kira", "kiran"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_2() {
          assert_eq!("apple".to_string(),Solution::longest_word(
           ["a", "banana", "app", "appl", "ap", "apply", "apple"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_3() {
          assert_eq!(String::new(),Solution::longest_word(
            ["abc", "bc", "ab", "qwe"].into_iter().map(String::from).collect::<Vec<String>>(),
        ));
    }
}
