use std::{str::FromStr, io::{self, BufReader, BufRead}, path::Path, fs::File, vec};

#[derive(Debug)]
struct Instruction {
    amount: u32,
    from: u32,
    to: u32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() > 6 {
            return Err(format!("To many tokens {}", "hello"))
        }

        assert_eq!(tokens[0], "move");
        assert_eq!(tokens[2], "from");
        assert_eq!(tokens[4], "to");

        let amount = tokens[1].parse::<u32>().map_err(|e| e.to_string())?;
        let from = tokens[3].parse::<u32>().map_err(|e| e.to_string())?;
        let to = tokens[5].parse::<u32>().map_err(|e| e.to_string())?;
        Ok(Instruction { amount, from, to })
    }
}

fn process_instructions_part1(stacks: &mut Vec<Vec<char>>, instr: &Instruction) {
    for _ in 0..instr.amount {
        let stack_from: &mut Vec<char> = &mut stacks[(instr.from - 1) as usize];
        let last = stack_from.pop();

        let stack_to: &mut Vec<char> = &mut stacks[(instr.to - 1) as usize];
        if let Some(val) = last {
            stack_to.push(val);
        }
    }
}

fn process_instructions_part2(stacks: &mut Vec<Vec<char>>, instr: &Instruction) {
    let stack_from: &mut Vec<char> = &mut stacks[(instr.from - 1) as usize];
    let mut stack_split = stack_from.split_off(stack_from.len() - instr.amount as usize);
    let stack_to: &mut Vec<char> = &mut stacks[(instr.to - 1) as usize];
    stack_to.append(&mut stack_split);
}

fn main() -> io::Result<()> {
    let mut stacks: Vec<Vec<char>> = vec![
        vec!['J', 'H', 'P', 'M', 'S', 'F', 'N', 'V'],
        vec!['S', 'R', 'L', 'M', 'J', 'D', 'Q'],
        vec!['N', 'Q', 'D', 'H', 'C', 'S', 'W', 'B'],
        vec!['R', 'S', 'C', 'L'],
        vec!['M', 'V', 'T', 'P', 'F', 'B'],
        vec!['T', 'R', 'Q', 'N', 'C'],
        vec!['G', 'V', 'R'],
        vec!['C', 'Z', 'S', 'P', 'D', 'L', 'R'],
        vec!['D', 'S', 'J', 'V', 'G', 'P', 'B', 'F'],
    ];

    let path = Path::new("./bin/day5/input");
    let file = File::open(&path)?;
    let lines = BufReader::new(&file).lines().skip(10);

    for l in lines {
        let line = l.unwrap();
        let instr = line.parse::<Instruction>().unwrap();
        process_instructions_part1(&mut stacks, &instr);
    }

    let mut out: String = String::new();
    for s in stacks {
        if let Some(c) = s.last() {
            out.push(*c);
        }
    }
    println!("{}", out);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let i: Instruction = "move 1 from 8 to 4".parse().unwrap();
        assert_eq!(i.amount, 1);
        assert_eq!(i.from, 8);
        assert_eq!(i.to, 4);
    }

    #[test]
    fn test_instruction_processing_part1() {
        let mut stacks: Vec<Vec<char>> = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];

        let instr = vec![
            "move 1 from 2 to 1".parse::<Instruction>().unwrap(),
            "move 3 from 1 to 3".parse::<Instruction>().unwrap(),
            "move 2 from 2 to 1".parse::<Instruction>().unwrap(),
            "move 1 from 1 to 2".parse::<Instruction>().unwrap(),
        ];

        for i in instr {
            process_instructions_part1(&mut stacks, &i);
        }

        let mut out: String = String::new();
        for s in stacks {
            if let Some(c) = s.last() {
                out.push(*c);
            }
        }
        assert_eq!(out, "CMZ");
    }

    #[test]
    fn test_instruction_processing_part2() {
        let mut stacks: Vec<Vec<char>> = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];

        let instr = vec![
            "move 1 from 2 to 1".parse::<Instruction>().unwrap(),
            "move 3 from 1 to 3".parse::<Instruction>().unwrap(),
            "move 2 from 2 to 1".parse::<Instruction>().unwrap(),
            "move 1 from 1 to 2".parse::<Instruction>().unwrap(),
        ];

        for i in instr {
            process_instructions_part2(&mut stacks, &i);
        }

        let mut out: String = String::new();
        for s in stacks {
            if let Some(c) = s.last() {
                out.push(*c);
            }
        }
        assert_eq!(out, "MCD");
    }
}
