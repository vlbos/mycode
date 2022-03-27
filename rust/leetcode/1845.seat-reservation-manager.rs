/*
 * @lc app=leetcode id=1845 lang=rust
 *
 * [1845] Seat Reservation Manager
 */

// @lc code=start
struct SeatManager {
    r: Vec<bool>,
    m: usize,
    n: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        Self {
            r: vec![false; 100000],
            m: 0,
            n: n as usize,
        }
    }

    fn reserve(&mut self) -> i32 {
        let ans = self.m;
        self.r[ans] = true;
        self.m = self.n;
        for i in ans + 1..self.n {
            if !self.r[i] {
                self.m = i;
                break;
            }
        }
        ans as i32 + 1
    }

    fn unreserve(&mut self, seat_number: i32) {
        let i = seat_number as usize - 1;
        self.m = self.m.min(i);
        self.r[i] = false;
    }
}
/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */
// @lc code=end
