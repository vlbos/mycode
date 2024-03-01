// 573\. Squirrel Simulation
// =========================

// There's a tree, a squirrel, and several nuts. Positions are represented by the cells in a 2D grid.
// Your goal is to find the **minimal** distance for the squirrel to collect all the nuts and put them under the tree one by one.
//  The squirrel can only take at most **one nut** at one time and can move in four directions - up, down, left and right, to the adjacent cell.
// The distance is represented by the number of moves.

// **Example 1:**

// **Input:**
// Height : 5
// Width : 7
// Tree position : \[2,2\]
// Squirrel : \[4,4\]
// Nuts : \[\[3,0\], \[2,5\]\]
// **Output:** 12
// **Explanation:**
// ![](https://assets.leetcode.com/uploads/2018/10/22/squirrel_simulation.png)​​​​​

// **Note:**

// 1.  All given positions won't overlap.
// 2.  The squirrel can take at most one nut at one time.
// 3.  The given positions of nuts have no order.
// 4.  Height and width are positive integers. 3 <= height \* width <= 10,000.
// 5.  The given positions contain at least one nut, only one tree and one squirrel.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Square](https://leetcode.ca/tags/#Square)
// @lc code=start
impl Solution {
    //pub fn  distance(a: &[i32], b: &[i32]) -> i32 {
    //     i32::abs(a[0] - b[0]) + i32::abs(a[1] - b[1])
    // }

    pub fn min_distance(
        _: i32,
        _: i32,
        tree: Vec<i32>,
        squirrel: Vec<i32>,
        nuts: Vec<Vec<i32>>,
    ) -> i32 {
        // if nuts.is_empty() {
        //     return 0;
        // }
        // let nuts_distance = nuts
        //     .iter()
        //     .fold(0, |acc, curr| acc + Solution::distance(&tree, &curr) * 2);
        // if tree == squirrel {
        //     return nuts_distance;
        // }
        // let min_first_distance = nuts
        //     .iter()
        //     .map(|n| Solution::distance(n, &squirrel) - Solution::distance(n, &tree))
        //     .min()
        //     .unwrap();
        // nuts_distance + min_first_distance
        let mut ans = 0;
        let mut max_diff = i32::MIN;
        for nut in &nuts {
            let dist = tree
                .iter()
                .zip(nut)
                .map(|(&a, &b)| (a - b).abs())
                .sum::<i32>();
            ans += 2 * dist;
            max_diff = max_diff.max(
                dist - squirrel
                    .iter()
                    .zip(nut)
                    .map(|(&a, &b)| (a - b).abs())
                    .sum::<i32>(),
            );
        }
        ans - max_diff
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;

    #[test]
    pub fn test_min_distance_1() {
        assert_eq!(
            Solution::min_distance(5, 7, vec![2, 2], vec![4, 4], lc_matrix![[3, 0], [2, 5]]),
            12
        );
    }
}
