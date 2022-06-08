/*
 * @lc app=leetcode id=1705 lang=rust
 *
 * [1705] Maximum Number of Eaten Apples
 */

// @lc code=start
impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
         //  let n = days.len();
        // let mut ans = 0;
        // let mut maxday = n;
        // let mut i = 0;
        // let mut q = std::collections::BinaryHeap::new();
        // while i < maxday {
        //     if i < n {
        //         let j = i as i32 + days[i];
        //         q.push((-j, apples[i]));
        //         maxday = maxday.max(j as usize);
        //     }
        //     while let Some((d, a)) = q.pop() {
        //         if -d > i as i32 && a > 0 {
        //             ans += 1;
        //             q.push((d, a - 1));
        //             break;
        //         }
        //     }

        //     i += 1;
        // }
        // ans
         let n = days.len();
        let mut ans = 0;
        let mut i = 0;
        let mut m = std::collections::BTreeMap::new();
        while i < n || !m.is_empty() {
            if i < n {
                let j = i as i32 + days[i];
                *m.entry(j - 1).or_insert(0) += apples[i];
            }
            while !m.is_empty() {
                let (&d, &a) = m.iter().next().unwrap();
                if d >= i as i32 && a > 0 {
                    ans += 1;
                     *m.entry(d).or_insert(0) -= 1;
                    break;
                }else{
                        m.remove(&d);
                }
              
            }
            i += 1;
        }
        ans
    }
}
// @lc code=end
