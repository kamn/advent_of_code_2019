use std::fs;
use std::collections::HashMap;

fn read_file() -> String {
    println!("Reading file?");

    let contents = fs::read_to_string("day12.txt")
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
    contents.to_owned()
}

#[derive(Clone, Debug, Copy)]
struct Moon {
    x : i32,
    y : i32,
    z : i32,
    vx : i32,
    vy : i32,
    vz : i32,
}


fn parse_input (input : String) -> Vec<Moon> {
    let mut vec = vec![];
    let lines : Vec<String> = input.lines().map(ToOwned::to_owned).collect();

    for (i, line) in lines.into_iter().enumerate() {
        let mut result : Vec<String> = line.trim().split("=").map(ToOwned::to_owned).collect();
        result = result.join("").trim().split("<").map(ToOwned::to_owned).collect();
        result = result.join("").trim().split(">").map(ToOwned::to_owned).collect();//.join("");
        result = result.join("").trim().split("x").map(ToOwned::to_owned).collect();//.join("");
        result = result.join("").trim().split("y").map(ToOwned::to_owned).collect();//.join("");
        result = result.join("").trim().split("z").map(ToOwned::to_owned).collect();//.join("");

        let mut new_moon = Moon {x:0, y:0, z:0, vx:0, vy:0, vz:0};
        let vals : Vec<i32> = result.join("").split(", ")
            .map(ToOwned::to_owned)
            .map(|x| {
                x.trim().parse::<i32>().unwrap()
            })
            .collect();
        new_moon.x = vals[0];
        new_moon.y = vals[1];
        new_moon.z = vals[2];
        vec.push(new_moon.clone())
    }
    vec
}

fn get_moon_pairs(moons : &mut Vec<Moon>) -> Vec<(&mut Moon, &mut Moon)> {
    let mut moon_x = &mut moons[0];
    moon_x.x = 100;
    println!(">{:?}<", moon_x);

    let moon_a =  &mut moons[0];
    
    let moon_b = &mut moons[1];
    let moon_c = &mut moons[2];
    let moon_d = &mut moons[3];

    vec![]

}

fn clamp(input: i32, min: i32, max: i32) -> i32 {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

fn physics_step(mut moons : &mut Vec<Moon>, steps : i32) -> Vec<Moon> {
    for t in 0..steps { 
        let other_moons = moons.clone();

        
        //Find velocity
        for (i, moon_a) in moons.iter_mut().enumerate() {
            for (z, moon_b) in other_moons.clone().into_iter().enumerate() {
                if i != z {
                    moon_a.vx += clamp(moon_b.x - moon_a.x, -1, 1);
                    moon_a.vy += clamp(moon_b.y - moon_a.y, -1, 1);
                    moon_a.vz += clamp(moon_b.z - moon_a.z, -1, 1);
                }
            }

        }


        //Apply velocity
        for moon in moons.iter_mut() {
            moon.x += moon.vx;
            moon.y += moon.vy;
            moon.z += moon.vz;

        }
    }
    moons.clone()
}
 

fn calc_energy(moon: Moon) -> i32 {
    (moon.x.abs() + moon.y.abs() + moon.z.abs()) * (moon.vx.abs() + moon.vy.abs() + moon.vz.abs())
}

#[allow(dead_code)]
pub fn calc() -> i32 {
    let code = read_file();
    let result = &mut parse_input(code);
    let data = physics_step(result, 1000);
    let mut count = 0;
    for m in data.into_iter() {
        count += calc_energy(m)
    }
    //823 too low
    count
}

#[test]
fn test_day12a_parse1() {
    let moons = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";
    let result = parse_input(moons.to_string());
    assert_eq!(result[0].y, 0);
}

#[test]
fn test_day12a_step1() {
    let moons = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";
    let result = &mut parse_input(moons.to_string());
    println!(">{:?}<", result[0]);
    physics_step(result, 1);
    println!(">{:?}<", result[0]);
    //assert_eq!(result[0].y, 100);
}

#[test]
fn test_day12a_step2() {
    let moons = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";
    let result = &mut parse_input(moons.to_string());
    let data = physics_step(result, 100);
    let mut count = 0;
    for m in data.into_iter() {
        count += calc_energy(m)
    }
    assert_eq!(count, 1940);
}