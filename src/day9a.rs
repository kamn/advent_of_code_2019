use std::fs;
use std::collections::HashMap;


fn read_file() -> String {
    println!("Reading file?");

    let contents = fs::read_to_string("day9.txt")
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
    contents.to_owned()
}

fn create_computer_mem(contents : String) -> HashMap<i64, i64>{
    let data : Vec<String> = contents.split(",").map(ToOwned::to_owned).collect();
    let mut mem = HashMap::new();
    for (pos, e) in data.into_iter().enumerate(){
        let v = e.parse::<i64>().unwrap();
        let pos32 = pos.to_string().parse::<i64>().unwrap();
        println!("Pos.data:\t{} -> {}", pos32, v);
        mem.insert(pos32, v);
    }
    //index
    mem.insert(-2, 0_i64);
    //relative
    mem.insert(-3, 0_i64);
    mem
}


fn get_mem_value(mem : &HashMap<i64,i64>, idx : i64, mode : i64) -> i64 {
    match mode {
        //??
        0 => {
            let other_idx = mem.get(&idx).unwrap_or(&0_i64);
            let val = mem.get(&other_idx).unwrap_or(&0_i64);
            val.clone()
        }
        //??
        1 => {
            let val = mem.get(&idx).unwrap_or(&0_i64);
            val.clone()
        }
        //Relative
        2 => {
            let relative_base : i64 = get_mem_value(&mem, -3, 1);
            let offset = mem.get(&idx).unwrap_or(&0_i64);
            println!("Relative : {} {}", relative_base, offset);
            let val = mem.get(&(relative_base + offset)).unwrap_or(&0_i64);
            val.clone()
        }
        _ => {
            0
        }
    }
}


fn get_mem_idx(mem : &HashMap<i64,i64>, idx : i64, mode : i64) -> i64 {
    match mode {
        //??
        1 => {
            let val = mem.get(&idx).unwrap_or(&0_i64);
            val.clone()
        }
        //Relative
        2 => {
            let relative_base : i64 = get_mem_value(&mem, -3, 1);
            let offset = mem.get(&idx).unwrap_or(&0_i64);
            println!("Relative : {} {}", relative_base, offset);
            relative_base + offset
        }
        _ => {
            let val = mem.get(&idx).unwrap_or(&0_i64);
            val.clone()
        }
    }
}

fn parse_instruction(instruction : i64) -> (i64, i64, i64 , i64) {
    let opcode = instruction % 100;
    let a = (instruction / 10000) % 10;
    let b = (instruction / 1000) % 10;
    let c = (instruction / 100) % 10;
    (a, b, c, opcode)
}

fn  run_computer(mem : &mut HashMap<i64, i64>, mut inputs : Vec<i64>) -> Vec<i64> {
    let mut idx : i64 = get_mem_value(&mem, -2, 1);
    let mut output_result : Vec<i64> = vec![];
    while idx >= 0 {
        let instruction = mem.get(&idx).unwrap_or(&0_i64);
        let (a,b,c,opcode) = parse_instruction(instruction.clone());
        println!("idx> {}", idx);
        match opcode {
            1 => {
                println!("Step => Add");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                let result = val1 + val2;
                let location = get_mem_idx(&mem, idx+3, a);
                ///println!("step: {} + {} = {}\t{}",val1, val2, result, "+++");
                mem.insert(location, result);
                idx = idx +4;
            },
            2 => {
                println!("Step => Multi");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                let result = val1 * val2;
                let location = get_mem_idx(&mem, idx+3, a);
                mem.insert(location, result);
                idx = idx + 4;
                //println!("step\t{} * {} = {} => {}", val1, val2, result, location);
            },
            3 => {
                println!("Step => Input");
                let input_val = inputs.pop().unwrap();
                println!("Input\t {} -> ", input_val);
                let location = get_mem_idx(&mem, idx + 1, c);
                mem.insert(location, input_val);
                println!("=>\t {}", get_mem_value(&mem, 1000, 1));
                idx = idx +2;
                //map_mem(&mem);
            },
            4 => {
                println!("Step => Output");
                let val1 = get_mem_value(&mem, idx+1, c);
                println!("Output\t {}", val1);
                output_result.push(val1);
                //&mem.insert(-2, idx +2);
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
                let location = get_mem_idx(&mem, idx+3, a);
                println!("step: less than {}, {}, to {}",val1, val2, location);

                if val1 < val2 {
                    &mem.insert(location, 1);
                } else {
                    &mem.insert(location, 0);
                }
                idx = idx + 4;
            },
            8 => {
                println!("Step => equals ");
                let val1 = get_mem_value(&mem, idx+1, c);
                let val2 = get_mem_value(&mem, idx+2, b);
                let location = get_mem_idx(&mem, idx+3, a);
                println!("step: equals  {}, {}, to {}",val1, val2, location);

                if val1 == val2 {
                    mem.insert(location, 1);
                } else {
                    mem.insert(location, 0);
                }
                idx = idx + 4;
            },
            9 => {
                println!("Step => relative mod ");
                let mut relative_base = get_mem_value(&mem, -3, 1);
                let val1 = get_mem_value(&mem, idx+1, c);
                relative_base = relative_base + val1;
                println!("=> {}", relative_base);
                mem.insert(-3, relative_base);
                idx = idx + 2;
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

#[allow(dead_code)]
pub fn calc() -> i64 {
    let code = read_file();
    let mem = &mut  create_computer_mem(code.to_string());
    let input_vec : Vec<i64> = vec![1_i64];
    let output_mem = run_computer(mem, input_vec);
    let result_str : Vec<String> = output_mem.into_iter()
        .map(|x : i64|  x.to_string())
        .collect();
    println!("{:?}", result_str);
    result_str.join("").parse::<i64>().unwrap()
}


#[test]
fn test_day9a_example1() {
    let code = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let mem = &mut  create_computer_mem(code.to_string());
    let input_vec : Vec<i64> = vec![1];
    let output_mem = run_computer(mem, input_vec);
    let result_str : Vec<String> = output_mem.into_iter()
        .map(|x : i64|  x.to_string())
        .collect();
    assert_eq!(result_str.join(","), "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
}

#[test]
fn test_day9a_example2() {
    let code = "1102,34915192,34915192,7,4,7,99,0";
    let mem = &mut  create_computer_mem(code.to_string());
    let input_vec : Vec<i64> = vec![];
    let output_mem = run_computer(mem, input_vec);
    let result_str : Vec<String> = output_mem.into_iter()
        .map(|x : i64|  x.to_string())
        .collect();
    assert_eq!(result_str.join(","), "1219070632396864");
}

#[test]
fn test_day9a_example3() {
    let code = "104,1125899906842624,99";
    let mem = &mut  create_computer_mem(code.to_string());
    let input_vec : Vec<i64> = vec![];
    let output_mem = run_computer(mem, input_vec);
    let result_str : Vec<String> = output_mem.into_iter()
        .map(|x : i64|  x.to_string())
        .collect();
    assert_eq!(result_str.join(","), "1125899906842624");
}


#[test]
fn test_day9a_example4() {
    let code = "104,1125899906842624,99";
    let mem = &mut  create_computer_mem(code.to_string());
    mem.insert(1000_i64, 1_i64);
    assert_eq!(get_mem_value(&mem, 1000_i64, 1_i64), 1_i64);
}
