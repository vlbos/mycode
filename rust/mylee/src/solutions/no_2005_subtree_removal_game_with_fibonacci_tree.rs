// # [2005. Subtree Removal Game with Fibonacci Tree](https://leetcode.com/problems/subtree-removal-game-with-fibonacci-tree)

// ## Description

// A Fibonacci tree is a binary tree created using the order function order(n):

//
// 	order(0) is the empty tree.
// 	order(1) is a binary tree with only one node.
// 	order(n) is a binary tree that consists of a root node with the left subtree as order(n - 2) and the right subtree as order(n - 1).
//

// Alice and Bob are playing a game with a Fibonacci tree with Alice starting first. On each turn, a player selects a node and removes that node and its subtree. The player that is forced to delete root loses.

// Given the integer n, return true if Alice wins the game or false if Bob wins, assuming both players play optimally.

// A subtree of a binary tree  is a tree that consists of a node in tree and all of this node's descendants. The tree  could also be considered as a subtree of itself.

//
// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2000-2099/2005.Subtree%20Removal%20Game%20with%20Fibonacci%20Tree/images/image-20210914173520-3.png" style="width: 200px; height: 184px;" />

//
// Input: n = 3
// Output: true
// Explanation:
// Alice takes the node 1 in the right subtree.
// Bob takes either the 1 in the left subtree or the 2 in the right subtree.
// Alice takes whichever node Bob doesn't take.
// Bob is forced to take the root node 3, so Bob will lose.
// Return true because Alice wins.
//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2000-2099/2005.Subtree%20Removal%20Game%20with%20Fibonacci%20Tree/images/image-20210914173634-4.png" style="width: 75px; height: 75px;" />

//
// Input: n = 1
// Output: false
// Explanation:
// Alice is forced to take the root node 1, so Alice will lose.
// Return false because Alice loses.
//

// Example 3:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2000-2099/2005.Subtree%20Removal%20Game%20with%20Fibonacci%20Tree/images/image-20210914173425-1.png" style="width: 100px; height: 106px;" />

//
// Input: n = 2
// Output: true
// Explanation:
// Alice takes the node 1.
// Bob is forced to take the root node 2, so Bob will lose.
// Return true because Alice wins.
//

//
// Constraints:

//
// 	1 <= n <= 100
//

//  bool find_game_winner(int n) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_game_winner(n: i32) -> bool {
        n % 6 != 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_find_game_winner_1() {
        assert!(Solution::find_game_winner(3));
    }
    #[test]
    pub fn test_find_game_winner_2() {
        assert!(!Solution::find_game_winner(1));
    }
    #[test]
    pub fn test_find_game_winner_3() {
        assert!(Solution::find_game_winner(2));
    }
}
