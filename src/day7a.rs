use std::fs;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::ops::Rem;


fn read_file() -> String {
    println!("Reading file?");

    let contents = fs::read_to_string("day7.txt")
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
    contents.to_owned()
}

fn create_computer_mem(contents : String) -> HashMap<i32, i32>{
    let data : Vec<String> = contents.split(",").map(ToOwned::to_owned).collect();
    let mut mem = HashMap::new();
    for (pos, e) in data.into_iter().enumerate(){
        let v = e.parse::<i32>().unwrap();
        let pos32 = pos.to_string().parse::<i32>().unwrap();
        //println!("Pos.data:\t{} -> {}", pos32, v);
        mem.insert(pos32, v);
    }
    mem
}


fn get_mem_value(mem : &HashMap<i32,i32>, idx : i32, mode : i32) -> i32 {
    match mode {
        0 => {
            let other_idx = mem.get(&idx).unwrap();
            let val = mem.get(&other_idx).unwrap();
            val.clone()
        }
        1 => {
            let val = mem.get(&idx).unwrap();
            val.clone()
        }
        _ => {
            0
        }
    }
}

fn parse_instruction(instruction : i32) -> (i32, i32, i32 , i32) {
    let opcode = instruction % 100;
    let a = (instruction / 10000) % 10;
    let b = (instruction / 1000) % 10;
    let c = (instruction / 100) % 10;
    (a, b, c, opcode)
}

fn run_computer(mut mem : HashMap<i32, i32>, mut inputs : Vec<i32>) -> i32 {
    let mut idx : i32 = 0;
    let mut result : i32 = 0;
    while idx >= 0 {
        let instruction = mem.get(&idx).unwrap_or(&99);
        let (a,b,c,opcode) = parse_instruction(instruction.clone());
        println!("idx> {}", idx);
        match opcode {
            1 => {
                println!("Step => Add");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                let result = val1 + val2;
                let location = get_mem_value(&mem, idx+3, 1);
                println!("step: {} + {} = {}\t{}",val1, val2, result, "+++");
                mem.insert(location, result);
                idx = idx +4;
            },
            2 => {
                println!("Step => Multi");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                let result = val1 * val2;
                let location = get_mem_value(&mem, idx+3, 1);
                mem.insert(location, result);
                idx = idx + 4;
                println!("step\t{} * {} = {} => {}", val1, val2, result, location);
            },
            3 => {
                println!("Step => Input");
                let mut n = String::new();
                let input_val = inputs.pop().unwrap();
                println!("Input, {}", n);
                let location = mem.get(&(idx + 1)).cloned().unwrap();
                mem.insert(location, input_val);
                idx = idx +2;
                //map_mem(&mem);
            },
            4 => {
                println!("Step => Output");
                let val1 = get_mem_value(&mem, (idx+1), c);
                println!("Output \t {}", val1);
                result = val1;
                idx = idx +2;
            },
            5 => {
                println!("Step => jump-if-true ");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                println!("step: jump-if-true {}, {}",val1, val2);

                if val1 != 0 {
                    idx = val2
                } else {
                    idx = idx + 3;
                }
            },
            6 => {
                println!("Step => jump-if-false ");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                println!("step: jump-if-false {}, {}",val1, val2);

                if val1 == 0 {
                    idx = val2
                } else {
                    idx = idx + 3;
                }
            },
            7 => {
                println!("Step => less-than ");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                let location = get_mem_value(&mem, idx+3, 1);
                println!("step: less than {}, {}, to {}",val1, val2, location);

                if val1 < val2 {
                    mem.insert(location, 1);
                } else {
                    mem.insert(location, 0);
                }
                idx = idx + 4;
            },
            8 => {
                println!("Step => equals ");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                let location = get_mem_value(&mem, idx+3, 1);
                println!("step: equals  {}, {}, to {}",val1, val2, location);

                if val1 == val2 {
                    mem.insert(location, 1);
                } else {
                    mem.insert(location, 0);
                }
                idx = idx + 4;
            },
            99 => {
                println!("Exit");
                idx = -1;
            },
            _ => {
                println!("UNKNOWN! {}", opcode);
                idx = -1;
            }
        };
    }
    result
}

fn get_thrust_signal(sequence : Vec<i32>, code : String) -> i32 {
    let mut intial_input = 0;
    for seq_val in sequence.into_iter() {

        intial_input = run_single_amp(vec![intial_input, seq_val], code.to_owned());
    }
    intial_input
}

fn run_single_amp(inputs: Vec<i32>, code : String) -> i32 {
    let mut mem = create_computer_mem(code);
    run_computer(mem, inputs.clone())
}

fn get_all_combos() -> Vec<Vec<i32>> {
    let mut big_vec = vec![];
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        if (a != b && a != c && a != d && a != e
                            && b != c && b != d && b != e
                            && c != d && c != e
                            && d != e) {
                            big_vec.push(vec![a,b,c,d,e]);
                        }
                    }
                }
            }
        }
    }
    big_vec.clone()
}

#[allow(dead_code)]
pub fn calc() -> i32 {
    let data = read_file();
    let mut highest = 0;
    for combo in get_all_combos().into_iter() {
        println!("Combo:\n{:?}", combo);
        let result = get_thrust_signal(combo, data.to_owned());
        if (result > highest) {
            highest = result;
        }
    }
    highest
}


#[test]
fn test_day7a_example1() {
    let code = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    assert_eq!(get_thrust_signal(vec![4, 3, 2, 1, 0], code.to_string()), 43210);
}

#[test]
fn test_day7a_example2() {
    let code = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    assert_eq!(get_thrust_signal(vec![0, 1, 2, 3, 4], code.to_string()), 54321);
}

#[test]
fn test_day7a_example3() {
    let code = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    assert_eq!(get_thrust_signal(vec![1,0,4,3,2], code.to_string()), 65210);
}
