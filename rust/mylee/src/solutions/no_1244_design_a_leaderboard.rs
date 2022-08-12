/*
Design a Leaderboard class, which has 3 functions:

    addScore(playerId, score): Update the leaderboard by adding score to the given player's score. If there is no player with such id in the leaderboard, add him to the leaderboard with the given score.
    top(K): Return the score sum of the top K players.
    reset(playerId): Reset the score of the player with the given id to 0 (in other words erase it from the leaderboard). It is guaranteed that the player was added to the leaderboard before calling this function.

Initially, the leaderboard is empty.


Example 1:
Input:
["Leaderboard","addScore","addScore","addScore","addScore","addScore","top","reset","reset","addScore","top"]
[[],[1,73],[2,56],[3,39],[4,51],[5,4],[1],[1],[2],[2,51],[3]]
Output:
[null,null,null,null,null,null,73,null,null,null,141]

Explanation:
Leaderboard leaderboard = new Leaderboard ();
leaderboard.addScore(1,73);   // leaderboard = [[1,73]];
leaderboard.addScore(2,56);   // leaderboard = [[1,73],[2,56]];
leaderboard.addScore(3,39);   // leaderboard = [[1,73],[2,56],[3,39]];
leaderboard.addScore(4,51);   // leaderboard = [[1,73],[2,56],[3,39],[4,51]];
leaderboard.addScore(5,4);    // leaderboard = [[1,73],[2,56],[3,39],[4,51],[5,4]];
leaderboard.top(1);           // returns 73;
leaderboard.reset(1);         // leaderboard = [[2,56],[3,39],[4,51],[5,4]];
leaderboard.reset(2);         // leaderboard = [[3,39],[4,51],[5,4]];
leaderboard.addScore(2,51);   // leaderboard = [[2,51],[3,39],[4,51],[5,4]];
leaderboard.top(3);           // returns 141 = 51 + 51 + 39;


Constraints:
    1 <= playerId, K <= 10000
    It's guaranteed that K is less than or equal to the current number of players.
    1 <= score <= 100
    There will be at most 1000 function calls.


*/
#[allow(dead_code)]
pub struct Solution {}
pub struct Leaderboard {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Leaderboard {
   pub fn  new() -> Self {
        Self {}
    }

   pub fn  add_score(&self, player_id: i32, score: i32) {}

   pub fn  top(&self, k: i32) -> i32 {
        0
    }

   pub fn  reset(&self, player_id: i32) {}
}

/**
 * Your Leaderboard object will be instantiated and called as such:
 * let obj = Leaderboard::new();
 * obj.add_score(playerId, score);
 * let ret_2: i32 = obj.top(K);
 * obj.reset(playerId);
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_leader_board_1() {
        let k = 10;
        let player_id = 0;
        let score = 0;
        let obj = Leaderboard::new();
        obj.add_score(player_id, score);
        let _ret_2: i32 = obj.top(k);
        obj.reset(player_id);
    }
}
