use std::fs;

fn main() {
    let input = fs::read_to_string("day6.in").unwrap();
    let test = is_unique(&input[1234..1238]);
    println!("{test}");
    let result = part1(input.clone());
    let result2 = part2(input);
    println!("First 4 unique: {result}");
    println!("First 14 unique: {result2}");
}

fn is_unique(s: &str) -> bool{
    use std::collections::HashSet;
    let mut map = HashSet::new();
    for c in s.chars(){
        if map.contains(&c){
            return false
        }
        else{
            map.insert(c);
        }
    }
    true
}

fn part1(s: String) -> usize{
    for i in 0..s.len()-4{
        if is_unique(&s[i..i+4]){
            return i+4;
        }
    }
    0
}

fn part2(s: String) -> usize{
    for i in 0..s.len()-4{
        if is_unique(&s[i..i+14]){
            return i+14;
        }
    }
    0
}
