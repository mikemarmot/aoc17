use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day02.txt").unwrap());
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let res = calc(&input);
    println!("Result of day02 p2: {}", res);
}

fn calc(input: &Vec<String>) -> usize {
    let mut res = 0;

    for l in input {
        let data = l.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        for n in &data {
            for m in &data {
                if m != n && m % n == 0 {
                    res += m / n;
                }

            }
        }
    }

    res
}

mod tests {
    #[test]
    fn test_calc() {
        let input: Vec<String> = vec![
            String::from("5 9 2 8"),
            String::from("9 4 7 3"),
            String::from("3 8 6 5"),

        ];

        assert_eq!(super::calc(&input), 9);
    }
}