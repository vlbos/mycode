// 1924\. Erect the Fence II[](https://leetcode.ca/2021-08-03-1924-Erect-the-Fence-II/#1924-erect-the-fence-ii)
// ============================================================================================================

// Level[](https://leetcode.ca/2021-08-03-1924-Erect-the-Fence-II/#level)
// ----------------------------------------------------------------------

// Hard

// Description[](https://leetcode.ca/2021-08-03-1924-Erect-the-Fence-II/#description)
// ----------------------------------------------------------------------------------

// You are given a 2D integer array `trees` where `trees[i] = [x_i, y_i]` represents the location of the `i-th` tree in the garden.

// You are asked to fence the entire garden using the minimum length of rope possible.
// The garden is well-fenced only if **all the trees are enclosed** and the rope used **forms a perfect circle**.
// A tree is considered enclosed if it is inside or on the border of the circle.

// More formally, you must form a circle using the rope with a center `(x, y)` and radius `r` where all trees lie inside or on the circle and `r` is **minimum**.

// Return _the center and radius of the circle as a length 3 array `[x, y, r]`_. Answers within `10-5` of the actual answer will be accepted.

// **Example 1:**

// ![Image text](https://assets.leetcode.com/uploads/2021/07/06/trees1.png)

// **Input:** trees = \[\[1,1\],\[2,2\],\[2,0\],\[2,4\],\[3,3\],\[4,2\]\]

// **Output:** \[2.00000,2.00000,2.00000\]

// **Explanation:** The fence will have center = (2, 2) and radius = 2

// **Example 2:**

// ![Image text](https://assets.leetcode.com/uploads/2021/07/06/trees2.png)

// **Input:** trees = \[\[1,2\],\[2,2\],\[4,2\]\]

// **Output:** \[2.50000,2.00000,1.50000\]

// **Explanation:** The fence will have center = (2.5, 2) and radius = 1.5

// **Constraints:**

// *   `1 <= trees.length <= 3000`
// *   `trees[i].length == 2`
// *   `0 <= x_i, y_i <= 3000`

// Solution[](https://leetcode.ca/2021-08-03-1924-Erect-the-Fence-II/#solution)
// ----------------------------------------------------------------------------

// This problem is the smallest-circle problem. Use Welzl’s algorithm to find the smallest circle’s center and radius.

//     class Solution {
//         public double[] outer_trees(int[][] trees) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<f64> {
        let mut ans = vec![trees[0][0] as f64, trees[0][1] as f64, 0.0];
        let n = trees.len();
        let get_squared_distance = |xy1: &Vec<f64>, xy2: &Vec<f64>| {
            xy1[..2]
                .iter()
                .zip(&xy2[..2])
                .map(|(&v1, &v2)| (v1 - v2) * (v1 - v2))
                .sum::<f64>()
        };
        let get_circle = |i: usize, j: usize, k: usize| {
            let (p1, p2) = (
                (trees[i][1] - trees[k][1]) as f64,
                (trees[i][1] - trees[j][1]) as f64,
            );
            let (q1, q2) = (
                (trees[i][0] - trees[k][0]) as f64,
                (trees[i][0] - trees[j][0]) as f64,
            );
            let a = trees[i]
                .iter()
                .zip(&trees[j])
                .map(|(&v1, &v2)| ((v1 * v1) - (v2 * v2)) as f64)
                .sum::<f64>();
            let b = trees[i]
                .iter()
                .zip(&trees[k])
                .map(|(&v1, &v2)| ((v1 * v1) - (v2 * v2)) as f64)
                .sum::<f64>();
            let c = (2 * (trees[i][0] - trees[j][0]) * (trees[i][1] - trees[k][1])
                - 2 * (trees[i][1] - trees[j][1]) * (trees[i][0] - trees[k][0]))
                as f64;
            let (x, y) = ((p1 * a - p2 * b) / c, (q2 * b - q1 * a) / c);
            let r =
                get_squared_distance(&vec![x, y], &vec![trees[k][0] as f64, trees[k][1] as f64])
                    .sqrt();
            vec![x, y, r]
        };
        for i in 1..n {
            let xyi = vec![trees[i][0] as f64, trees[i][1] as f64, 0.0];
            if get_squared_distance(&xyi, &ans) <= ans[2] * ans[2] {
                continue;
            }
            ans = xyi.clone();
            for j in 0..i {
                let xyj = vec![trees[j][0] as f64, trees[j][1] as f64, 0.0];
                if get_squared_distance(&xyj, &ans) <= ans[2] * ans[2] {
                    continue;
                }
                ans = vec![(xyi[0] + xyj[0]) / 2.0, (xyi[1] + xyj[1]) / 2.0, 0.0];
                ans[2] = get_squared_distance(&xyj, &ans).sqrt();
                for k in 0..j {
                    let xyk = vec![trees[k][0] as f64, trees[k][1] as f64, 0.0];
                    if get_squared_distance(&xyk, &ans) > ans[2] * ans[2] {
                        ans = get_circle(i, j, k);
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_outer_trees_1() {
        assert_eq!(
            vec![2.00000, 2.00000, 2.00000],
            Solution::outer_trees(vec![
                vec![1, 1],
                vec![2, 2],
                vec![2, 0],
                vec![2, 4],
                vec![3, 3],
                vec![4, 2]
            ])
        );
    }
    #[test]
    pub fn test_outer_trees_2() {
        assert_eq!(
            vec![2.50000, 2.00000, 1.50000],
            Solution::outer_trees(vec![vec![1, 2], vec![2, 2], vec![4, 2]])
        );
    }
}
