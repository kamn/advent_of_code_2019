use std::fs;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::ops::Rem;
use std::collections::HashSet;

#[derive(Clone, Debug)]
enum SpaceObject {
    Nothing,
    Asteroid
}

#[derive(Clone, Debug)]
struct AsteroidInfoScanner {
    origin : (i32, i32),
    formulas : HashSet<(i32, i32)>,
    asteroid_visible_counter : i32 
}


fn read_file() -> String {
    println!("Reading file?");

    let contents = fs::read_to_string("day10.txt")
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
    contents.to_owned()
}


fn parse_input (input : String) -> HashMap<(i32,i32), SpaceObject> {
    let mut map = HashMap::new();
    let lines : Vec<String> = input.lines().map(ToOwned::to_owned).collect();

    for (i, line) in lines.into_iter().enumerate() {
        let mut result : Vec<String> = line.trim().split("").map(ToOwned::to_owned).collect();
        result.pop();
        result.drain(0..1);
        for (y, val) in result.into_iter().enumerate() {
            if val == "#" {
                map.insert((y as i32,i as i32), SpaceObject::Asteroid);
            }
        }
    }
    map
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
    //println!("GCD of {} + {} = {}", x.abs(),  y.abs(), mutual_gcd);
    if x == 0 || y == 0 {
        if x == 0 {
            (0, y/y.abs())
        } else {
            (x/x.abs(), 0)
        }
    } else {
        (x / mutual_gcd, y / mutual_gcd)
    }
}

fn check_spaces_at_distance(map: HashMap<(i32,i32), SpaceObject>, (x,y): (i32, i32), distance : i32) -> HashSet<(i32, i32)> {
    let mut formulas = HashSet::new();
    for i in -distance..(distance+1) {
        for z in -distance..(distance+1) {
            if(i.abs() == distance || z.abs() == distance){
                let at_space = map.get(&(x + i, y + z)).unwrap_or(&SpaceObject::Nothing);
                match at_space {
                    SpaceObject::Asteroid  => {
                        let formula = find_forumla((x,y), (x + i, y + z));
                        formulas.insert(formula);
                        ()
                    },
                    _ => {
                        ()
                    }
                }
            }
        }
    }
    formulas
}

fn count_visible_asteroids(map: HashMap<(i32,i32), SpaceObject>, (x,y): (i32, i32)) -> usize {
    let mut formulas = HashSet::new();
    for dist in 1..100 {
        let new_hashset = check_spaces_at_distance(map.clone(), (x,y).clone(), dist);
        //println!("{:?}", new_hashset);
        formulas = formulas.union(&new_hashset).cloned().collect();
    }
    println!("{:?} -> {:?}", (x,y), formulas.len());

    formulas.len()
}

fn find_best_location (map: HashMap<(i32,i32), SpaceObject>) -> (i32,i32) {
    let mut best =(-1,-1);
    let mut most_ast = 0;
    for x in 0..100 {
        for y in 0..100 {
            let at_space = map.get(&(x,y)).unwrap_or(&SpaceObject::Nothing);
            match at_space {
                SpaceObject::Asteroid  => {
                    let count = count_visible_asteroids(map.clone(), (x,y));
                    if count > most_ast {
                        most_ast = count;
                        best = (x,y);
                        }
                },
                _ => {
                    ();
                }
            }
        }
    }

    best
}


#[allow(dead_code)]
pub fn calc() -> usize {
    let input = read_file();
    let map = parse_input(input);
    let result = find_best_location(map.clone());
    count_visible_asteroids(map,result)
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
fn test_day10a_formula_ex0() {
    let r = ".#..#
    .....
    #####
    ....#
    ...##";
    let map = parse_input(r.to_string());
    let result = find_best_location(map.clone());
    assert_eq!(result, (3,4));
}

#[test]
fn test_day10a_formula_ex1() {
    let r = "......#.#.
    #..#.#....
    ..#######.
    .#.#.###..
    .#..#.....
    ..#....#.#
    #..#....#.
    .##.#..###
    ##...#..#.
    .#....####";
    let map = parse_input(r.to_string());
    let result = find_best_location(map.clone());
    assert_eq!(count_visible_asteroids(map, (5,8)), 33);
    assert_eq!(result, (5,8));
}
#[test]

fn test_day10a_formula_ex2() {
    let r = "#.#...#.#.
    .###....#.
    .#....#...
    ##.#.#.#.#
    ....#.#.#.
    .##..###.#
    ..#...##..
    ..##....##
    ......#...
    .####.###.";
    let map = parse_input(r.to_string());
    let result = find_best_location(map.clone());
    assert_eq!(count_visible_asteroids(map, (1,2)), 35);

    assert_eq!(result, (1,2));
}
#[test]

fn test_day10a_formula_ex3() {
    let r = ".#..#..###
    ####.###.#
    ....###.#.
    ..###.##.#
    ##.##.#.#.
    ....###..#
    ..#.#..#.#
    #..#.#.###
    .##...##.#
    .....#.#..";
    let map = parse_input(r.to_string());
    let result = find_best_location(map.clone());
    assert_eq!(count_visible_asteroids(map,  (6,3)), 41);

    assert_eq!(result, (6,3));
}

#[test]

fn test_day10a_formula_ex4() {
    let r = ".#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##";
    let map = parse_input(r.to_string());
    let result = find_best_location(map.clone());
    assert_eq!(count_visible_asteroids(map, (11,13)), 210);

    assert_eq!(result, (11,13));
}
