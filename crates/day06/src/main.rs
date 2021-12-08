use std::{fs::File, io::{BufRead, BufReader}, time::Instant};

fn simulate(input: &Vec<i32>, time: i32) -> usize {
    let mut population: Vec<i32> = input.clone();
    for _ in 0..time {
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

fn pt1(input: &Vec<i32>) -> usize {
    simulate(input, 80)
}

fn pt2(input: &Vec<i32>) -> usize {
    let mut population: Vec<i32> = input.clone();
    for i in 0..256 {
        print!("day {} ", i);
        let now = Instant::now();
        for fish in 0..population.len() {
            if population[fish] == 0 {
                population[fish] = 6;
                population.push(8);
            } else {
                population[fish] -= 1;
            }
        }
        print!("population: {} ", population.len());
        println!("took: {:?} ", now.elapsed());
    }
    population.len()
}

fn simulate_descendants(age: i32, time: i32) -> usize {
    let mut population: Vec<i32> = vec![age];
    for i in 0..time {
        print!("day {} ", i);
        let now = Instant::now();
        for fish in 0..population.len() {
            if population[fish] == 0 {
                population[fish] = 6;
                population.push(8);
            } else {
                population[fish] -= 1;
            }
        }
        print!("population: {} ", population.len());
        println!("took: {:?} ", now.elapsed());
    }
    population.len() - 1
}

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/6")?;
    let reader: BufReader<File> = BufReader::new(f);
    let lines: Vec<String> = reader.lines().map(|i| i.unwrap()).collect();
    let input: Vec<i32> = lines.iter().map(|i| {
        i.split(",").map(|e| e.parse().unwrap())
    }).flatten().collect();

    //println!("running");
    //println!("what {}", count_descendants(&6, 11));
    //let t = 1;
    //assert_eq!(1 + count_descendants(&0, t), simulate(&vec![0], t), "failed on t={}", t);
    //println!("done");

    //println!("pt1: {}", pt1(&input));
    //println!("pt2: {}", pt2(&input));
    Ok(())
}

fn count_descendants(age: &i32, time: i32) -> usize {
    fn _m(age: &i32, time: i32) -> usize {
        let mut current_age = age;
        let mut current_time = time;
        let mut d = 0;
        while dbg!(current_time) > 0 && current_time > *current_age {
            current_time = current_time - dbg!(current_age);
            if current_time >= 1 {
                d += 1;
                d += count_descendants(&8, current_time);
                current_age = &6;
            }
        }
        d
    }
    _m(age, time)
}


#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_example1() {
        let input: Vec<i32> = vec![3,4,3,1,2];
        assert_eq!(pt1(&input), 5934);
    }

    //#[test]
    fn test_example1_alt() {
        let input: Vec<i32> = vec![3,4,3,1,2];
        //let counted: i32 = input.iter().map(|e| count_descendants(e, 80)).sum();
        let counted: usize = input.iter().map(|e| count_descendants(e, 80)).sum();
        assert_eq!(counted, 5934);
    }

    //#[test]
    fn test_count_descendants() {
        assert_eq!(count_descendants(&0, 1), 1);
        assert_eq!(count_descendants(&6, 6), 0);
        assert_eq!(count_descendants(&6, 7), 1);
        //assert_eq!(count_descendants(&6, 13), 1);
    }

    #[test]
    fn test_count_descendants_same() {
        let t = 7;
        assert_eq!(1 + count_descendants(&0, t), simulate(&vec![0], t), "failed on t={}", t);
        //for t in 0..2 {
        //    assert_eq!(1 + count_descendants(&0, t), simulate(&vec![0], t), "failed on t={}", t);
        //}
    }

    //#[test]
    fn test_simulate_descendants() {
        assert_eq!(simulate_descendants(1, 5), 1);
        assert_eq!(simulate_descendants(6, 2), 0);
        //assert_eq!(simulate_descendants(6, 100), 0);
    }

}
