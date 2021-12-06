use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day02.txt").unwrap());
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let res = calc(&input);
    println!("Result of day02 p1: {}", res);
}

fn calc(input: &Vec<String>) -> usize {
    let mut res = 0;

    for l in input {
        let data = l.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let max = data.iter().max().unwrap();
        let min = data.iter().min().unwrap();
        res += max - min;
    }

    res
}

mod tests {
    #[test]
    fn test_calc() {
        let input: Vec<String> = vec![
            String::from("5 1 9 5"),
            String::from("7 5 3"),
            String::from("2 4 6 8"),

        ];

        assert_eq!(super::calc(&input), 18);
    }
}