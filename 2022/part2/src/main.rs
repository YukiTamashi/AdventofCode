use std::{fs, str::FromStr};

#[derive(Debug)]
struct PlayError;

#[derive(Debug)]
struct OutcomeError;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Play{
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl Play{
    fn result(self, second: Self) -> Outcome{
        if self == second{
            Outcome::Draw
        }
        else if second == (self as isize + 1).into(){
            Outcome::Win
        }
        else{
            Outcome::Loss
        }
    }

    fn from_outcome(self, outcome: Outcome) -> Self{
        match outcome{
            Outcome::Draw => self,
            Outcome::Loss => (self as isize + 2).into(),
            Outcome::Win => (self as isize + 1).into()
        }
    }
}

impl From<isize> for Play{
    fn from(mut a: isize) -> Self {
        if a > 3{
            a -= 3;
        }
        match a{
            1 => Play::Rock,
            2 => Play::Paper,
            3 => Play::Scissors,
            _ => panic!("Wrong value, shouldn't happen")
        }
    }
}

enum Outcome{
    Loss = 0,
    Draw = 3,
    Win = 6
}

impl From<(Play, Play)> for Outcome{
    fn from(a: (Play, Play)) -> Self {
        if a.0 == a.1{
            Outcome::Draw
        }
        else{
            todo!()
        }
    }
}

impl FromStr for Outcome{
    type Err = OutcomeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1{
            Err(OutcomeError)
        }
        else{
            match s{
                "X" => Ok(Outcome::Loss),
                "Y" => Ok(Outcome::Draw),
                "Z" => Ok(Outcome::Win),
                _ => Err(OutcomeError)
            }
        }
    }
}


impl FromStr for Play{
    type Err = PlayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1{
            Err(PlayError)
        }
        else{
            match s{
                "A" | "X" => Ok(Play::Rock),
                "B" | "Y" => Ok(Play::Paper),
                "C" | "Z" => Ok(Play::Scissors),
                _ => Err(PlayError)
            }
        }
    }
}

fn first_part(content: &str) -> isize{
    let mut score = 0;
    for line in content.lines(){
        let (opponent, yours) = plays_from_line(line);
        score += score_from_plays(opponent, yours);
    }
    score
}

fn second_part(content: &str) -> isize{
    let mut score = 0;
    for line in content.lines(){
        let (opponent, outcome) = outcome_from_line(line);
        let yours = opponent.from_outcome(outcome);
        score += score_from_plays(opponent, yours);
    }
    score
}

fn plays_from_line(line: &str) -> (Play, Play){
    let a: Vec<&str> = line.split_whitespace().collect();
    (Play::from_str(a[0]).unwrap(), Play::from_str(a[1]).unwrap())
}

fn outcome_from_line(line: &str) -> (Play, Outcome){
    let a: Vec<&str> = line.split_whitespace().collect();
    (Play::from_str(a[0]).unwrap(), Outcome::from_str(a[1]).unwrap())
}

fn score_from_plays(opponent: Play, yours: Play) -> isize{
    let mut score = 0;
    score += yours as isize;
    score += opponent.result(yours) as isize;
    score
}

fn main() {
    let content = fs::read_to_string("day2.in").unwrap();
    let score = first_part(&content);
    let second_score = second_part(&content);
    println!("First: {}", score);
    println!("Second: {}", second_score);
}