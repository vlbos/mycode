/*
 * @lc app=leetcode id=1396 lang=rust
 *
 * [1396] Design Underground System
 */

// @lc code=start
use std::collections::HashMap;
struct UndergroundSystem {
    s: HashMap<i32, (String, i32)>,
    table: HashMap<(String, String), (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Self {
            s: HashMap::new(),
            table: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.s.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((start_name, start_time)) = self.s.get(&id) {
            self.table
                .entry((start_name.clone(), station_name.clone()))
                .and_modify(|x| {
                    x.0 += t - start_time;
                    x.1 += 1;
                })
                .or_insert((t - start_time, 1));
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some(&(sum, count)) = self.table.get(&(start_station, end_station)) {
            return sum as f64 / count as f64;
        }
        0f64
    }
}
/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
// @lc code=end
