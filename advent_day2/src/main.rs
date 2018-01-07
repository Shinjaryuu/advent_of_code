use std::fs::File;
use std::io::prelude::*;


fn check_sum (nums : &Vec<i32>) -> i32 {
    let max = match nums.iter().max(){
        Some(i) => i,
        None => {println!("Empty list! Assuming checksum 0."); &0},
    };
    let min = match nums.iter().min(){
        Some(i) => i,
        None => {println!("Empty list! Assuming checksum 0."); &0},
    };
    max - min
}

fn line_to_numvec(line : &str) -> Vec<i32> {
    let nums = line.split_whitespace().map(|x| x.parse::<i32>().expect("Cannot convert to number")).collect();
    nums
}

fn main() {
    let filename = "input.txt";
    let mut f = File::open(filename).expect("File not found!");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Problem while reading file.");

    let nums = contents.lines().map(line_to_numvec);

    let sum : i32 = nums.map(|x| check_sum(&x)).sum();
    //let a = vec![2,10,4,5,7];
    //let cs = check_sum(&a);
    //println!("{}",contents);
    //let sum = nums.map(check_sum);
    //for n in nums{
    //    println!{"{:?}",check_sum(&n)};
    //}
    println!("{:?}", sum)
}
