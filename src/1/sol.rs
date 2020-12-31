use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let filename = "./src/1/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);


    // part 1
    // let mut match_set = HashSet::new();
    // for line in reader.lines() {
    //     let line = line.unwrap(); // Ignore errors.
    //     let val1 = line.parse::<i32>().unwrap();
    //     let val2 = 2020 - val1;
    //     if match_set.contains(&val2) {
    //         println!("answer is {}", val1 * val2);
    //     } else {
    //         match_set.insert(val1);
    //     }
    // }

    // part 2

    let mut expense_list = Vec::new(); 
    
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        
        let val = line.parse::<i32>().unwrap();
        expense_list.push(val);

    }

    expense_list.sort();

    // println!("{:?}",expense_list);
    for i in 0..expense_list.len()-1{
        let cval = expense_list[i];
        let mut l_pointer = i+1;
        let mut r_pointer = expense_list.len() - 1;
    
        while r_pointer > l_pointer{
            let res = cval + expense_list[r_pointer] +  expense_list[l_pointer];
            if res == 2020{
                println!("sums are {}, {}, {}", cval, expense_list[r_pointer],  expense_list[l_pointer]);
                println!("answer is {}", cval * expense_list[r_pointer] *  expense_list[l_pointer]);
                break
            } else if res > 2020 {
                r_pointer-=1;
            } else {
                l_pointer+=1;
            }
        }
    }
}
