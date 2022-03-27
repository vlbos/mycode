/*
 * @lc app=leetcode id=1584 lang=rust
 *
 * [1584] Min Cost to Connect All Points
 */

// @lc code=start
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let mut connected = HashSet::<Vec<i32>>::new();
        use std::collections::BinaryHeap;
        let dist =
            |x: &Vec<i32>, y: &Vec<i32>| -> i32 { (x[0] - y[0]).abs() + (x[1] - y[1]).abs() };
        let adding_edges = |start: &Vec<i32>,
                            heap: &mut BinaryHeap<(i32, Vec<i32>)>,
                            connected: &HashSet<Vec<i32>>| {
            for p in &points {
                if !connected.contains(p) {
                    heap.push((-dist(start, p), p.clone()));
                }
            }
        };
        let mut heap = BinaryHeap::new();
        connected.insert(points[0].clone());
        adding_edges(&points[0], &mut heap, &connected);
        let mut ans = 0;
        while connected.len() != points.len() {
            if let Some((d, e)) = heap.pop() {
                if connected.contains(&e) {
                    continue;
                }
                ans -= d;
                connected.insert(e.clone());
                adding_edges(&e, &mut heap, &connected);
            }
        }
        ans
    }
}
// @lc code=end
