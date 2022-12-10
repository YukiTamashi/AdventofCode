use std::fs;

fn main() {
    let moves = fs::read_to_string("day5.in").unwrap();
    let stacks = fs::read_to_string("stacks.in").unwrap();
    let mut stacks = build_stack(&stacks);
    println!("{:?}", stacks);
    solve_moves(&mut stacks, &moves);
    println!("{:?}", stacks);
}


fn build_stack(s: &str) -> Vec<Vec<char>>{
    let mut result: Vec<Vec<char>> = vec!();
    let a = s.lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    println!("{:#?}", a);
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

fn solve_moves(stacks: &mut Vec<Vec<char>>, moves: &str){
    for line in moves.lines(){
        let mut moves = vec!();
        for word in line.split_whitespace(){
            if let Ok(n) = word.parse::<usize>(){
                moves.push(n);
            }
        }
        println!("{:?}", moves);
        for _ in 0..moves[0]{
            transfer(stacks, moves[1], moves[2]);
        }
    }
}

fn transfer(stacks: &mut Vec<Vec<char>>, from: usize, to: usize) {
    let c = stacks[from-1].pop();
    if let Some(c) = c{
        stacks[to-1].push(c);
    }
}
