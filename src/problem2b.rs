
use std::fs;
use std::collections::HashMap;

pub fn read_file() -> Vec<String> {

    let contents = fs::read_to_string("problem2.txt")
        .expect("Something went wrong reading the file");
    
    //println!("With text:\n{}", contents);
    contents.split(",").map(ToOwned::to_owned).collect()
}

fn create_computer_mem(data : Vec<String>) -> HashMap<i32, i32>{
    let mut mem = HashMap::new();
    for (pos, e) in data.into_iter().enumerate(){
        let v = e.parse::<i32>().unwrap();
        let pos32 = pos.to_string().parse::<i32>().unwrap();
        mem.insert(pos32, v);
    }
    
    mem
}


fn run_computer(mut mem : HashMap<i32, i32>) -> i32 {
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
                mem.insert(*location, result);
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
            },
            99 => {
                idx = -1;
            },
            _ => {
                
                idx = -1;
            }
        };
    }
    *mem.get(&0).unwrap()
}

#[allow(dead_code)]
pub fn calc() -> i32 {
    for x in 1..99 {
        for y in 1..99 {
            let data = read_file();
            let mut mem = create_computer_mem(data);
            mem.insert(1, x);
            mem.insert(2, y);
            let result = run_computer(mem);
            if result == 19690720 {
                println!("x.y:\t{}.{}", x, y);

            }
        }
    }
    0
}