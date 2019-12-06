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

fn follow_orbits(orbit_map : &HashMap<String, String>, target : &String, satellite : &String, count : i32) -> i32 {
    let planet = orbit_map.get(satellite).unwrap();
    if planet == target {
        count + 1
    } else {
        follow_orbits(orbit_map, target, planet, count + 1)
    }

}

fn find_mutual_path (orbit_map : &HashMap<String, String>, satelliteA : &String, satelliteB : &String) -> String {

    let mut planetA = satelliteA;
    let mut planetB = satelliteB;
    let mut done = false;
    while !done && planetA != "COM" {
        while !done &&  planetB != "COM" {
            if planetA == planetB {
                done = true
            } else {
                planetB = orbit_map.get(planetB).unwrap();
            }
        }
        if !done {
            planetA = orbit_map.get(planetA).unwrap();
            planetB = satelliteB;

        }
    }
    planetA.clone()
}

fn count_jumps(orbit_map : &HashMap<String, String>) -> i32 {
    let santa_planet = orbit_map.get("SAN").unwrap();
    let your_planet = orbit_map.get("YOU").unwrap();
    let connecting_planet = find_mutual_path(orbit_map, &santa_planet, &your_planet);  
    println!("Connector: {}", connecting_planet);
    follow_orbits(orbit_map, &connecting_planet, &santa_planet, 0) + follow_orbits(orbit_map, &connecting_planet, &your_planet, 0)
}


pub fn calc() -> i32 {
    let data = read_file();
    let result_map = parse_lines(data);
    let orbit_checksum = count_jumps(&result_map);
    orbit_checksum
}


#[test]
fn test_example_jumps() {
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
K)L
K)YOU
I)SAN";
    let vec :  Vec<String> = test_input.lines().map(ToOwned::to_owned).collect();
    let result_map = parse_lines(vec);
    let orbit_jump_count = count_jumps(&result_map);
    assert_eq!(orbit_jump_count, 4);
}