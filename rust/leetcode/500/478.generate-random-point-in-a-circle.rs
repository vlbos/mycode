/*
 * @lc app=leetcode id=478 lang=rust
 *
 * [478] Generate Random Point in a Circle
 */

// @lc code=start
struct Solution {
radius: f64, 
x_center: f64, 
y_center: f64
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self{radius,x_center,y_center}
    }
    
    fn rand_point(&self) -> Vec<f64> {
        use rand::distributions::{Distribution, Uniform};
        let mut rng = rand::thread_rng();
        let die = Uniform::from(0f64..1f64);
        let throw = (die.sample(&mut rng) as f64).sqrt();
        let d = self.radius*throw;
        let theta = (die.sample(&mut rng) as f64)*2f64*std::f64::consts::PI;
        vec![self.x_center+d*theta.cos(),self.y_center+d*theta.sin()]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */
// @lc code=end

