// 296\. Best Meeting Point
// ========================

// A group of two or more people wants to meet and minimize the total travel distance. You are given a 2D grid of values 0 or 1, where each 1 marks the home of someone in the group. The distance is calculated using [Manhattan Distance](http://en.wikipedia.org/wiki/Taxicab_geometry), where distance(p1, p2) = `|p2.x - p1.x| + |p2.y - p1.y|`.

// **Example:**

// **Input:**

// 1 - 0 - 0 - 0 - 1
// |   |   |   |   |
// 0 - 0 - 0 - 0 - 0
// |   |   |   |   |
// 0 - 0 - 1 - 0 - 0

// **Output: 6

// Explanation:** Given three people living at `(0,0)`, `(0,4)`, and `(2,2)`:
//              The point `(0,2)` is an ideal meeting point, as the total travel distance
//              of 2+2+2=6 is minimal. So return 6.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Snapchat](https://leetcode.ca/tags/#Snapchat) [Twitter](https://leetcode.ca/tags/#Twitter)

// @lc code=start
impl Solution {
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        if grid[0].is_empty() {
            return 0;
        }
        let mut rows = vec![];
        for (i, r) in grid.iter().enumerate() {
            for v in r.iter() {
                if *v == 1 {
                    rows.push(i);
                }
            }
        }
        let mut cols = vec![];
        for j in 0..grid[0].len() {
            for i in 0..grid.len() {
                if grid[i][j] == 1 {
                    cols.push(j);
                }
            }
        }
        (Solution::min_distance_linear(&rows) + Solution::min_distance_linear(&cols)) as i32
    }

    fn min_distance_linear(arr: &[usize]) -> usize {
        if arr.is_empty() {
            return 0;
        }
        let mut distance = 0usize;
        let mut i = 0usize;
        let mut j = arr.len() - 1;
        while i < j {
            distance += (arr[j] - arr[i]) as usize;
            i += 1;
            j -= 1;
        }
        return distance;
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_total_distance() {
        let matrix = vec![
            vec![1, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
        ];
        assert_eq!(Solution::min_total_distance(matrix), 6);
    }
}
