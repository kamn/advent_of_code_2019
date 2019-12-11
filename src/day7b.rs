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
    //index
    mem.insert(-2, 0);
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

fn run_computer(mem : &mut HashMap<i32, i32>, mut inputs : Vec<i32>, signal : i32) -> i32 {
    let mut idx : i32 = get_mem_value(&mem, -2, 1);
    let mut output_result : i32 = signal;
    while idx >= 0 {
        let instruction = mem.get(&idx).unwrap_or(&99);
        let (a,b,c,opcode) = parse_instruction(instruction.clone());
        //println!("idx> {}", idx);
        match opcode {
            1 => {
                //println!("Step => Add");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                let result = val1 + val2;
                let location = get_mem_value(&mem, idx+3, 1);
                ///println!("step: {} + {} = {}\t{}",val1, val2, result, "+++");
                mem.insert(location, result);
                idx = idx +4;
            },
            2 => {
                //println!("Step => Multi");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                let result = val1 * val2;
                let location = get_mem_value(&mem, idx+3, 1);
                mem.insert(location, result);
                idx = idx + 4;
                //println!("step\t{} * {} = {} => {}", val1, val2, result, location);
            },
            3 => {
                //println!("Step => Input");
                let n = String::new();
                let input_val = inputs.pop().unwrap();
                print!("Input\t {} -> ", input_val);
                let location = mem.get(&(idx + 1)).cloned().unwrap();
                mem.insert(location, input_val);
                idx = idx +2;
                //map_mem(&mem);
            },
            4 => {
                //println!("Step => Output");
                let val1 = get_mem_value(&mem, (idx+1), c);
                println!("Output\t {}", val1);
                output_result = val1;
                &mem.insert(-2, idx +2);
                idx = -1;
            },
            5 => {
                //println!("Step => jump-if-true ");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                //println!("step: jump-if-true {}, {}",val1, val2);

                if val1 != 0 {
                    idx = val2
                } else {
                    idx = idx + 3;
                }
            },
            6 => {
                //println!("Step => jump-if-false ");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                //println!("step: jump-if-false {}, {}",val1, val2);

                if val1 == 0 {
                    idx = val2
                } else {
                    idx = idx + 3;
                }
            },
            7 => {
                //println!("Step => less-than ");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                let location = get_mem_value(&mem, idx+3, 1);
                //println!("step: less than {}, {}, to {}",val1, val2, location);

                if val1 < val2 {
                    &mem.insert(location, 1);
                } else {
                    &mem.insert(location, 0);
                }
                idx = idx + 4;
            },
            8 => {
                //println!("Step => equals ");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                let location = get_mem_value(&mem, idx+3, 1);
                //println!("step: equals  {}, {}, to {}",val1, val2, location);

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
                &mem.insert(-2, -1);
            },
            _ => {
                println!("UNKNOWN! {}", opcode);
                idx = -1;
            }
        };
    }
    output_result
}

fn computer_halted(mem : &mut HashMap<i32, i32>) -> bool {
    let idx : i32 = get_mem_value(&mem, -2, 1);
    let halted = idx == -1;
    println!("halted {:?}", halted);
    halted
}

fn get_thrust_signal(sequence : Vec<i32>, code : String) -> i32 {
    let mut intial_input = 0;
    let amp_a = &mut create_computer_mem(code.to_owned());
    let amp_b = &mut create_computer_mem(code.to_owned());
    let amp_c = &mut create_computer_mem(code.to_owned());
    let amp_d = &mut create_computer_mem(code.to_owned());
    let amp_e = &mut create_computer_mem(code.to_owned());
 
    let mut round_robin_vec = vec![amp_e,amp_d,amp_c,amp_b,amp_a];
    let mut stop = false;
    for seq_val in sequence.clone().into_iter() {
        let cur_mem = round_robin_vec.pop().unwrap();
        intial_input = run_single_amp(vec![intial_input, seq_val], cur_mem, intial_input);
        round_robin_vec.insert(0, cur_mem);
    }

    println!("### sTARTING LOOPING!");
    while !stop {
        let mut in_loop = intial_input;
        for seq_val in sequence.clone().into_iter() {
            let cur_mem = round_robin_vec.pop().unwrap();
            in_loop = run_single_amp(vec![in_loop], cur_mem, in_loop);
            round_robin_vec.insert(0, cur_mem);
        }

        let cur_mem = round_robin_vec.pop().unwrap();
        stop = computer_halted(cur_mem);
        round_robin_vec.push(cur_mem);
        if !stop  {
            intial_input = in_loop
        }else {

        }

    }
    intial_input
}

fn run_single_amp(inputs: Vec<i32>, mem : &mut HashMap<i32, i32>, signal : i32) -> i32 {
    run_computer(mem, inputs.clone(), signal)
}

fn get_all_feedback_combos() -> Vec<Vec<i32>> {
    let mut big_vec = vec![];
    for a in 5..10 {
        for b in 5..10 {
            for c in 5..10 {
                for d in 5..10 {
                    for e in 5..10 {
                        if a != b && a != c && a != d && a != e
                            && b != c && b != d && b != e
                            && c != d && c != e
                            && d != e {
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
    for combo in get_all_feedback_combos().into_iter() {
        println!("Combo:\n{:?}", combo);
        let result = get_thrust_signal(combo, data.to_owned());
        if (result > highest) {
            highest = result;
        }
    }
    highest
}


#[test]
fn test_day7b_example1() {
    let code = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    assert_eq!(get_thrust_signal(vec![9,8,7,6,5], code.to_string()), 139629729);
}

#[test]
fn test_day7b_example2() {
    let code = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    assert_eq!(get_thrust_signal(vec![9,7,8,5,6], code.to_string()), 18216);
}
