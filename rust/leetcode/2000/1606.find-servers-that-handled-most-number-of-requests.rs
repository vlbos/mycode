/*
 * @lc app=leetcode id=1606 lang=rust
 *
 * [1606] Find Servers That Handled Most Number of Requests
 */

// @lc code=start
impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut available =
            BinaryHeap::from((0..k).map(|x| Reverse(x)).collect::<Vec<Reverse<i32>>>());
        let mut busy = BinaryHeap::<Reverse<(i32,i32)>>::new();
        let mut requests = vec![0; k as usize];
        for (i, (start, t)) in arrival.into_iter().zip(load).enumerate() {
            let ii = i as i32;
            while !busy.is_empty() && busy.peek().unwrap().0.0 <= start {
                let Reverse((_, id)) = busy.pop().unwrap();
                available.push(Reverse(ii + ((id - ii) % k+k)%k));
            }
            if let Some(Reverse(id)) = available.pop() {
                let id = id % k;
                requests[id as usize] += 1;
                busy.push(Reverse((start + t, id)));
            }
        }
        let max = *requests.iter().max().unwrap();
        requests
            .iter()
            .enumerate()
            .filter(|x| *x.1 == max)
            .map(|x| x.0 as i32)
            .collect()
    }
}
// @lc code=end
