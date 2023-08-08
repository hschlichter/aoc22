use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum GameError {
    ShapeParseError,
    OutcomeParseError,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Shape {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissor),
            _ => Err(GameError::ShapeParseError),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Outcome {
    Win,
    Draw,
    Loss,
}

impl FromStr for Outcome {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(GameError::OutcomeParseError),
        }
    }
}

pub struct Game {
    left: Shape,
    right: Shape,
}

impl Game {
    pub fn new(l: &str, r: &str) -> Result<Self, GameError> {
        let left = l.parse()?;
        let right = r.parse()?;
        
        Ok(Self {
            left,
            right,
        })
    }

    pub fn new_v2(l: &str, r: &str) -> Result<Self, GameError> {
        let left = l.parse::<Shape>()?;
        let outcome = r.parse::<Outcome>()?;

        let right = match (left, outcome) {
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Rock, Outcome::Loss) => Shape::Scissor,
            (Shape::Paper, Outcome::Win) => Shape::Scissor,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Paper, Outcome::Loss) => Shape::Rock,
            (Shape::Scissor, Outcome::Win) => Shape::Rock,
            (Shape::Scissor, Outcome::Draw) => Shape::Scissor,
            (Shape::Scissor, Outcome::Loss) => Shape::Paper,
        };

        Ok(Self {
            left,
            right,
        })
    }

    // Evaluates the game according to the right player
    pub fn eval_game_right(&self) -> i32 {
        match (&self.right, &self.left) {
            (Shape::Rock, Shape::Scissor) | (Shape::Scissor, Shape::Paper) | (Shape::Paper, Shape::Rock) => 6,
            (Shape::Rock, Shape::Rock) | (Shape::Paper, Shape::Paper) | (Shape::Scissor, Shape::Scissor) => 3,
            _ => 0,
        }
    }

    // Points for shape selection of the right player
    pub fn eval_shape_right(&self) -> i32 {
        match &self.right {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
}

fn main() {
    let path = Path::new("./bin/day2/input");
   
    let games: Vec<Game> = BufReader::new(File::open(&path).unwrap())
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let v: Vec<&str> = l.split(' ').collect();
            Game::new(v[0], v[1]).unwrap()
        })
        .collect();

    let total_points: i32 = games.iter().map(|g| g.eval_game_right() + g.eval_shape_right()).sum();
    println!("Part 1 - points: {}", total_points);

    let games_v2: Vec<Game> = BufReader::new(File::open(&path).unwrap())
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let v: Vec<&str> = l.split(' ').collect();
            Game::new_v2(v[0], v[1]).unwrap()
        })
        .collect();

    let total_points_v2: i32 = games_v2.iter().map(|g| g.eval_game_right() + g.eval_shape_right()).sum();
    println!("Part 2 - points: {}", total_points_v2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_shape() {
        assert_eq!("A".parse::<Shape>(), Ok(Shape::Rock));
        assert_eq!("B".parse::<Shape>(), Ok(Shape::Paper));
        assert_eq!("C".parse::<Shape>(), Ok(Shape::Scissor));
        assert_eq!("X".parse::<Shape>(), Ok(Shape::Rock));
        assert_eq!("Y".parse::<Shape>(), Ok(Shape::Paper));
        assert_eq!("Z".parse::<Shape>(), Ok(Shape::Scissor));
    }

    #[test]
    fn test_parse_invalid_shape() {
        assert!("D".parse::<Shape>().is_err());
        assert!("W".parse::<Shape>().is_err());
        assert!("123".parse::<Shape>().is_err());
        assert!("".parse::<Shape>().is_err());
    }

    #[test]
    fn outcome_from_str_valid_input() {
        assert_eq!("Z".parse::<Outcome>(), Ok(Outcome::Win));
        assert_eq!("Y".parse::<Outcome>(), Ok(Outcome::Draw));
        assert_eq!("X".parse::<Outcome>(), Ok(Outcome::Loss));
    }

    #[test]
    fn outcome_from_str_invalid_input() {
        assert!("A".parse::<Outcome>().is_err());
        assert!("B".parse::<Outcome>().is_err());
        assert!("C".parse::<Outcome>().is_err());
    }

    #[test]
    fn test_game_eval_game_right() {
        let data: [(&str, &str, i32); 9] = [
            ("A", "C", 0),
            ("B", "A", 0),
            ("C", "B", 0),
            ("A", "A", 3),
            ("B", "B", 3),
            ("C", "C", 3),
            ("A", "B", 6),
            ("B", "C", 6),
            ("C", "A", 6),
        ];
        for d in &data {
            let g = Game::new(d.0, d.1).unwrap();
            assert_eq!(g.eval_game_right(), d.2);
        }
    }

    #[test]
    fn test_game_eval_shape_right() {
        let data: [(&str, &str, i32); 9] = [
            ("A", "A", 1),
            ("B", "A", 1),
            ("C", "A", 1),
            ("A", "B", 2),
            ("B", "B", 2),
            ("C", "B", 2),
            ("A", "C", 3),
            ("B", "C", 3),
            ("C", "C", 3),
        ];
        for d in data {
            let g = Game::new(d.0, d.1).unwrap();
            assert_eq!(g.eval_shape_right(), d.2);
        }
    }

    #[test]
    fn test_new_v2_rock_win() {
        let game = Game::new_v2("A", "Z").unwrap();
        assert_eq!(game.left, Shape::Rock);
        assert_eq!(game.right, Shape::Paper);
    }

    #[test]
    fn test_new_v2_rock_draw() {
        let game = Game::new_v2("A", "Y").unwrap();
        assert_eq!(game.left, Shape::Rock);
        assert_eq!(game.right, Shape::Rock);
    }

    #[test]
    fn test_new_v2_rock_loss() {
        let game = Game::new_v2("A", "X").unwrap();
        assert_eq!(game.left, Shape::Rock);
        assert_eq!(game.right, Shape::Scissor);
    }

    #[test]
    fn test_new_v2_paper_win() {
        let game = Game::new_v2("B", "Z").unwrap();
        assert_eq!(game.left, Shape::Paper);
        assert_eq!(game.right, Shape::Scissor);
    }

    #[test]
    fn test_new_v2_paper_draw() {
        let game = Game::new_v2("B", "Y").unwrap();
        assert_eq!(game.left, Shape::Paper);
        assert_eq!(game.right, Shape::Paper);
    }

    #[test]
    fn test_new_v2_paper_loss() {
        let game = Game::new_v2("B", "X").unwrap();
        assert_eq!(game.left, Shape::Paper);
        assert_eq!(game.right, Shape::Rock);
    }

    #[test]
    fn test_new_v2_scissor_win() {
        let game = Game::new_v2("C", "Z").unwrap();
        assert_eq!(game.left, Shape::Scissor);
        assert_eq!(game.right, Shape::Rock);
    }

    #[test]
    fn test_new_v2_scissor_draw() {
        let game = Game::new_v2("C", "Y").unwrap();
        assert_eq!(game.left, Shape::Scissor);
        assert_eq!(game.right, Shape::Scissor);
    }

    #[test]
    fn test_new_v2_scissor_loss() {
        let game = Game::new_v2("C", "X").unwrap();
        assert_eq!(game.left, Shape::Scissor);
        assert_eq!(game.right, Shape::Paper);
    }
}
