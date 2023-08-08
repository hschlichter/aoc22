// Day 8

use std::{path::Path, fs::File, io::{self, BufReader, BufRead}};

fn is_visible_edges(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let top_row = x == 0;
    let left_column = y == 0;
    let bottom_row = y == grid.len() - 1;
    let right_column = x == grid[y].len() - 1;
    if top_row || bottom_row || left_column || right_column {
        return true;
    }

    false
}

fn is_visible_inner_left_right(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let num = &grid[y][x];
    let left = grid[y].iter().take(x).fold(true, |acc, e| acc && num > e);
    let right = grid[y].iter().rev().take(grid[y].len() - x - 1).fold(true, |acc, e| acc && num > e);

    if left || right {
        return true;
    }

    false
}

fn is_visible_inner_top_bottom(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let num = &grid[y][x];
    let top = (0..y).map(|n| &grid[n][x]).fold(true, |acc, e| acc && num > e);
    let bottom = (y + 1..grid.len()).rev().map(|n| &grid[n][x]).fold(true, |acc, e| acc && num > e);

    if top || bottom {
        return true;
    }

    false
}

fn is_visible(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    is_visible_edges(grid, x, y) || is_visible_inner_left_right(grid, x, y) || is_visible_inner_top_bottom(grid, x, y)
}

fn find_num_tree_top(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    if y == 0 {
        return 0;
    }

    let num = &grid[y][x];
    let trees: Vec<&u32> = (0..y).map(|n| &grid[n][x]).rev().collect();
    let tree_visible_num = trees.iter().take_while(|n| **n < num).count();

    if tree_visible_num == trees.len() {
        return tree_visible_num;
    }

    tree_visible_num + 1
}

fn find_num_tree_bottom(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    if y == grid.len() - 1 {
        return 0;
    }

    let num = &grid[y][x];
    let trees: Vec<&u32> = (y + 1..grid.len()).map(|n| &grid[n][x]).collect();
    let tree_visible_num = trees.iter().take_while(|n| **n < num).count();

    if tree_visible_num == trees.len() {
        return tree_visible_num;
    }

    tree_visible_num + 1
}

fn find_num_tree_left(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    if x == 0 {
        return 0;
    }

    let num = &grid[y][x];
    let trees: Vec<&u32> = grid[y].iter().take(x).rev().collect();
    let tree_visible_num = trees.iter().take_while(|n| **n < num).count();

    if tree_visible_num == trees.len() {
        return tree_visible_num;
    }

    tree_visible_num + 1
}

fn find_num_tree_right(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    if x == grid[y].len() - 1 {
        return 0;
    }

    let num = &grid[y][x];
    let trees: Vec<&u32> = grid[y].iter().skip(x + 1).collect();
    let tree_visible_num = trees.iter().take_while(|n| **n < num).count();

    if tree_visible_num == trees.len() {
        return tree_visible_num;
    }

    tree_visible_num + 1
}

fn scenic_score(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let top = find_num_tree_top(grid, x, y);
    let bottom = find_num_tree_bottom(grid, x, y);
    let left = find_num_tree_left(grid, x, y);
    let right = find_num_tree_right(grid, x, y);
    top * bottom * left * right
}

fn load_grid(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for l in lines {
        let mut row: Vec<u32> = Vec::new();
        for c in l.chars() {
            if let Some(digit) = c.to_digit(10) {
                row.push(digit);
            }
        }
        grid.push(row);
    }

    grid
}

