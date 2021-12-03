use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_digits(input: &Vec<String>, digit_position: usize) -> (i32, i32) {
    let mut counter = (0, 0);
    for num in input {
        let digits = num.chars().collect::<Vec<char>>();
        if digits[digit_position] == '0' {
            counter.0 += 1;
        } else {
            counter.1 += 1;
        }
    }
    let (zeros, ones) = counter;
    if zeros > ones {
        (0, 1)
    } else {
        if zeros == ones {
            println!("they tied");
        }
        (1, 0)
    }
}

fn get_winner_break_tie(input: &Vec<String>, digit_position: usize, tie_break_default: i32) -> i32 {
    let mut counter = (0, 0);
    for num in input {
        let digits = num.chars().collect::<Vec<char>>();
        if digits[digit_position] == '0' {
            counter.0 += 1;
        } else {
            counter.1 += 1;
        }
    }
    let (zeros, ones) = counter;
    if zeros == ones {
        return tie_break_default;
    }
    if zeros > ones {
        0
    } else {
        1
    }
}

fn get_loser_break_tie(input: &Vec<String>, digit_position: usize, tie_break_default: i32) -> i32 {
    let mut counter = (0, 0);
    for num in input {
        let digits = num.chars().collect::<Vec<char>>();
        if digits[digit_position] == '0' {
            counter.0 += 1;
        } else {
            counter.1 += 1;
        }
    }
    let (zeros, ones) = counter;
    if zeros == ones {
        return tie_break_default;
    }
    if zeros > ones {
        1
    } else {
        0
    }
}

fn make_number(bits: Vec<i32>) -> i32 {
    let mut ret = 0;
    for b in bits {
        ret = ret << 1;
        ret += b;
    }
    ret
}

fn get_gamma_epsilon(input: &Vec<String>) -> (i32, i32) {
    let width = input[0].len();
    let mut gamma_digits = vec![];
    let mut epsilon_digits = vec![];
    for digit_position in 0..width {
        let (gam, ep) = get_digits(&input, digit_position);
        gamma_digits.push(gam);
        epsilon_digits.push(ep);
    }
    let gamma = make_number(gamma_digits);
    let epsilon = make_number(epsilon_digits);
    (gamma, epsilon)
}

fn find_oxygen(input: &Vec<String>) -> i32 {
    let width = input[0].len();
    let mut candidates = input.clone();
    for position in 0..width {
        if candidates.len() == 1 {
            break;
        }
        let winner = get_winner_break_tie(&candidates, position, 1);
        candidates = candidates
            .iter()
            .filter(|num| {
                num.chars()
                    .nth(position)
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap()
                    == winner
            })
            .map(|e| e.clone())
            .collect();
    }
    let digits: Vec<_> = candidates[0]
        .chars()
        .map(|e| e.to_string().parse::<i32>().unwrap())
        .collect();
    make_number(digits)
}

fn find_co2(input: &Vec<String>) -> i32 {
    let width = input[0].len();
    let mut candidates = input.clone();
    for position in 0..width {
        if candidates.len() == 1 {
            break;
        }
        let winner = get_loser_break_tie(&candidates, position, 0);
        candidates = candidates
            .iter()
            .filter(|num| {
                num.chars()
                    .nth(position)
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap()
                    == winner
            })
            .map(|e| e.clone())
            .collect();
    }
    let digits: Vec<_> = candidates[0]
        .chars()
        .map(|e| e.to_string().parse::<i32>().unwrap())
        .collect();
    make_number(digits)
}

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/3")?;
    let reader: BufReader<File> = BufReader::new(f);
    let input: Vec<String> = reader.lines().map(|i| i.unwrap()).collect();
    let (g, e) = get_gamma_epsilon(&input);
    println!("pt 1 {}", g * e);
    let (o, c) = (find_oxygen(&input), find_co2(&input));
    println!("pt 2 {}", o * c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_get_digits() {
        let input = TEST.lines().map(|i| i.to_string()).collect();
        let (g, e) = get_digits(&input, 0);
        assert_eq!(g, 1);
        assert_eq!(e, 0);
    }

    #[test]
    fn test_make_number() {
        assert_eq!(make_number(vec![1, 0]), 2);
        assert_eq!(make_number(vec![1, 1, 1, 0, 0]), 28);
    }

    #[test]
    fn test_pt1() {
        let input = TEST.lines().map(|i| i.to_string()).collect();
        let (g, e) = get_gamma_epsilon(&input);
        assert_eq!(g, 22);
        assert_eq!(e, 9);
        assert_eq!(g * e, 198);
    }

    #[test]
    fn test_find_ox() {
        let input = TEST.lines().map(|i| i.to_string()).collect();
        let o = find_oxygen(&input);
        assert_eq!(o, 23);
    }

    #[test]
    fn test_find_co2() {
        let input = TEST.lines().map(|i| i.to_string()).collect();
        let c = find_co2(&input);
        assert_eq!(c, 10);
    }
}
