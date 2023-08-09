use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("./bin/day1/input");

    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut calories: Vec<i32> = Vec::new();
    let mut elve: Vec<i32> = Vec::new();

    for line in lines {
        let l = line.expect("fubar");
        match l.is_empty() {
            true => {
                let sum: i32 = elve.iter().sum();
                elve.clear();
                calories.push(sum);
            }
            false => elve.push(l.parse::<i32>().unwrap()),
        }
    }
    calories.sort_by(|a, b| b.cmp(a));

    // Part 1
    let highest = calories.iter().next().unwrap();
    println!("Part 1 - highest: {}", highest);

    // Part 2
    let top3_sum: i32 = calories.iter().take(3).sum();
    println!("Part 2 - top3_sum: {}", top3_sum);
}

// Solution by ChatGPT!
// use std::fs::File;
// use std::io::{self, BufRead};
//
// fn main() -> io::Result<()> {
//     let file = File::open("input.txt")?;
//     let reader = io::BufReader::new(file);
//
//     let mut max_calories = 0;
//     let mut current_calories = 0;
//
//     for line in reader.lines() {
//         let line = line?;
//         if line.is_empty() {
//             if current_calories > max_calories {
//                 max_calories = current_calories;
//             }
//             current_calories = 0;
//         } else {
//             let calories: u32 = line.parse().unwrap();
//             current_calories += calories;
//         }
//     }
//
//     if current_calories > max_calories {
//         max_calories = current_calories;
//     }
//
//     println!("The maximum calories carried by an Elf is: {}", max_calories);
//
//     Ok(())
// }
