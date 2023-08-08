use std::{path::Path, fs::File, io::{self, BufReader, BufRead}, collections::HashSet};

fn is_chars_unique(buf: Vec<char>) -> bool {
    let mut freq: HashSet<char> = HashSet::new();
    for c in buf.iter() {
        freq.insert(*c);
    }

    buf.len() == freq.len()
}

fn detect_sequence_marker(input: &String, distinct_num: usize) -> u32 {
    for i in 0..input.len() - distinct_num - 1 {
        let ss = &input[i..i + distinct_num];
        if is_chars_unique(ss.chars().collect()) {
            return (i + distinct_num) as u32;
        }
    }

    0
}

fn main() -> io::Result<()> {
    let path = Path::new("./bin/day6/input");
    let file = File::open(&path)?;
    let lines = BufReader::new(&file).lines();

    for l in lines {
        let line = l.unwrap();
        let marker_4 = detect_sequence_marker(&line, 4);
        println!("Part 1 - marker: {}", marker_4);

        let marker_14 = detect_sequence_marker(&line, 14);
        println!("Part 2 - marker: {}", marker_14);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chars_is_unique() {
        let buf = vec!['b', 'a', 'd', 'e'];
        assert!(is_chars_unique(buf));
    }
 
    #[test]
    fn test_chars_is_not_unique_start_inside() {
        let buf = vec!['b', 'a', 'b', 'e'];
        assert!(!is_chars_unique(buf));
    }

    #[test]
    fn test_chars_is_not_unique_inside_end() {
        let buf = vec!['b', 'a', 'g', 'a'];
        assert!(!is_chars_unique(buf));
    }

    #[test]
    fn test_chars_is_not_unique_start_end() {
        let buf = vec!['b', 'a', 'f', 'b'];
        assert!(!is_chars_unique(buf));
    }

    #[test]
    fn test_chars_is_not_unique_inside_inside() {
        let buf = vec!['b', 'f', 'f', 'e'];
        assert!(!is_chars_unique(buf));
    }

    #[test]
    fn test_detect_sequence_finds_marker_0() {
        assert_eq!(detect_sequence_marker(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 4), 5);
    }

    #[test]
    fn test_detect_sequence_finds_marker_1() {
        assert_eq!(detect_sequence_marker(&String::from("nppdvjthqldpwncqszvftbrmjlhg"), 4), 6);
    }

    #[test]
    fn test_detect_sequence_finds_marker_2() {
        assert_eq!(detect_sequence_marker(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 4), 10);
    }

    #[test]
    fn test_detect_sequence_finds_marker_3() {
        assert_eq!(detect_sequence_marker(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 4), 11);
    }
}
