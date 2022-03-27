/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let r = String::from("IVXLCDM");
        //         const ten:i32 =10;
        //         let mut i  = 0;
        //         let mut result = 0;
        //         let mut preindex=0;
        //     let mut prevalue=0;
        //         while i<s.len(){
        //             let c = &s[i..i+1];
        //            let index = r.find(c).unwrap_or(s.len()) as u32;
        //            let p = if index%2==0 {1}else{5};
        // let v = i32::pow(ten,index/2)*p;
        // result+=v;
        //            if(index> preindex){
        //  result-=prevalue*2;
        // }
        //  preindex = index;
        // prevalue =  v;
        // i+=1;
        let n = vec![1, 5, 10, 50, 100, 500, 1000];
        const ten: i32 = 10;
        let mut i = 0;
        let mut result = 0;
        let mut mid = 0;
        let mut preindex = 0;
        let mut prevalue = 0;
        while i < s.len() {
            let c = &s[i..i + 1];
            let index = r.find(c).unwrap_or(s.len()) as u32;
            let v = *n.get((index as usize)).unwrap();
            result += v;
            if (index > preindex) {
                result -= prevalue * 2;
            }

            preindex = index;
            prevalue = v;
            i += 1;
        }

        result
    }
}
// @lc code=end
