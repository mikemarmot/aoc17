use std::fs;

pub fn doit() {
    let input = fs::read_to_string("data/input_day01.txt").unwrap();
    let res = calc(&input);
    println!("Result of day01 p2: {}", res);
}

fn calc(input: &str) -> usize {
    let input = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let mut res = 0;
    for i in 0..input.len() {
        if input[i] == input[(i+input.len()/2) % input.len()] {
            res += input[i];
        }
    }
    res as usize
}

mod tests {
    #[test]
    fn test_calc() {
        assert!(super::calc(&"1212") == 6);
        assert!(super::calc(&"1221") == 0);
        assert!(super::calc(&"123425") == 4);
        assert!(super::calc(&"123123") == 12);
        assert!(super::calc(&"12131415") == 4);
    }
}