
use std::fs;
use std::collections::HashMap;

pub fn read_file() -> Vec<String> {
    println!("Reading file?");

    let contents = fs::read_to_string("problem2.txt")
        .expect("Something went wrong reading the file");
    
    //println!("With text:\n{}", contents);
    contents.split(",").map(ToOwned::to_owned).collect()
}

fn create_computer_mem(data : Vec<String>) -> HashMap<i32, i32>{
    let mut mem = HashMap::new();
    for (pos, e) in data.into_iter().enumerate(){
        println!("Pos.data:\t{}.{}", pos, e);
        let v = e.parse::<i32>().unwrap();
        let pos32 = pos.to_string().parse::<i32>().unwrap();
        mem.insert(pos32, v);
    }
    
    mem
}

fn map_mem(mem : &HashMap<i32, i32>) {
    for (pos, value) in mem {
        println!("\t{}: {}\t", pos, value);
    }
} 

fn run_computer(mut mem : HashMap<i32, i32>) -> HashMap<i32, i32> {
    let mut idx : i32 = 0;
    while idx >= 0 {
        let opcode = mem.get(&idx).unwrap_or(&99);
        match opcode {
            1 => {
                let idx1 = mem.get(&(idx + 1)).unwrap();
                let val1 = mem.get(&idx1).unwrap();
                let idx2 = mem.get(&(idx + 2)).unwrap();
                let val2 = mem.get(&idx2).unwrap();
                let result = val1 + val2;
                let location = mem.get(&(idx + 3)).unwrap();
                println!("step: {} + {} = {}\t{}",val1, val2, result, "+++");
                mem.insert(*location, result);
                map_mem(&mem);
                idx = idx +4;
            },
            2 => {
                let idx1 = mem.get(&(idx + 1)).unwrap();
                let val1 = mem.get(&idx1).unwrap();
                let idx2 = mem.get(&(idx + 2)).unwrap();
                let val2 = mem.get(&idx2).unwrap();
                let result = val1 * val2;
                let location = mem.get(&(idx + 3)).unwrap();
                mem.insert(*location, result);
                idx = idx +4;
                println!("step\t{}", "***");
                map_mem(&mem);
            },
            99 => {
                println!("result?\t{}", mem.get(&0).unwrap());
                idx = -1;
            },
            _ => {
                
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