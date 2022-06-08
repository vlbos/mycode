/*
 * @lc app=leetcode id=391 lang=rust
 *
 * [391] Perfect Rectangle
 */

// @lc code=start
impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut area = 0;
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (
            rectangles[0][0],
            rectangles[0][1],
            rectangles[0][2],
            rectangles[0][3],
        );
        let mut cnt = std::collections::HashMap::new();
        for rect in &rectangles {
            let (x, y, a, b) = (rect[0], rect[1], rect[2], rect[3]);
            area += (a - x) * (b - y);
            min_x = min_x.min(x);
            max_x = max_x.max(a);
            min_y = min_y.min(y);
            max_y = max_y.max(b);
            *cnt.entry(vec![x, y]).or_insert(0) += 1;
            *cnt.entry(vec![x, b]).or_insert(0) += 1;
            *cnt.entry(vec![a, y]).or_insert(0) += 1;
            *cnt.entry(vec![a, b]).or_insert(0) += 1;
        }
        if area != (max_x - min_x) * (max_y - min_y)
            || *cnt.get(&vec![min_x, min_y]).unwrap_or(&0) != 1
            || *cnt.get(&vec![max_x, max_y]).unwrap_or(&0) != 1
            || *cnt.get(&vec![min_x, max_y]).unwrap_or(&0) != 1
            || *cnt.get(&vec![max_x, min_y]).unwrap_or(&0) != 1
        {
            return false;
        }
        cnt.remove(&vec![min_x, min_y]);
        cnt.remove(&vec![max_x, max_y]);
        cnt.remove(&vec![min_x, max_y]);
        cnt.remove(&vec![max_x, min_y]);
        cnt.values().all(|x| *x == 2 || *x == 4)
    }
}
// @lc code=end
