/*
 * @lc app=leetcode id=871 lang=rust
 *
 * [871] Minimum Number of Refueling Stops
 */

// @lc code=start
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut bh = std::collections::BinaryHeap::new();
        let mut stations = stations;
        let mut tank = start_fuel;
        stations.push(vec![target, i32::MAX]);
        let mut ans = 0;
        let mut prev = 0;
        for station in &stations {
            let (location, capacity) = (station[0], station[1]);
            tank -= location - prev;
            while !bh.is_empty() && tank < 0 {
                tank += bh.pop().unwrap();
                ans += 1;
            }
            if tank < 0 {
                return -1;
            }
            bh.push(capacity);
            prev = location;
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut dp=vec![0;stations.len()+1];
        dp[0]=start_fuel;
        for (i,v) in stations.iter().enumerate(){
            for j in (0..=i).rev(){
                if dp[j]>=v[0]{
                    dp[j+1]=dp[j+1].max(dp[j]+v[1]);
                }
            }
        }
        dp.into_iter().enumerate().filter_map(|(i,v)| if v>=target{Some(i as i32)}else{None}).next().unwrap_or(-1)
    }
}