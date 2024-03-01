// 1244\. Design A Leaderboard
// ===========================

// Design a Leaderboard class, which has 3 functions:

// 1.  `addScore(playerId, score)`: Update the leaderboard by adding `score` to the given player's score.
// If there is no player with such id in the leaderboard, add him to the leaderboard with the given `score`.
// 2.  `top(K)`: Return the score sum of the top `K` players.
// 3.  `reset(playerId)`: Reset the score of the player with the given id to 0.
// It is guaranteed that the player was added to the leaderboard before calling this function.

// Initially, the leaderboard is empty.

// **Example 1:**

// **Input:**
// \["Leaderboard","addScore","addScore","addScore","addScore","addScore","top","reset","reset","addScore","top"\]
// \[\[\],\[1,73\],\[2,56\],\[3,39\],\[4,51\],\[5,4\],\[1\],\[1\],\[2\],\[2,51\],\[3\]\]
// **Output:**
// \[null,null,null,null,null,null,73,null,null,null,141\]

// **Explanation:**
// Leaderboard leaderboard = new Leaderboard ();
// leaderboard.addScore(1,73);   // leaderboard = \[\[1,73\]\];
// leaderboard.addScore(2,56);   // leaderboard = \[\[1,73\],\[2,56\]\];
// leaderboard.addScore(3,39);   // leaderboard = \[\[1,73\],\[2,56\],\[3,39\]\];
// leaderboard.addScore(4,51);   // leaderboard = \[\[1,73\],\[2,56\],\[3,39\],\[4,51\]\];
// leaderboard.addScore(5,4);    // leaderboard = \[\[1,73\],\[2,56\],\[3,39\],\[4,51\],\[5,4\]\];
// leaderboard.top(1);           // returns 73;
// leaderboard.reset(1);         // leaderboard = \[\[2,56\],\[3,39\],\[4,51\],\[5,4\]\];
// leaderboard.reset(2);         // leaderboard = \[\[3,39\],\[4,51\],\[5,4\]\];
// leaderboard.addScore(2,51);   // leaderboard = \[\[2,51\],\[3,39\],\[4,51\],\[5,4\]\];
// leaderboard.top(3);           // returns 141 = 51 + 51 + 39;

// **Constraints:**

// *   `1 <= playerId, K <= 10000`
// *   It's guaranteed that `K` is less than or equal to the current number of players.
// *   `1 <= score <= 100`
// *   There will be at most `1000` function calls.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Wayfair](https://leetcode.ca/tags/#Wayfair)
use std::collections::{BTreeMap, HashMap};
#[allow(dead_code)]
pub struct Leaderboard {
    p2s: HashMap<i32, i32>,
    s2c: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Leaderboard {
    pub fn new() -> Self {
        Self {
            p2s: HashMap::new(),
            s2c: BTreeMap::new(),
        }
    }

    pub fn add_score(&mut self, player_id: i32, score: i32) {
        if let Some(v) = self.p2s.get_mut(&player_id) {
            self.s2c.entry(*v).and_modify(|c| *c -= 1);
            *v += score;
            *self.s2c.entry(*v).or_insert(0) += 1;
        } else {
            self.p2s.insert(player_id, score);
            *self.s2c.entry(score).or_insert(0) += 1;
        }
    }

    pub fn top(&self, k: i32) -> i32 {
        let mut k = k;
        let mut ans = 0;
        for (&s, &c) in self.s2c.iter().rev() {
            if c < k {
                ans += c * s;
                k -= c;
            } else {
                ans += k * s;
                break;
            }
        }
        ans
    }

    pub fn reset(&mut self, player_id: i32) {
        let v = *self.p2s.get(&player_id).unwrap();
        self.s2c.entry(v).and_modify(|c| *c -= 1);
        self.p2s.remove(&player_id);
    }
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
    pub fn test_leader_board_1() {
        let mut obj = Leaderboard::new();
        for (p, s) in [(1, 73), (2, 56), (3, 39), (4, 51), (5, 4)] {
            obj.add_score(p, s);
        }

        assert_eq!(73, obj.top(1));
        for p in [1, 2] {
            obj.reset(p);
        }
        obj.add_score(2, 51);
        assert_eq!(141, obj.top(3));
    }
}
