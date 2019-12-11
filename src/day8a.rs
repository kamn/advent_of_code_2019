use std::fs;

fn read_file() -> String {
    println!("Reading file?");

    let contents = fs::read_to_string("day8.txt")
        .expect("Something went wrong reading the file");
    
    //println!("With text:\n{}", contents);
    contents.to_owned()
}

fn parse_input (contents : String) -> Vec<String> {
    let mut result : Vec<String> = contents.trim().split("").map(ToOwned::to_owned).collect();
    result.pop();
    result.drain(0..1);
    result
}

fn count_string (layer : Vec<String>, pattern : String)  -> i32 {
    let mut total = 0;
    for c in layer.into_iter() {
        if c == pattern {
            total  = total +1;
        }
    }
    total
}

fn process(raw_data : Vec<String>, height : usize, width : usize) -> i32 {
    let mut least_zero = 10000000;
    let mut calculated = -1;
    println!("size? {}",height*width);
    let iter  = raw_data.chunks(height*width);

    for layer in iter {
        let collected = layer.to_vec();
        println!("collected? {:?}",collected );
        println!("count? {:?}",collected.len() );

        let num_zeros = count_string(collected.clone(), "0".to_string());
        println!("zeros? {}",num_zeros );
        if num_zeros != 0 && num_zeros < least_zero {
            let num_ones = count_string(collected.clone(), "1".to_string());
            let num_twos = count_string(collected.clone(), "2".to_string());
            calculated = num_ones * num_twos;
            least_zero = num_zeros;
        }
    }
    calculated
}

#[allow(dead_code)]
pub fn calc() -> i32 {
    let data = read_file();
    let input = parse_input(data);
    process(input, 25, 6)
}




#[test]
fn test_day8a_count_str() {
    let example1 = "123456789012";
    let x : Vec<String> = example1.to_string().split("").map(ToOwned::to_owned).collect();
    assert_eq!(count_string(x.to_owned(), "1".to_string()), 2);
    assert_eq!(count_string(x.to_owned(), "5".to_string()), 1);

}

