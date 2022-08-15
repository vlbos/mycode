// 1197\. Minimum Knight Moves
// ===========================

// In an **infinite** chess board with coordinates from `-infinity` to `+infinity`, you have a **knight** at square `[0, 0]`.

// A knight has 8 possible moves it can make, as illustrated below. Each move is two squares in a cardinal direction, then one square in an orthogonal direction.

// ![](https://assets.leetcode.com/uploads/2018/10/12/knight.png)

// Return the minimum number of steps needed to move the knight to the square `[x, y]`.  It is guaranteed the answer exists.

// **Example 1:**

// **Input:** x = 2, y = 1
// **Output:** 1
// **Explanation:** \[0, 0\] → \[2, 1\]

// **Example 2:**

// **Input:** x = 5, y = 5
// **Output:** 4
// **Explanation:** \[0, 0\] → \[2, 1\] → \[4, 2\] → \[3, 4\] → \[5, 5\]

// **Constraints:**

// *   `|x| + |y| <= 300`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Oracle](https://leetcode.ca/tags/#Oracle)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        let (x, y) = (x.abs(), y.abs());
        let mut ans = 0;
        let mut q = std::collections::VecDeque::from([(0, 0)]);
        let mut visited = std::collections::HashSet::from([(0, 0)]);
        while !q.is_empty() {
            let len = q.len();
            for _ in 0..len {
                let (xx, yy) = q.pop_front().unwrap();
                if (x, y) == (xx, yy) {
                    return ans;
                }
                for i in [-1, 1] {
                    for j in [-2, 2] {
                        for (ii, jj) in [(xx + i, yy + j), (xx + j, yy + i)] {
                            if !visited.contains(&(ii, jj)) && ii >= -1 && jj >= -1 {
                                q.push_back((ii, jj));
                                visited.insert((ii, jj));
                            }
                        }
                    }
                }
            }

            ans += 1;
        }
        ans
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_knight_moves_1() {
        assert_eq!(1, Solution::min_knight_moves(2, 1));
    }
    #[test]
    pub fn test_min_knight_moves_2() {
        assert_eq!(4, Solution::min_knight_moves(5, 5));
    }
}
