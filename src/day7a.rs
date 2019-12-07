use std::fs;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::ops::Rem;


fn read_file() -> Vec<String> {
    println!("Reading file?");

    let contents = fs::read_to_string("day7.txt")
        .expect("Something went wrong reading the file");
    
    //println!("With text:\n{}", contents);
    contents.lines().map(ToOwned::to_owned).collect()
}



#[allow(dead_code)]
pub fn calc() -> i32 {
    let data = read_file();
    0
}


#[test]
fn test_example() {
    assert_eq!(true, false);
}