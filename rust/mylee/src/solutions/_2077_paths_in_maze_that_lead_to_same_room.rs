// # [2077. Paths in Maze That Lead to Same Room](https://leetcode.com/problems/paths-in-maze-that-lead-to-same-room)

// ## Description

// A maze consists of n rooms numbered from 1 to n, and some rooms are connected by corridors.
// You are given a 2D integer array corridors where corridors[i] = [room1i, room2i] indicates that there is a corridor connecting room1i and room2i,
// allowing a person in the maze to go from room1i to room2i and vice versa.

// The designer of the maze wants to know how confusing the maze is. The confusion score of the maze is the number of different cycles of length 3.

//
// 	For example, 1-> 2-> 3-> 1 is a cycle of length 3, but 1-> 2-> 3-> 4 and 1-> 2-> 3-> 2-> 1 are not.
//

// Two cycles are considered to be different if one or more of the rooms visited in the first cycle is not in the second cycle.

// Return the confusion score of the maze.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2000-2099/2077.Paths%20in%20Maze%20That%20Lead%20to%20Same%20Room/images/image-20211114164827-1.png" style="width: 440px; height: 350px;" />
//
// Input: n = 5, corridors = [[1,2],[5,2],[4,1],[2,4],[3,1],[3,4]]
// Output: 2
// Explanation:
// One cycle of length 3 is 4-> 1-> 3-> 4, denoted in red.
// Note that this is the same cycle as 3-> 4-> 1-> 3 or 1-> 3-> 4-> 1 because the rooms are the same.
// Another cycle of length 3 is 1-> 2-> 4-> 1, denoted in blue.
// Thus, there are two different cycles of length 3.
//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2000-2099/2077.Paths%20in%20Maze%20That%20Lead%20to%20Same%20Room/images/image-20211114164851-2.png" style="width: 329px; height: 250px;" />
//
// Input: n = 4, corridors = [[1,2],[3,4]]
// Output: 0
// Explanation:
// There are no cycles of length 3.
//

// Constraints:

//
// 	2 <= n <= 1000
// 	1 <= corridors.length <= 5 * 104
// 	corridors[i].length == 2
// 	1 <= room1i, room2i <= n
// 	room1i != room2i
// 	There are no duplicate corridors.
//
//  int number_of_paths(int n, vector<vector<int>>& corridors) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn number_of_paths(n: i32, corridors: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashMap, HashSet};
        let mut adj = HashMap::new();
        for corridor in &corridors {
            let (u, v) = if corridor[0] > corridor[1] {
                (corridor[1], corridor[0])
            } else {
                (corridor[0], corridor[1])
            };
            adj.entry(u).or_insert(HashSet::new()).insert(v);
        }
        (1..=n)
            .map(|i| {
                adj.get(&i)
                    .unwrap_or(&HashSet::new())
                    .iter()
                    .map(|j| {
                        adj.get(j)
                            .unwrap_or(&HashSet::new())
                            .iter()
                            .filter(|k| adj.get(&i).unwrap_or(&HashSet::new()).contains(k))
                            .count() as i32
                    })
                    .sum::<i32>()
            })
            .sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_number_of_paths_1() {
        assert_eq!(
            2,
            Solution::number_of_paths(
                5,
                vec![
                    vec![1, 2],
                    vec![5, 2],
                    vec![4, 1],
                    vec![2, 4],
                    vec![3, 1],
                    vec![3, 4]
                ]
            )
        );
    }
    #[test]
    pub fn test_number_of_paths_2() {
        assert_eq!(
            0,
            Solution::number_of_paths(4, vec![vec![1, 2], vec![3, 4]])
        );
    }
}