fn main() -> io::Result<()> {
    let path = Path::new("./bin/day8/input");
    let file = File::open(path)?;
    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|l| l.ok()).collect();

    let grid = load_grid(&lines);
    
    let mut highest_scenic_score = 0;
    let mut visible_count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_visible(&grid, i, j) {
                visible_count += 1;
            }

            let score = scenic_score(&grid, i, j);
            if score > highest_scenic_score {
                highest_scenic_score = score;
            }
        }
    }
    println!("visible trees: {}", visible_count);
    println!("highest scenic score: {}", highest_scenic_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

const LINES: &str = r#"30373
25512
65332
33549
35390
"#;

    #[test]
    fn verify_grid_loading() {
        let lines = LINES.lines().map(String::from).collect();
        let grid = load_grid(&lines);
        assert_eq!(grid, vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
    }

    #[test]
    fn verify_find_num_tree_top() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert_eq!(find_num_tree_top(&grid, 0, 0), 0);
        assert_eq!(find_num_tree_top(&grid, 0, 1), 1);
        assert_eq!(find_num_tree_top(&grid, 0, 2), 2);
        assert_eq!(find_num_tree_top(&grid, 0, 3), 1);
        assert_eq!(find_num_tree_top(&grid, 0, 4), 1);

        assert_eq!(find_num_tree_top(&grid, 1, 0), 0);
        assert_eq!(find_num_tree_top(&grid, 1, 1), 1);
        assert_eq!(find_num_tree_top(&grid, 1, 2), 1);
        assert_eq!(find_num_tree_top(&grid, 1, 3), 1);
        assert_eq!(find_num_tree_top(&grid, 1, 4), 2);

        assert_eq!(find_num_tree_top(&grid, 2, 0), 0);
        assert_eq!(find_num_tree_top(&grid, 2, 1), 1);
        assert_eq!(find_num_tree_top(&grid, 2, 2), 1);
        assert_eq!(find_num_tree_top(&grid, 2, 3), 2);
        assert_eq!(find_num_tree_top(&grid, 2, 4), 1);

        assert_eq!(find_num_tree_top(&grid, 3, 0), 0);
        assert_eq!(find_num_tree_top(&grid, 3, 1), 1);
        assert_eq!(find_num_tree_top(&grid, 3, 2), 2);
        assert_eq!(find_num_tree_top(&grid, 3, 3), 3);
        assert_eq!(find_num_tree_top(&grid, 3, 4), 4);

        assert_eq!(find_num_tree_top(&grid, 4, 0), 0);
        assert_eq!(find_num_tree_top(&grid, 4, 1), 1);
        assert_eq!(find_num_tree_top(&grid, 4, 2), 1);
        assert_eq!(find_num_tree_top(&grid, 4, 3), 3);
        assert_eq!(find_num_tree_top(&grid, 4, 4), 1);
    }

    #[test]
    fn verify_find_num_tree_bottom() {
        let grid = vec![
            vec![3],
            vec![4],
            vec![3],
            vec![5],
            vec![3],
        ];
        assert_eq!(find_num_tree_bottom(&grid, 0, 0), 1);
        assert_eq!(find_num_tree_bottom(&grid, 0, 1), 2);
        assert_eq!(find_num_tree_bottom(&grid, 0, 2), 1);
        assert_eq!(find_num_tree_bottom(&grid, 0, 3), 1);
        assert_eq!(find_num_tree_bottom(&grid, 0, 4), 0);
    }

    #[test]
    fn verify_find_num_tree_left() {
        let grid = vec![
            vec![2, 5, 5, 1, 2],
        ];
        assert_eq!(find_num_tree_left(&grid, 0, 0), 0);
        assert_eq!(find_num_tree_left(&grid, 1, 0), 1);
        assert_eq!(find_num_tree_left(&grid, 2, 0), 1);
        assert_eq!(find_num_tree_left(&grid, 3, 0), 1);
        assert_eq!(find_num_tree_left(&grid, 4, 0), 2);
    }

    #[test]
    fn verify_find_num_tree_right() {
        let grid = vec![
            vec![2, 5, 5, 1, 2],
        ];
        assert_eq!(find_num_tree_right(&grid, 0, 0), 1);
        assert_eq!(find_num_tree_right(&grid, 1, 0), 1);
        assert_eq!(find_num_tree_right(&grid, 2, 0), 2);
        assert_eq!(find_num_tree_right(&grid, 3, 0), 1);
        assert_eq!(find_num_tree_right(&grid, 4, 0), 0);
    }

    #[test]
    fn verify_scenic_score() {
        let lines = LINES.lines().map(String::from).collect();
        let grid = load_grid(&lines);
        assert_eq!(scenic_score(&grid, 2, 1), 4);
        assert_eq!(scenic_score(&grid, 2, 3), 8);
    }

    #[test]
    fn verify_visible_from_edges() {
        let grid = vec![
            vec![2, 5, 5],
            vec![6, 5, 3],
            vec![3, 3, 5],
        ];
        assert!(is_visible_edges(&grid, 0, 0));
        assert!(is_visible_edges(&grid, 1, 0));
        assert!(is_visible_edges(&grid, 2, 0));
        assert!(is_visible_edges(&grid, 0, 1));
        assert!(!is_visible_edges(&grid, 1, 1));
        assert!(is_visible_edges(&grid, 2, 1));
        assert!(is_visible_edges(&grid, 0, 2));
        assert!(is_visible_edges(&grid, 1, 2));
        assert!(is_visible_edges(&grid, 2, 2));
    }

    #[test]
    fn verify_visible_from_left_or_right_0() {
        let grid = vec![
            vec![2, 5, 5, 1, 2],
        ];
        assert!(is_visible_inner_left_right(&grid, 1, 0));
        assert!(is_visible_inner_left_right(&grid, 2, 0));
        assert!(!is_visible_inner_left_right(&grid, 3, 0));
    }

    #[test]
    fn verify_visible_from_left_or_right_1() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
        ];
        assert!(!is_visible_inner_left_right(&grid, 1, 0));
        assert!(!is_visible_inner_left_right(&grid, 2, 0));
        assert!(is_visible_inner_left_right(&grid, 3, 0));
    }

    #[test]
    fn verify_visible_from_top_or_bottom_0() {
        let grid = vec![
            vec![3],
            vec![2],
            vec![6],
            vec![3],
            vec![3],
        ];
        assert!(!is_visible_inner_top_bottom(&grid, 0, 1));
        assert!(is_visible_inner_top_bottom(&grid, 0, 2));
        assert!(!is_visible_inner_top_bottom(&grid, 0, 3));
    }

    #[test]
    fn verify_visible_from_top_or_bottom_1() {
        let grid = vec![
            vec![7],
            vec![1],
            vec![3],
            vec![4],
            vec![9],
        ];
        assert!(!is_visible_inner_top_bottom(&grid, 0, 1));
        assert!(!is_visible_inner_top_bottom(&grid, 0, 2));
        assert!(!is_visible_inner_top_bottom(&grid, 0, 3));
    }

    #[test]
    fn verify_visible_from_top_or_bottom_2() {
        let grid = vec![
            vec![3],
            vec![5],
            vec![3],
            vec![5],
            vec![3],
        ];
        assert!(is_visible_inner_top_bottom(&grid, 0, 1));
        assert!(!is_visible_inner_top_bottom(&grid, 0, 2));
        assert!(is_visible_inner_top_bottom(&grid, 0, 3));
    }

    #[test]
    fn verify_expected_visibility() {
        let lines = LINES.lines().map(String::from).collect();
        let grid = load_grid(&lines);

        let expected = vec![
            vec![true, true, true, true, true],
            vec![true, true, true, false, true],
            vec![true, true, false, true, true],
            vec![true, false, true, false, true],
            vec![true, true, true, true, true],
        ];

        assert_eq!(grid.len(), expected.len());
        for i in 0..grid.len() {
            assert_eq!(grid[i].len(), expected[i].len());
            for j in 0..grid[i].len() {
                let val = expected.get(i).unwrap().get(j).unwrap();
                assert_eq!(is_visible(&grid, i, j), *val);
            }
        }
    }

    #[test]
    fn verify_total_visible() {
        let lines = LINES.lines().map(String::from).collect();
        let grid = load_grid(&lines);

        let mut visible_count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let v = is_visible(&grid, i, j);
                if v {
                    visible_count += 1;
                }
            }
        }

        assert_eq!(visible_count, 21);
    }
}
