
use std::fs;
use std::str::Lines;

pub fn read_file() -> Vec<String> {
    println!("Reading file?");

    let contents = fs::read_to_string("problem1.txt")
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
    contents.lines().map(ToOwned::to_owned).collect()
}

fn calc_fuel(num : i32) -> i32 {
    (num/3) -2
}

pub fn calc() -> i32 {
    let data = read_file();
    data.into_iter()
    .map(|x| x.parse::<i32>().unwrap())
    .fold(0, |acc, x| acc + calc_fuel(x))
}
//Modules
//  Mass
//  fuel = floor(mass/3) - 2
