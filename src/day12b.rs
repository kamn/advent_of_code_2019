use std::fs;
use std::collections::HashMap;
use std::ops::Rem;

fn read_file() -> String {
    println!("Reading file?");

    let contents = fs::read_to_string("day12.txt")
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
    contents.to_owned()
}

#[derive(Clone, Debug, Copy)]
struct Moon {
    id : usize,
    x : i16,
    y : i16,
    z : i16,
    vx : i16,
    vy : i16,
    vz : i16,
}
impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&  self.y == other.y &&  self.z == other.z
    }
}
impl Eq for Moon {}

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

        let mut new_moon = Moon {id:i, x:0, y:0, z:0, vx:0, vy:0, vz:0};
        let vals : Vec<i16> = result.join("").split(", ")
            .map(ToOwned::to_owned)
            .map(|x| {
                x.trim().parse::<i16>().unwrap()
            })
            .collect();
        new_moon.x = vals[0];
        new_moon.y = vals[1];
        new_moon.z = vals[2];
        vec.push(new_moon.clone())
    }
    vec
}

fn clamp(input: i16, min: i16, max: i16) -> i16 {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

fn calc_diff(a: i16, b: i16) -> i16 {
    if a < b {
        -1
    } else if b < a {
        1
    } else {
        0
    }
}


fn physics_step2(moon_a : &mut Moon, moon_b : &mut Moon, moon_c : &mut Moon, moon_d : &mut Moon,) -> i64 {
    let original_a = moon_a.clone();
    let original_b = moon_b.clone();
    let original_c = moon_c.clone();
    let original_d = moon_d.clone();

    let mut steps = 1;
    let mut done = false;
    while !done {
        let abx_diff = calc_diff(moon_b.x, moon_a.x);
        let acx_diff = calc_diff(moon_c.x, moon_a.x);
        let adx_diff = calc_diff(moon_d.x, moon_a.x);
        let bcx_diff = calc_diff(moon_c.x, moon_b.x);
        let bdx_diff = calc_diff(moon_d.x, moon_b.x);
        let cdx_diff = calc_diff(moon_d.x, moon_c.x);

        moon_a.vx += abx_diff + acx_diff + adx_diff;
        moon_b.vx += -abx_diff + bcx_diff + bdx_diff;
        moon_c.vx += -acx_diff + -bcx_diff + cdx_diff;
        moon_d.vx += -adx_diff + -bdx_diff + -cdx_diff;

        let aby_diff = calc_diff(moon_b.y, moon_a.y);
        let acy_diff = calc_diff(moon_c.y, moon_a.y);
        let ady_diff = calc_diff(moon_d.y, moon_a.y);
        let bcy_diff = calc_diff(moon_c.y, moon_b.y);
        let bdy_diff = calc_diff(moon_d.y, moon_b.y);
        let cdy_diff = calc_diff(moon_d.y, moon_c.y);
        moon_a.vy += aby_diff + acy_diff + ady_diff;
        moon_b.vy += -aby_diff + bcy_diff + bdy_diff;
        moon_c.vy += -acy_diff + -bcy_diff + cdy_diff;
        moon_d.vy += -ady_diff + -bdy_diff + -cdy_diff;

        let abz_diff = calc_diff(moon_b.z, moon_a.z);
        let acz_diff = calc_diff(moon_c.z, moon_a.z);
        let adz_diff = calc_diff(moon_d.z, moon_a.z);
        let bcz_diff = calc_diff(moon_c.z, moon_b.z);
        let bdz_diff = calc_diff(moon_d.z, moon_b.z);
        let cdz_diff = calc_diff(moon_d.z, moon_c.z);
        moon_a.vz += abz_diff + acz_diff + adz_diff;
        moon_b.vz += -abz_diff + bcz_diff + bdz_diff;
        moon_c.vz += -acz_diff + -bcz_diff + cdz_diff;
        moon_d.vz += -adz_diff + -bdz_diff + -cdz_diff;

        moon_a.x += moon_a.vx;
        moon_a.y += moon_a.vy;
        moon_a.z += moon_a.vz;

        moon_b.x += moon_b.vx;
        moon_b.y += moon_b.vy;
        moon_b.z += moon_b.vz;

        moon_c.x += moon_c.vx;
        moon_c.y += moon_c.vy;
        moon_c.z += moon_c.vz;
        
        moon_d.x += moon_d.vx;
        moon_d.y += moon_d.vy;
        moon_d.z += moon_d.vz;

        //Find velocity
        
        if steps % 1_000_000 == 0 {
            println!("Steps: {}", steps);
        }
        if (steps == 2){
            //done = true;
        }
        steps += 1;

        if (original_a == *moon_a && original_b == *moon_b && original_c == *moon_c && original_d == *moon_d) {
            done = true;
            println!("Step!: {}", steps);
        }
        /*
        println!("Step: {}", steps);
        println!("A!: {:?}", moon_a);
        println!("B!: {:?}", moon_b);
        println!("C!: {:?}", moon_c);
        println!("D!: {:?}", moon_d);
        */
    }
    steps
}

fn physics_step(mut moons : &mut Vec<Moon>) -> i64 {
    let originals = moons.clone();
    let mut steps = 1;
    let mut done = false;
    while !done { 
        let other_moons = moons.clone();

        //Find velocity
        for  moon_a in moons.iter_mut() {
            for moon_b in other_moons.iter() {
                if moon_a.id != moon_b.id {

                    moon_a.vx += calc_diff(moon_b.x, moon_a.x);
                    moon_a.vy += calc_diff(moon_b.y, moon_a.y);
                    moon_a.vz += calc_diff(moon_b.z, moon_a.z);


                    /*
                    moon_a.x += calc_diff(moon_b.x, moon_a.x);
                    moon_a.y += calc_diff(moon_b.y, moon_a.y);
                    moon_a.z += calc_diff(moon_b.z, moon_a.z);
                    */
                    
                } else {
                    //moon_a.x += moon_b.vx;
                    //moon_a.y += moon_b.vy;
                    //moon_a.z += moon_b.vz;
                }
            }

        }

        //Apply velocity
        //*/
        for moon in moons.iter_mut() {
            moon.x += moon.vx;
            moon.y += moon.vy;
            moon.z += moon.vz;
        }//*/

        if steps % 1_000_000 == 0 {
            println!("Steps: {}", steps);
        }
        steps += 1;
        if (originals[0] == moons[0] && originals[1] == moons[1] && originals[2] == moons[2] && originals[3] == moons[3]) {
            done = true;
            println!("Step!: {}", steps);
        }
    }
    steps
}
 

#[allow(dead_code)]
pub fn calc() -> i64 {
    let code = read_file();
    let result = &mut parse_input(code);
    let data = physics_step(result);

    data
}


#[test]
fn test_day12a_step1() {
    let moons = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";
    let result = &mut parse_input(moons.to_string());
    let data = physics_step(result);
    assert_eq!(data, 4686774924);
}

#[test]
fn test_day12a_step2() {
    let moons = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";
    let result = &mut parse_input(moons.to_string());
    let data = physics_step2(&mut result[0].clone(), &mut result[1].clone(), &mut result[2].clone(), &mut result[3].clone());
    assert_eq!(data, 2772);
}