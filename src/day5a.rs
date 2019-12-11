use std::fs;
use std::collections::HashMap;
use std::io;


pub fn read_file() -> Vec<String> {
    println!("Reading file?");

    let contents = fs::read_to_string("day5.txt")
        .expect("Something went wrong reading the file");
    
    //println!("With text:\n{}", contents);
    contents.split(",").map(ToOwned::to_owned).collect()
}

fn create_computer_mem(data : Vec<String>) -> HashMap<i32, i32>{
    let mut mem = HashMap::new();
    for (pos, e) in data.into_iter().enumerate(){
        let v = e.parse::<i32>().unwrap();
        let pos32 = pos.to_string().parse::<i32>().unwrap();
        println!("Pos.data:\t{} -> {}", pos32, v);
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

fn run_computer(mut mem : HashMap<i32, i32>) -> HashMap<i32, i32> {
    let mut idx : i32 = 0;
    while idx >= 0 {
        let instruction = mem.get(&idx).unwrap_or(&99);
        let (_a,b,c,opcode) = parse_instruction(instruction.clone());
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
                //map_mem(&mem);
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
                //map_mem(&mem);
            },
            3 => {
                println!("Step => Input");
                let mut n = String::new();
                io::stdin()
                    .read_line(&mut n)
                    .expect("failed to read input.");
                let input_val = n.trim().parse::<i32>().unwrap();
                println!("Input, {}", n);
                let location = mem.get(&(idx + 1)).cloned().unwrap();
                mem.insert(location, input_val);
                idx = idx +2;
                //map_mem(&mem);
            },
            4 => {
                println!("Step => Output");
                let val1 = get_mem_value(&mem, idx+1, c);
                println!("Output \t {}", val1);
                idx = idx +2;
                //println!("step\t{}", "***");
                //map_mem(&mem);
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
    mem
}

#[allow(dead_code)]
pub fn calc() -> i32 {
    let data = read_file();
    let mem = create_computer_mem(data);
    run_computer(mem);
    0
}

#[test]
fn test_parse_instruction() {
    assert_eq!(parse_instruction(1902), (0, 1, 9, 2));
    assert_eq!(parse_instruction(1002), (0, 1, 0, 2));

}

#[test]
fn test_parse_part1() {
    assert_eq!(parse_instruction(1902), (0, 1, 9, 2));
    assert_eq!(parse_instruction(1002), (0, 1, 0, 2));

}