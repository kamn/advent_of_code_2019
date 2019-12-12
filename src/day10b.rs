use std::fs;
use std::collections::HashMap;
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
    let x = x1 - x2;
    let y = y1 - y2;
    let mutual_gcd = gcd(x.abs(), y.abs());
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
            if i.abs() == distance || z.abs() == distance {
                let at_space = map.get(&(x + i, y + z)).unwrap_or(&SpaceObject::Nothing);
                match at_space {
                    SpaceObject::Asteroid  => {
                        let formula = find_forumla((x,y), (x + i, y + z));
                        if (formula == (-2,-1)) {
                            println!("FOUND {:?}", (x + i, y + z));
                        }
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


fn get_all_formulas(map: HashMap<(i32,i32), SpaceObject>, (x,y): (i32, i32)) -> HashSet<(i32, i32)> {
    let mut formulas = HashSet::new();
    for dist in 1..60 {
        let new_hashset = check_spaces_at_distance(map.clone(), (x,y).clone(), dist);
        formulas = formulas.union(&new_hashset).cloned().collect();
    }
    println!("{:?} -> {:?}", (x,y), formulas.len());

    formulas
}

fn get_sorted_formulas(map: HashMap<(i32,i32), SpaceObject>, (x,y): (i32, i32)) -> Vec<(i32, i32)> {
    let forumla_set = get_all_formulas(map, (x,y));
    let mut non_sorted_vec = vec![];
    for x in forumla_set.iter() {
        non_sorted_vec.push(x.clone());
    }
    non_sorted_vec.sort_by(|a, b| {
        sort_by_polar(a.clone(), b.clone())
    });
    non_sorted_vec
}

fn get_polar_radian((x,y): (i32, i32)) -> f32 {
    let fx = (x as f32);
    let fy = -(y as f32);
    let result = fx.atan2(fy);
    if result < 0.0 {
        result + (std::f32::consts::PI * 2.0)
    } else {
        result
    }
}

fn sort_by_polar((x1,y1) :(i32,i32), (x2,y2) :(i32,i32)) -> std::cmp::Ordering {
    let polar1 = get_polar_radian((x1, y1));
    let polar2 = get_polar_radian((x2, y2));
    polar1.partial_cmp(&polar2).unwrap()

}


pub fn calc() -> usize {
    let input = read_file();
    let map = parse_input(input);
    //(28, 29) -> 340
    //(15, 19)
    //(43, 48)
    //4348 -> Too high
    // 2nd
    // (28, 29)
    // (13, 36) 
    // 4165

    //3rd
    //
    //(26, 28)
    //2628
    let mut non_sorted_vec = get_sorted_formulas(map.clone(), (28, 29));
    for (i, x) in non_sorted_vec.iter().enumerate() {
        println!("{} -> {:?}",i, x)
    }
    println!("{:?}",  &map.get(&(41,65)).unwrap_or(&SpaceObject::Nothing));
    println!("{:?}",  &map.get(&((28 + 13*2),(29 + 36*2))).unwrap_or(&SpaceObject::Nothing));

    println!("{:?}",  &map.get(&(65,41)).unwrap_or(&SpaceObject::Nothing));
    0
}

#[test]
fn test_day10a_example1() {

    //Above
    assert_eq!(get_polar_radian((0, -1)), 0.0f32);

    //Right
    assert_eq!(get_polar_radian((1, 0)), std::f32::consts::FRAC_PI_2);

    //Below
    assert_eq!(get_polar_radian((0, 1)), std::f32::consts::PI);

    //Left
    assert_eq!(get_polar_radian((-1, 0)), std::f32::consts::PI + std::f32::consts::FRAC_PI_2);
}

#[test]
fn test_day10a_formula_ex0() {
    let r = "
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##";
    //(8,3)
    let map = parse_input(r.to_string());
    let forumla_set = get_all_formulas(map, (8,3));
    let mut non_sorted_vec = vec![];
    for x in forumla_set.iter() {
        non_sorted_vec.push(x);
    }
    non_sorted_vec.sort_by(|a, b| {
        sort_by_polar(*a.clone(), *b.clone())
    });
     for (i, x) in non_sorted_vec.iter().enumerate() {
        println!("{} -> {:?}",i, x)
    }
    assert_eq!(forumla_set.len(), 2);
}
