/*
 * @lc app=leetcode id=2043 lang=rust
 *
 * [2043] Simple Bank System
 */

// @lc code=start
struct Bank {
balance: Vec<i64>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        Self{balance}
    }
    
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if account1<=0 || account1>self.balance.len() as i32{
                return false;
        }
           if account2<=0 || account2>self.balance.len() as i32{
                return false;
        }
        let (i1,i2)=(account1 as usize-1,account2 as usize-1);
        if self.balance[i1]<money{
        return false;
        }
        self.balance[i1]-=money;
        self.balance[i2]+=money;
        true
    }
    
    fn deposit(&mut self, account: i32, money: i64) -> bool {
           if account<=0 || account>self.balance.len() as i32{
                return false;
        }
        self.balance[account as usize-1]+=money;
        true
    }
    
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if account<=0 || account>self.balance.len() as i32{
                return false;
        }
        let i=(account as usize-1);
        if self.balance[i]<money{
        return false;
        }
        self.balance[i]-=money;
        true
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */
// @lc code=end

