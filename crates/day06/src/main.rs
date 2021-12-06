use std::{fs::File, io::{BufRead, BufReader}};



fn pt1(input: &Vec<i32>) -> usize {
    let mut population: Vec<i32> = input.clone();
    for _ in 0..80 {
        for fish in 0..population.len() {
            if population[fish] == 0 {
                population[fish] = 6;
                population.push(8);
            } else {
                population[fish] -= 1;
            }
        }
    }
    population.len()
}

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/6")?;
    let reader: BufReader<File> = BufReader::new(f);
    let input: Vec<i32> = reader.lines().map(|i| i.unwrap().parse().unwrap()).collect();
    println!("pt1: {}", pt1(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_example1() {
        let input: Vec<i32> = vec![3,4,3,1,2];
        assert_eq!(pt1(&input), 5934);
    }

}
