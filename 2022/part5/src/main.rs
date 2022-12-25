use std::fs;

#[derive(Debug)]
struct Move{
    amount: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Move{
    fn from(s: &str) -> Self {
        let mut moves = vec!();
        for word in s.split_whitespace(){
            if let Ok(n) = word.parse::<usize>(){
                moves.push(n);
            }
        }
        if moves.len() != 3{
            panic!()
        }
        Move { amount: moves[0], from: moves[1] -1, to: moves[2] -1}
    }
}

fn main() {
    let moves = fs::read_to_string("day5.in").unwrap();
    let stacks = fs::read_to_string("stacks.in").unwrap();
    let stacks = build_stack(&stacks);
    let moves = solve_moves(&moves);
    println!("Part 1: {}", part1(&stacks, &moves));
    println!("Part 2: {}", part2(&stacks, &moves))
}


fn build_stack(s: &str) -> Vec<Vec<char>>{
    let mut result: Vec<Vec<char>> = vec!();
    let a = s.lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    // println!("{:#?}", a);
    for (i, c) in a[8].iter().enumerate(){
        if *c != ' '{
            let mut new = vec!();
            for vec in a.clone().iter().rev(){
                let c = vec[i];
                if c != ' ' && c.is_alphabetic(){
                    new.push(vec[i])
                }
            }
            result.push(new)
        }
    }
    result
}

fn solve_moves(moves: &str)-> Vec<Move>{
    let mut moves_vec = vec!();
    for line in moves.lines(){
        moves_vec.push(Move::from(line));
    }
    moves_vec
}

fn transfer(mut stacks: Vec<Vec<char>>, moves: &Vec<Move>) -> Vec<Vec<char>>{
    for m in moves{
        for _ in 0..m.amount{
            let c = stacks[m.from].pop();
            if let Some(c) = c{
                stacks[m.to].push(c);
            }
        }
    }
    stacks
}

fn transfer_retain(mut stacks: Vec<Vec<char>>, moves: &Vec<Move>) -> Vec<Vec<char>>{
    let mut temp = vec!();
    for m in moves{
        for _ in 0..m.amount{
            let c = stacks[m.from].pop();
            if let Some(c) = c{
                temp.push(c);
            }
        }
        while !temp.is_empty(){
            stacks[m.to].push(temp.pop().unwrap());
        }
    }
    stacks
}

fn part1(stacks: & [Vec<char>], moves: &Vec<Move>) -> String{
    let stacks = transfer(stacks.to_owned(), moves);
    let mut result = "".into();
    for s in stacks{
        result = format!("{}{}", result, s.last().unwrap());
    }
    result
}

fn part2(stacks: & [Vec<char>], moves: &Vec<Move>) -> String{
    let stacks = transfer_retain(stacks.to_owned(), moves);
    let mut result = "".into();
    for s in stacks{
        result = format!("{}{}", result, s.last().unwrap());
    }
    result
}