use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse(input: Vec<String>) -> Vec<Direction> {
    use Direction::*;
    input.iter().map(|e| {
        let s = e.split(" ").collect::<Vec<&str>>();
        let word = s[0];
        let num: i32 = s[1].parse().unwrap();
        if word == "forward" {
            Forward(num)
        } else if word == "up" {
            Up(num)
        } else if word == "down" {
            Down(num)
        } else {
            panic!("got weird word: {}", word);
        }
    }).collect::<Vec<Direction>>()
}

fn pt1(input: &Vec<Direction>) -> i32 {
    use Direction::*;
    let (finalx, finaly) = input.iter().fold((0, 0), |(x, y), e| {
        match e {
            Forward(n) => {
                (x + n, y)
            },
            Up(n) => {
                (x, y - n)
            },
            Down(n) => {
                (x, y + n)
            }
        }
    });
    finalx * finaly
}


fn pt2(input: &Vec<Direction>) -> i32 {
    use Direction::*;
    let (finalx, finaly, finalaim) = input.iter().fold((0, 0, 0), |(x, y, a), e| {
        match e {
            Forward(n) => {
                (x + n, y + (n * a), a)
            },
            Up(n) => {
                (x, y, a - n)
            },
            Down(n) => {
                (x, y, a + n)
            }
        }
    });
    finalx * finaly
}

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/2")?;
    let reader: BufReader<File> = BufReader::new(f);
    let input: Vec<Direction> = parse(reader.lines().map(|i| i.unwrap()).collect());
    println!("pt 1 {}", pt1(&input));
    println!("pt 2 {}", pt2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_parse() {
        use Direction::*;
        let expected = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        assert_eq!(parse(TEST.lines().map(|i| i.to_string()).collect()), expected);
    }

    #[test]
    fn test_pt1() {
        let input = parse(TEST.lines().map(|i| i.to_string()).collect());
        assert_eq!(pt1(&input),150);
    }

    #[test]
    fn test_pt2() {
        let input = parse(TEST.lines().map(|i| i.to_string()).collect());
        assert_eq!(pt2(&input), 900);
    }

}
