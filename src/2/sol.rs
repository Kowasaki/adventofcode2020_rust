use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

struct Counter {
    input_str: String,
    count_map: HashMap<char, i32>

}

impl Counter {
    fn new(input_str: String) -> Counter {
        Counter{input_str: input_str.clone(), count_map: Counter::get_count(input_str.clone())}
    }

    fn get_count(input_str: String) -> HashMap<char, i32> {
        let mut count_map = HashMap::new();
        for ch in input_str.chars(){
            *count_map.entry(ch).or_insert(0) += 1;
        }
        return count_map;
    }
}

fn main() {
    let filename = "./src/2/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut valid = 0;

    // part 1
    // for line in reader.lines() {
    //     let line = line.unwrap(); 
    //     let line_vec = line.split(" ").collect::<Vec<&str>>();
    //     let (range, ch, password) = (line_vec[0], line_vec[1].as_bytes()[0] as char, line_vec[2]);
    //     let range_vec = range.split("-").collect::<Vec<&str>>();
    //     let (lbound, ubound) = (range_vec[0].parse::<i32>().unwrap(), range_vec[1].parse::<i32>().unwrap());
        
    //     let c_counter = Counter::new(password.to_string());
    //     let c_map = c_counter.count_map;

    //     if !c_map.contains_key(&ch){
    //         continue
    //     } else if c_map.get(&ch).unwrap() > &ubound || c_map.get(&ch).unwrap() < &lbound{
    //         continue
    //     } else {
    //         valid += 1
    //     }

    //     println!("{} valid entries", valid);

    // }


    // part 2 
    for line in reader.lines() {
        let line = line.unwrap(); 
        let line_vec = line.split(" ").collect::<Vec<&str>>();
        let (range, ch, password) = (line_vec[0], line_vec[1].as_bytes()[0] as char, line_vec[2]);
        let range_vec = range.split("-").collect::<Vec<&str>>();
        let (pos1, pos2) = (range_vec[0].parse::<usize>().unwrap(), range_vec[1].parse::<usize>().unwrap());
        let ipos1 = pos1 -1;
        let ipos2 = pos2 -1;

        let c1 = password.as_bytes()[ipos1] as char;
        let c2 = password.as_bytes()[ipos2] as char;

        if c1 == ch || c2 == ch{
            if c1 != c2 {
                valid += 1
            }
        }
    }
    println!("{} valid entries", valid);

}
