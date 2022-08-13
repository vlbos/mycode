// 469\. Convex Polygon
// ====================

// Given a list of points that form a polygon when joined sequentially,
// find if this polygon is convex [(Convex polygon definition)](https://en.wikipedia.org/wiki/Convex_polygon).

// **Note:**

// 1.  There are at least 3 and at most 10,000 points.
// 2.  Coordinates are in the range -10,000 to 10,000.
// 3.  You may assume the polygon formed by given points is always a simple polygon [(Simple polygon definition)](https://en.wikipedia.org/wiki/Simple_polygon).
// In other words, we ensure that exactly two edges intersect at each vertex, and that edges otherwise **don't intersect each other**.

// **Example 1:**

// \[\[0,0\],\[0,1\],\[1,1\],\[1,0\]\]

// Answer: True

// Explanation:![](https://assets.leetcode.com/uploads/2018/10/13/polygon_convex.png)

// **Example 2:**

// \[\[0,0\],\[0,10\],\[10,10\],\[10,0\],\[5,5\]\]

// Answer: False

// Explanation:![](https://assets.leetcode.com/uploads/2018/10/13/polygon_not_convex.png)

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
impl Solution {
    pub fn is_convex(points: Vec<Vec<i32>>) -> bool {
        // let points = points.into_iter().map(|v| (v[0], v[1])).collect::<Vec<_>>();
        // let len = points.len();
        // if len <= 3 {
        //     return true;
        // }
        // let mut direction: i64 = 0;
        // for i in 0..len {
        //     let (x1, y1) = points[i % len];
        //     let (x2, y2) = points[(i + 1) % len];
        //     let (x3, y3) = points[(i + 2) % len];
        //     let new_directon = Solution::cross((x2 - x1, y2 - y1), (x3 - x1, y3 - y1));
        //     if new_directon != 0 {
        //         if new_directon * direction < 0 {
        //             return false;
        //         }
        //         direction = new_directon;
        //     }
        // }
        // return true;
        let mut points = points;
        points.extend_from_within(..2);
        let mut pre = 0;
        for w in points.windows(3) {
            let mut dxy = Vec::new();
            for d in w.windows(2) {
                dxy.push(vec![d[1][0] - d[0][0], d[1][1] - d[0][1]]);
            }
            let cur = dxy[0][0] * dxy[1][1] - dxy[1][0] * dxy[0][1];
            if cur != 0 {
                if cur * pre < 0 {
                    return false;
                }
                pre = cur;
            }
        }
        true
    }

    // pub fn   cross(p1: (i32, i32), p2: (i32, i32)) -> i64 {
    //     (p1.0 as i64) * (p2.1 as i64) - (p2.0 as i64) * (p1.1 as i64)
    // }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_is_convex_1() {
        assert!(Solution::is_convex(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 1],
            vec![1, 0]
        ]));
    }

    #[test]
    pub fn test_is_convex_2() {
        assert!(!Solution::is_convex(vec![
            vec![0, 0],
            vec![0, 10],
            vec![10, 10],
            vec![10, 0],
            vec![5, 5]
        ]));
    }
}
