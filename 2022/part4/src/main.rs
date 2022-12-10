use std::fs;

fn main() {
    let content = fs::read_to_string("day4.in").unwrap();
    let result = process(content);
    let first = first(&result);
    let second = second(&result);
    println!("{first}");
    println!("{second}");
}

fn process(s: String) -> Vec<Vec<u32>>{
    let mut result: Vec<Vec<u32>> = vec!();
    result.pop();
    for line in s.lines(){
        let a: Vec<u32> = line.split(',').flat_map(|x| x.split('-')).map(|x| x.parse().unwrap()).collect();
        result.push(a);
    }
    result
}

fn first(v: &Vec<Vec<u32>>) -> u32{
    let mut result = 0;
    for val in v{
        if  (val[0]..=val[1]).contains(&val[2]) &&
            (val[0]..=val[1]).contains(&val[3]) ||
            (val[2]..=val[3]).contains(&val[0]) &&
            (val[2]..=val[3]).contains(&val[1]){
                result += 1;
            }
    }
    result
}

fn second(v: &Vec<Vec<u32>>) -> u32{
    let mut result = 0;
    for val in v{
        if  (val[0]..=val[1]).contains(&val[2]) ||
            (val[0]..=val[1]).contains(&val[3]) ||
            (val[2]..=val[3]).contains(&val[0]) ||
            (val[2]..=val[3]).contains(&val[1]){
                result += 1;
            }
    }
    result
}