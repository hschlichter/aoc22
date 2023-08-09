use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Result},
    ops::RangeInclusive,
    path::Path,
};

fn is_range_contain(left: RangeInclusive<i32>, right: RangeInclusive<i32>) -> bool {
    let mut set_left = HashSet::new();
    left.into_iter().for_each(|i| {
        set_left.insert(i);
    });

    let mut set_right = HashSet::new();
    right.into_iter().for_each(|i| {
        set_right.insert(i);
    });

    // set_left.is_superset(&set_right) || set_right.is_superset(&set_left)
    set_left.intersection(&set_right).count() > 0 //is_superset(&set_right) || set_right.is_superset(&set_left)
}

fn string_to_range(str: &str) -> RangeInclusive<i32> {
    let split: Vec<&str> = str.split("-").collect();
    RangeInclusive::new(
        split[0].parse::<i32>().unwrap(),
        split[1].parse::<i32>().unwrap(),
    )
}

fn main() -> Result<()> {
    let path = Path::new("./bin/day4/input");
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut superset_count = 0;
    for l in lines {
        let line = l.unwrap();

        let split: Vec<&str> = line.split(",").collect();
        if let [left, right] = split.as_slice() {
            let left_range = string_to_range(left);
            let right_range = string_to_range(right);

            if is_range_contain(left_range, right_range) {
                superset_count += 1;
            }
        }
    }

    println!("{}", superset_count);

    Ok(())
}
