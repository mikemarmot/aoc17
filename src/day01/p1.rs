use std::fs;

pub fn doit() {
    let input = fs::read_to_string("data/input_day01.txt").unwrap();
    let res = calc(&input);
    println!("Result of day01 p1: {}", res);
}

fn calc(input: &str) -> usize {
    let input = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let mut res = 0;
    for i in 0..input.len() {
        if input[i] == input[(i+1) % input.len()] {
            res += input[i];
        }
    }
    res as usize
}

mod tests {
    #[test]
    fn test_calc() {
        assert!(super::calc(&"1122") == 3);
        assert!(super::calc(&"1111") == 4);
        assert!(super::calc(&"1234") == 0);
        assert!(super::calc(&"91212129") == 9);
    }
}