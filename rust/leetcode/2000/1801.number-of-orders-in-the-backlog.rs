/*
 * @lc app=leetcode id=1801 lang=rust
 *
 * [1801] Number of Orders in the Backlog
 */

// @lc code=start
impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        let (mut minsell, mut maxbuy) =
            (BinaryHeap::<Vec<i32>>::new(), BinaryHeap::<Vec<i32>>::new());
        for (i, o) in orders.iter().enumerate() {
            if o[2] == 0 {
                let mut oo = o.clone();
                while oo[1] > 0 && !minsell.is_empty() && -minsell.peek().unwrap()[0] <= o[0] {
                    if let Some(s) = minsell.pop() {
                        let mut ss = s.clone();
                        if s[1] <= oo[1] {
                            oo[1] -= s[1];
                        } else {
                            ss[1] -= oo[1];
                            oo[1] = 0;
                            minsell.push(ss.clone());
                        }
                    }
                }
                if oo[1] > 0 {
                    maxbuy.push(oo.clone());
                }
            } else {
                let mut oo = o.clone();
                while oo[1] > 0 && !maxbuy.is_empty() && maxbuy.peek().unwrap()[0] >= o[0] {
                    if let Some(s) = maxbuy.pop() {
                        let mut ss = s.clone();
                        if s[1] <= oo[1] {
                            oo[1] -= s[1];
                        } else {
                            ss[1] -= oo[1];
                            oo[1] = 0;
                            maxbuy.push(ss.clone());
                        }
                    }
                }
                if oo[1] > 0 {
                    oo[0] *= -1;
                    minsell.push(oo.clone());
                }
            }
        }

        ((minsell.iter().map(|x| x[1] as i64).sum::<i64>() % 1000_000_007
            + maxbuy.iter().map(|x| x[1] as i64).sum::<i64>() % 1000_000_007)
            % 1000_000_007) as _
    }
}
// @lc code=end
