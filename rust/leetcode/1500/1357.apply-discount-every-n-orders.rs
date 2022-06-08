/*
 * @lc app=leetcode id=1357 lang=rust
 *
 * [1357] Apply Discount Every n Orders
 */

// @lc code=start
struct Cashier {
n: i32, discount: i32, pp:std::collections::HashMap<i32,i32>,
nth:i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let pp:std::collections::HashMap<i32,i32> = products.iter().cloned().zip(prices).collect();
        Self{n, discount, pp,nth:0}
    }
    
    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut total = 0;
        for (i,p) in product.iter().enumerate(){
            total+= self.pp[p]*amount[i];
        }
        self.nth+=1;
        let mut ans =total as f64 ;
        if self.nth%self.n==0{
            ans *= 1f64-(self.discount as f64)/100f64;
        }
        ans
    }
}

/**
 * Your Cashier object will be instantiated and called as such:
 * let obj = Cashier::new(n, discount, products, prices);
 * let ret_1: f64 = obj.get_bill(product, amount);
 */
// @lc code=end

