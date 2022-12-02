use std::fs;

pub fn run() {
    let data = fs::read_to_string("dec_01.txt").expect("unable to read file");

   let mut big_sum_one = 0;
   let mut big_sum_two = 0;
   let mut big_sum_three = 0;
   let mut sum = 0;

   for line in data.lines(){
    if !line.is_empty(){
        sum += line.parse::<i32>().unwrap();
    } else {
        if sum > big_sum_one{
            big_sum_three = big_sum_two;
            big_sum_two = big_sum_one;
            big_sum_one = sum;
        }else if sum > big_sum_two{
            big_sum_three = big_sum_two;
            big_sum_two = sum
        }else if sum > big_sum_three{
            big_sum_three = sum
        }
        sum = 0;
    }
   }
   println!("Part1: {}", big_sum_one);
   println!("Part2: {}", (big_sum_one + big_sum_two + big_sum_three));
}