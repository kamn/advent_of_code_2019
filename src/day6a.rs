use std::fs;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::ops::Rem;


fn read_file() -> Vec<String> {
    println!("Reading file?");

    let contents = fs::read_to_string("day6.txt")
        .expect("Something went wrong reading the file");
    
    //println!("With text:\n{}", contents);
    contents.lines().map(ToOwned::to_owned).collect()
}


fn parse_lines(input_lines: Vec<String>) -> HashMap<String, String> {
    let mut orbit_map = HashMap::new();
    for line in input_lines.into_iter() {
        println!("line {}", line);
        let split_line :  Vec<String> = line.split(")").map(ToOwned::to_owned).collect();
        let planet : String = split_line[0].trim().to_owned();
        let satellite = split_line[1].trim().to_owned();

        orbit_map.insert(satellite, planet);
    }
    println!("{:?}", orbit_map);
    orbit_map
}

fn follow_orbits(orbit_map : &HashMap<String, String>, satellite : &String, count : i32) -> i32 {
    let planet = orbit_map.get(satellite).unwrap();
    if planet == "COM" {
        count + 1
    } else {
        follow_orbits(orbit_map, planet, count + 1)
    }

}

fn count_orbits(orbit_map : &HashMap<String, String>) -> i32 {
    let mut counter = 0;
    for (pos, value) in orbit_map {
        counter = counter + follow_orbits(orbit_map, &pos, 0)
    }
    counter
}
pub fn calc() -> i32 {
    let data = read_file();
    let result_map = parse_lines(data);
    let orbit_checksum = count_orbits(&result_map);
    orbit_checksum
}


#[test]
fn test_example() {
    let test_input = 
    "COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L";
    let vec :  Vec<String> = test_input.lines().map(ToOwned::to_owned).collect();
    let result_map = parse_lines(vec);
    let orbit_checksum = count_orbits(&result_map);
    assert_eq!(orbit_checksum, 42);
}