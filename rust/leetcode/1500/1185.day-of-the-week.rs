/*
 * @lc app=leetcode id=1185 lang=rust
 *
 * [1185] Day of the Week
 */

// @lc code=start
impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let r =vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        let mut days = 0;
        for i in 1971..year{
             days+=365;
             if i%4==0 && (i%100!=0||i%400==0){
                days+=1;
             }
        }
        for i in 1..month{
             if i==2{
                days+=28;
                if year%4==0 && (year%100!=0||year%400==0){
                    days+=1;
                }
            }else if i==4||i==6||i==9||i==11{
                days+=30;
            }else{
                days+=31;
            }
        }
        days+=day;
        r[((days+4)%7) as usize].to_string()
    }
}
// @lc code=end

