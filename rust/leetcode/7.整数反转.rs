/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
//         let mut m = 0;
//         const ten:i32 = 10;
// const bin:i32 = 2;
//         let mut i=1;
//         let result = loop {
  
//         m = m*ten+x%i32::pow(ten,i)/i32::pow(ten,i-1);
//          if  (x>0&& m>(i32::pow(2,31)-1)/10)||(x<0 && m<((-1)*i32::pow(2,31))) {
// return 0;
// }

//        if (x/i32::pow(ten,i)==0){
//  break m;
// }
//             i+=1;
//         };
//         let xx = x.to_string();
//         let xx = xx.trim_start_matches('-');

        
// let xx =xx.chars().rev().collect::<String>();
//         let  signedxx = if x<0{ ("-".to_owned()+&xx).to_string()}else {xx};

//         // println!("xx{}",xx.chars().rev().collect::<String>());
//         // let t = if "2">"12" {1}else{0};
// // println!("t{}",t);
//         // println!("xx{}={}={}",signedxx,(i32::pow(2,31)-1).to_string(),((-1)*i32::pow(2,31)).to_string());

//    if  x>0 && signedxx.len()==(i32::pow(2,31)-1).to_string().len() && signedxx>(i32::pow(2,31)-1).to_string()||x<0 && signedxx.len()==((-1)*i32::pow(2,31)).to_string().len()&& signedxx>((-1)*i32::pow(2,31)).to_string() {
//  return 0;
//  }
//         let result = signedxx.parse::<i32>().unwrap();
        // let mut xx :i64 = x as i64;
        // let mut result:i64 = 0;
        // while xx!=0{
        //     result = result*10+xx%10;
        //     xx /=10;
        // }
        // let max = i32::MAX as i64 ;
        // let min = i32::MIN as i64 ;
        // if result> max||result<min { 0} else {result as i32}

        let mut xx  = x ;
        let mut result = 0;
        while xx!=0{
            if result > i32::MAX/10 || result <i32::MIN/10
{
return 0;
}
            result = result*10+xx%10;
            xx /=10;
        }

        result
    }
}
// @lc code=end

