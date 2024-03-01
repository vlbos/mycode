// 1168\. Optimize Water Distribution in a Village
// ===============================================

// There are `n` houses in a village. We want to supply water for all the houses by building wells and laying pipes.

// For each house `i`, we can either build a well inside it directly with cost `wells[i]`, or pipe in water from another well to it.
// The costs to lay pipes between houses are given by the array `pipes`,
// where each `pipes[i] = [house1, house2, cost]` represents the cost to connect `house1` and `house2` together using a pipe.
// Connections are bidirectional.

// Find the minimum total cost to supply water to all houses.

// **Example 1:**

// **![](https://assets.leetcode.com/uploads/2019/05/22/1359_ex1.png)**

// **Input:** n = 3, wells = \[1,2,2\], pipes = \[\[1,2,1\],\[2,3,1\]\]
// **Output:** 3
// **Explanation:**
// The image shows the costs of connecting houses using pipes.
// The best strategy is to build a well in the first house with cost 1 and connect the other houses to it with cost 2 so the total cost is 3.

// **Constraints:**

// *   `1 <= n <= 10000`
// *   `wells.length == n`
// *   `0 <= wells[i] <= 10^5`
// *   `1 <= pipes.length <= 10000`
// *   `1 <= pipes[i][0], pipes[i][1] <= n`
// *   `0 <= pipes[i][2] <= 10^5`
// *   `pipes[i][0] != pipes[i][1]`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google) [Yahoo](https://leetcode.ca/tags/#Yahoo)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn min_cost_to_supply_water(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
        let mut edges = pipes;
        edges.extend(
            wells
                .into_iter()
                .enumerate()
                .map(|(i, v)| vec![0, i as i32 + 1, v])
                .collect::<Vec<Vec<i32>>>(),
        );
        edges.sort_by_key(|x| x[2]);
        fn find(x: usize, parent: &mut Vec<usize>) -> usize {
            if x != parent[x] {
                let px = parent[x];
                parent[x] = find(px, parent);
            }
            parent[x]
        }
        fn unite(x: usize, y: usize, parent: &mut Vec<usize>) {
            let (x, y) = (find(x, parent), find(y, parent));
            if x != y {
                parent[x] = y;
            }
        }
        let mut parent: Vec<usize> = (0..=n as usize).collect();
        let mut ans = 0;
        for e in &edges {
            let (x, y) = (e[0] as usize, e[1] as usize);
            if find(x, &mut parent) == find(y, &mut parent) {
                continue;
            }
            unite(x, y, &mut parent);
            ans += e[2];
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_cost_to_supply_water_1() {
        assert_eq!(
            3,
            Solution::min_cost_to_supply_water(
                3,
                vec![1, 2, 2],
                vec![vec![1, 2, 1], vec![2, 3, 1]]
            )
        );
    }
}
