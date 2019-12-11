use std::fs;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::ops::Rem;


fn read_file() -> String {
    println!("Reading file?");

    let contents = fs::read_to_string("day9.txt")
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
    contents.to_owned()
}


fn parse_input (input : String) -> Vec<Vec<i32>> {
    vec![]
}

fn gcd(mut a: i32, mut b : i32 ) ->  i32 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn find_forumla((x2, y2) :(i32, i32), (x1, y1) : (i32, i32)) -> (i32, i32) {
    let mut x = x1 - x2;
    let mut y = y1 - y2;
    let mutual_gcd = gcd(x.abs(), y.abs());
    (x / mutual_gcd, y / mutual_gcd)
}



fn count_visible_asteroids(map: String, (x,y): (i32, i32)) -> i32 {
    0
}

fn find_best_location () -> () {

}


#[allow(dead_code)]
pub fn calc() -> i64 {
    let input = read_file();
    0
}


#[test]
fn test_day10a_gcd() {
    assert_eq!(gcd(4, 4), 4);}

#[test]
fn test_day10a_example1() {
    let r = "
    #.........
    ...A......
    ...B..a...
    .EDCG....a
    ..F.c.b...
    .....c....
    ..efd.c.gb
    .......c..
    ....f...c.
    ...e..d..c";

    assert_eq!(find_forumla((0,0), (4,0)), (1,0));
    //C
    assert_eq!(find_forumla((0,0), (4,4)), (1,1));
    //A
    assert_eq!(find_forumla((0,0), (6,2)), (3,1));
    assert_eq!(find_forumla((0,0), (3,1)), (3,1));
    //F
    assert_eq!(find_forumla((0,0), (2,4)), (1,2));
    assert_eq!(find_forumla((0,0), (3,6)), (1,2));


}

#[test]
fn test_day10a_formula_ex1() {

    assert_eq!(true, true);
}

#[test]
fn test_day10a_example2() {

}

#[test]
fn test_day10a_example3() {

}


#[test]
fn test_day9a_example4() {

}
