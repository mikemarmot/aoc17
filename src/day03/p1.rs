pub fn doit() {
    let input = 289326;
    let res = calc(input);
    println!("Result of day02 p1: {}", res);
}

fn calc(input: usize) -> usize {

    // (0) 0 = n * (4 * 1)
    // (1) 8 = n * (4 * 2)
    // (2) 24 = n * (4 * 3)
    // (3) 48 = n * (4 * 4)

    let mut res: Option<usize> = None;
    for i in 0.. {
        let maxval = 4 * i * (i + 1);
        if !((input - 1) <= maxval) {
            continue;
        }
        println!("LLL {}", i);
        res = Some(i);
        break;        
    }

    res.unwrap()
}

mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::calc(1), 0);
        assert_eq!(super::calc(12), 3);
        assert_eq!(super::calc(23), 2);
        assert_eq!(super::calc(1024), 31);
    }
}