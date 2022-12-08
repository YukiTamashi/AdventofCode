use std::fs;

fn main() {
    let content = fs::read_to_string("day1.in").unwrap();
    //Separates into sets of multiple lines, dividing based on where an empty line was found
    let a: Vec<&str> = content.split("\r\n\r\n").collect();
    //Iterates over each set of lines.
    //On each set, sums value of each line
    //Returns Vec of sum of each set
    let mut b: Vec<i32> = a
        .iter()
        .map(|x| 
            (*x).lines()
                .map(|x| 
                    x.parse::<i32>()
                     .unwrap())
                     .sum())
        .collect();
    b.sort();
    println!("Total from highest: {}", b[b.len() - 1]);
    println!(
        "Total from 3 highest: {}",
        b[b.len() - 1] + b[b.len() - 2] + b[b.len() - 3]
    );
}
