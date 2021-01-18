use std::collections::{HashSet, HashMap};
use std::io::{BufRead, BufReader};
use std::fs::File;

struct Passport {
    pass_map: HashMap<String, bool>

}

impl Passport {
    fn new() -> Passport {
        let mut pass_map = HashMap::new();
        pass_map.insert("byr".to_string(), false);
        pass_map.insert("iyr".to_string(), false);
        pass_map.insert("eyr".to_string(), false);
        pass_map.insert("hgt".to_string(), false);
        pass_map.insert("hcl".to_string(), false);
        pass_map.insert("ecl".to_string(), false);
        pass_map.insert("pid".to_string(), false);
    
        Passport{pass_map}
    }
    fn check_kv(&mut self, k: String, v: String){
        // let mut pass_map = HashMap::<String, bool>::new();
        if k == "byr"{
            let year = v.parse::<i32>().unwrap();
            if year <= 2002 && year >= 1920{
                *self.pass_map.entry(k).or_insert(false) = true;
            }
        } else if k == "iyr"{
            let year = v.parse::<i32>().unwrap();
            if year <= 2020 && year >= 2010{
                *self.pass_map.entry(k).or_insert(false) = true;
            }
        } else if k == "eyr"{
            let year = v.parse::<i32>().unwrap();
            if year <= 2030 && year >= 2020{
                *self.pass_map.entry(k).or_insert(false) = true;
            }
        } else if k == "hgt"{
            let num = &v[..(v.len()-2)];
            if v.contains("cm"){
                let h = num.parse::<i32>().unwrap();
                if h <= 193 && h >= 150{
                    *self.pass_map.entry(k).or_insert(false) = true;
                }
            }  else if v.contains("in"){
                let h = num.parse::<i32>().unwrap();
                if h <= 76 && h >= 59{
                    *self.pass_map.entry(k).or_insert(false) = true;
                }
            }
        } else if k == "hcl"{
            let val_set: HashSet<char> = vec!['0','1','2','3','4','5','6','7','8','9',
                                              'a','b','c','d','e','f'].into_iter().collect();
            if v.chars().nth(0).unwrap() == '#'{
                for i in 1..v.len(){
                    if !val_set.contains(&v.chars().nth(i).unwrap()){
                        break;
                    }
                    if i == v.len() - 1{
                        *self.pass_map.entry(k.clone()).or_insert(false) = true;
                    }
                }
            }
        } else if k == "ecl"{
            let val_set: HashSet<String> = vec!["amb".to_string(), "blu".to_string(), "brn".to_string(), "gry".to_string(), "grn".to_string(), "hzl".to_string(), "oth".to_string()].into_iter().collect();
            if val_set.contains(&v){
                *self.pass_map.entry(k).or_insert(false) = true;
            }
        } else if k == "pid"{
            if v.len() == 9{
                let int_val = match v.parse::<i32>(){
                    Ok(int_val) => *self.pass_map.entry(k).or_insert(false) = true,
                    Err(e) => println!("not a number: {}", e)
                };
            }
        }
    }
    fn get_validation(self)-> bool{
        for (_,v) in self.pass_map{
            if !v {
                return false;
            }
        }
        return true;
    }
}


fn part1(){
    let filename = "./src/4/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut valid = 0;
    let mut pass_keys = HashSet::new();
    pass_keys.insert("byr");
    pass_keys.insert("iyr");
    pass_keys.insert("eyr");
    pass_keys.insert("hgt");
    pass_keys.insert("hcl");
    pass_keys.insert("ecl");
    pass_keys.insert("pid");

    let mut temp_keys = HashSet::<String>::new();

    for line in reader.lines() {
        let line = line.unwrap(); 
        // println!("{}, len {}", line, line.len());
        let line_vec = line.split(" ").collect::<Vec<&str>>();
        // println!("{:?}", line_vec);
        if line.len() == 0 {
            if temp_keys.len() == pass_keys.len(){
                valid += 1;
                // println!("{:?} valid", temp_keys);
            } 
            temp_keys = HashSet::<String>::new();
        } else {
            for kv in line_vec {
                let kv_vec = kv.split(':').collect::<Vec<&str>>();
                if pass_keys.contains(kv_vec[0])  {
                    temp_keys.insert(kv_vec[0].to_string());
                }    
            }
        }
    }
    // check last entry outside loop
    if temp_keys.len() == pass_keys.len(){
        valid += 1;
    } 
    println!("{} valid entries", valid);

}

fn part2(){
    let filename = "./src/4/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut valid = 0;
    let mut passport = Passport::new();


    for line in reader.lines() {
        let line = line.unwrap(); 
        let line_vec = line.split(" ").collect::<Vec<&str>>();
        if line.len() == 0 {
            if passport.get_validation(){
                valid += 1;
            } 
            passport = Passport::new();
        } else {
            for kv in line_vec {
                let kv_vec = kv.split(':').collect::<Vec<&str>>();
                passport.check_kv(kv_vec[0].to_string(), kv_vec[1].to_string());
            }
        }
    }
    // check last entry outside loop
    if passport.get_validation(){
        valid += 1;
    } 

    println!("{} valid entries", valid);

}

fn main() {

    part1();
    part2();

}
