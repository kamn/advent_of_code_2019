use std::fs;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::ops::Rem;


fn read_file() -> String {
    println!("Reading file?");

    let contents = fs::read_to_string("day8.txt")
        .expect("Something went wrong reading the file");
    
    //println!("With text:\n{}", contents);
    contents.to_owned()
}

fn parse_input (contents : String) -> Vec<String> {
    let mut result : Vec<String> = contents.trim().split("").map(ToOwned::to_owned).collect();
    result.pop();
    result.drain(0..1);
    result
}

#[allow(dead_code)]
fn count_string (layer : Vec<String>, pattern : String)  -> i32 {
    let mut total = 0;
    for c in layer.into_iter() {
        if c == pattern {
            total  = total +1;
        }
    }
    total
}

fn process(raw_data : Vec<String>, height : usize, width : usize) -> Vec<String> {
    let mut image = vec!["2".to_string(); height*width];
    let iter  = raw_data.chunks(height*width);

    for layer in iter {
        let collected = layer.to_vec();
        let mut new_image = vec![];
        for (old_pixel, new_pixel) in image.into_iter().zip(collected.into_iter()) {
            if old_pixel == "2" {
                new_image.push(new_pixel);
            } else {
                new_image.push(old_pixel.to_string());
            }
        }
        image = new_image.clone();
        
    }
    image
}

fn print(image : Vec<String>, height : usize, width : usize) -> () {
    let iter  = image.chunks(width);
    for layer in iter {
        println!("{:?}",layer);
    }
}

#[allow(dead_code)]
pub fn calc() -> i32 {
    let data = read_file();
    let input = parse_input(data);
    let image = process(input, 25, 6);
    print(image, 6, 25);
    0
}




#[test]
fn test_day8a_count_str() {
    let example1 = "123456789012";
    let x : Vec<String> = example1.to_string().split("").map(ToOwned::to_owned).collect();
    assert_eq!(count_string(x.to_owned(), "1".to_string()), 2);
    assert_eq!(count_string(x.to_owned(), "5".to_string()), 1);

}

