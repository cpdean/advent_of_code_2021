use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// but why is it wrong?
fn pt1_wrong(input: &Vec<i32>) -> i32 {
    // find the avg, then the total cost of aligning everything to the avg
    let avg = input.iter().sum::<i32>() / input.len() as i32;
    dbg!(&avg);
    let costs = input.iter().map(|e| (e - avg).abs());
    costs.sum()
}

fn pt1(input: &Vec<i32>) -> i32 {
    // just guess every int in the range
    let (min, max) = (input.iter().min().unwrap(), input.iter().max().unwrap());
    let mut costs = vec![];
    for i in *min..=*max {
        let total_cost: i32 = input.iter().map(|e| (e - i).abs()).sum();
        costs.push((total_cost, i));
    }
    let min_cost = costs.iter().min();
    min_cost.unwrap().0
}

fn pt2_cost(n: i32) -> i32 {
    if n == 0 {
        0
    }
    else if n == 1 {
        1
    } else {
        n + pt2_cost(n - 1)
    }
}

fn pt2(input: &Vec<i32>) -> i32 {
    // just guess every int in the range
    let (min, max) = (input.iter().min().unwrap(), input.iter().max().unwrap());
    let mut costs = vec![];
    for i in *min..=*max {
        let total_cost: i32 = input.iter().map(|e| {
            pt2_cost((e - i).abs())
        }).sum();
        costs.push((total_cost, i));
    }
    let min_cost = costs.iter().min();
    min_cost.unwrap().0
}

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/7")?;
    let reader: BufReader<File> = BufReader::new(f);
    let lines: Vec<String> = reader.lines().map(|i| i.unwrap()).collect();
    let input: Vec<i32> = lines
        .iter()
        .map(|i| i.split(",").map(|e| e.parse().unwrap()))
        .flatten()
        .collect();
    println!("pt1: {}", pt1(&input));
    println!("pt2: {}", pt2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input: Vec<i32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(pt1(&input), 37);
    }

    #[test]
    fn test_example2() {
        let input: Vec<i32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(pt2(&input), 168);
    }
}
