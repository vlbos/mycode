use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input
        .par_iter()         .map(|&i| i * i)
        .sum()
}
fn main(){
println!("{}",sum_of_squares(&[1,2,3]));
}