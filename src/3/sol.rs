use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn double_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {

    let mut grid = grid;

    for row in 0..grid.len() {
        let line = grid[row].clone();
        &grid[row].extend(line);
    }
    return grid;
        
}

fn get_trees(right: i32, down: i32) -> i32 {

    let filename = "./src/3/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut trees = 0;
    let mut row = 0;
    let mut col = 0;
    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap(); 
        let line_arr = line.chars().collect::<Vec<char>>();
        grid.push(line_arr);
    }

    while row < grid.len() {
        // println!("{}, {}, {}", row, grid.len(), grid[row].len());
        if col >= grid[row].len(){
            grid = double_grid(grid);
        }
        if grid[row][col] == '#' {
            trees += 1
        }
        col += right as usize;
        row += down as usize;

    }
    return trees;

}

fn part1(){
    let trees = get_trees(3, 1);
    println!("{} trees", trees);

}

fn part2(){
    let a = get_trees(1, 1);
    let b = get_trees(3, 1);
    let c = get_trees(5, 1);
    let d = get_trees(7, 1);
    let e = get_trees(1, 2);

    println!("values is {}", a * b * c * d * e);

}


fn main() {

    part1();
    part2();

}
