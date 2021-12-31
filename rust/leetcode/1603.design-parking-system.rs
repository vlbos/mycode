/*
 * @lc app=leetcode id=1603 lang=rust
 *
 * [1603] Design Parking System
 */

// @lc code=start
struct ParkingSystem {
   lots:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self{lots:vec![big,medium,small].to_vec()}

    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        let i = car_type as usize -1;
        if self.lots[i]>0{
            self.lots[i]-=1;
            return true;
        }
        false
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */
// @lc code=end

