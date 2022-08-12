// 1135\. Connecting Cities With Minimum Cost
// ==========================================

// There are `N` cities numbered from 1 to `N`.

// You are given `connections`, where each `connections[i] = [city1, city2, cost]` represents the cost to connect `city1` and `city2` together.
//  (A _connection_ is bidirectional: connecting `city1` and `city2` is the same as connecting `city2` and `city1`.)

// Return the minimum cost so that for every pair of cities, there exists a path of connections (possibly of length 1) that connects those two cities together.
//  The cost is the sum of the connection costs used. If the task is impossible, return -1.

// **Example 1:**

// ![](https://leetcode.ca/all/img/1135_1.png)

// **Input:** N = 3, connections = \[\[1,2,5\],\[1,3,6\],\[2,3,1\]\]
// **Output:** 6
// **Explanation:**
// Choosing any 2 edges will connect all cities so we choose the minimum 2.

// **Example 2:**

// ![](https://leetcode.ca/all/img/1135_2.png)

// **Input:** N = 4, connections = \[\[1,2,3\],\[3,4,4\]\]
// **Output:** \-1
// **Explanation:**
// There is no way to connect all cities even if all edges are used.

// **Note:**

// 1.  `1 <= N <= 10000`
// 2.  `1 <= connections.length <= 10000`
// 3.  `1 <= connections[i][0], connections[i][1] <= N`
// 4.  `0 <= connections[i][2] <= 10^5`
// 5.  `connections[i][0] != connections[i][1]`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Uber](https://leetcode.ca/tags/#Uber)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn   minimum_cost(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut connections = connections;
        connections.sort_by_key(|x| x[2]);
        let mut weight = vec![1; n + 1];
        let mut parent: Vec<usize> = (0..=n).collect();
       pub fn  find(x: usize, parent: &mut Vec<usize>) -> usize {
            if x == parent[x] {
                return x;
            }
            let y = parent[x];
            parent[x] = find(y, parent);
            parent[x]
        }
       pub fn  unite(
            x: usize,
            y: usize,
            parent: &mut Vec<usize>,
            weight: &mut Vec<i32>,
            count: &mut i32,
        ) {
            let (px, py) = (find(x, parent), find(y, parent));
            let (px, py) = if weight[px] > weight[py] {
                (py, px)
            } else {
                (px, py)
            };
            parent[px] = py;
            weight[py] += weight[px];
            *count -= 1;
        }

        let mut ans = 0;
        let mut count = n as i32;

        for c in &connections {
            let (u, v) = (c[0] as usize, c[1] as usize);
            if find(u, &mut parent) != find(v, &mut parent) {
                unite(u, v, &mut parent, &mut weight, &mut count);
                ans += c[2];
            }
            if count == 1 {
                return ans;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_minimum_cost_1() {
        assert_eq!(
            6,
            Solution::minimum_cost(3, vec![vec![1, 2, 5], vec![1, 3, 6], vec![2, 3, 1]])
        );
    }
    #[test]
   pub fn  test_minimum_cost_2() {
        assert_eq!(
            -1,
            Solution::minimum_cost(4, vec![vec![1, 2, 3], vec![3, 4, 4]])
        );
    }
}
