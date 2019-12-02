mod problem1a;
mod problem1b;
mod problem2a;
mod problem2b;

fn main() {
    println!("Hello, world!");
     problem2a::read_file();
    let result = problem2b::calc();
    println!("Problem 2A:\n{}", result);

}
