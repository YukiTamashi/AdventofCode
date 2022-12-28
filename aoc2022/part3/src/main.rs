use std::{fs, collections::HashSet};

fn main() {
    let content = fs::read_to_string("day3.in").unwrap();
    let mut total = 0;
    let mut badge_total = 0;
    let mut stack: Vec<String> = vec!();
    for line in content.lines(){
        total += solve_line(line);
        stack.push((*line).to_owned());
        if stack.len() == 3{
            badge_total += find_badge(&stack);
            stack.clear();
        }
    }
    println!("Total: {total}");
    println!("Badge total: {badge_total}");
}

fn solve_line(line: &str) -> u32{
    let (first, second) = line.split_at(line.len()/2);
    let map: HashSet<char> = HashSet::from_iter(first.chars());
    for char in second.chars(){
        if map.contains(&char){
            return value_from_char(char)
        }
    }
    0
}

fn value_from_char(c: char) -> u32{
    let mut offset = 0;
    if c.is_uppercase(){
        offset = 26;
    }
    c.to_digit(36).unwrap() - 9 + offset
}

fn find_badge(v: &Vec<String>) -> u32{
    if v.len() != 3{
        panic!();
    }
    let set1: HashSet<char> = HashSet::from_iter(v[0].chars());
    let set2: HashSet<char> = HashSet::from_iter(v[1].chars());
    let set3: HashSet<char> = HashSet::from_iter(v[2].chars());
    let union: HashSet<char> = HashSet::from_iter(set1.intersection(&set2).copied());
    let answer = union.intersection(&set3).copied().collect::<Vec<char>>();
    value_from_char(answer[0])
}