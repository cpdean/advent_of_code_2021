use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn pt1(input: Vec<i32>) -> i32 {
    let init = (0, input[0]);
    let count: (i32, i32) = input.iter().fold(init, |(c, previous), element| {
        if element > &previous {
            (c + 1, *element)
        } else {
            (c, *element)
        }
    });
    count.0
}

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/1")?;
    let reader: BufReader<File> = BufReader::new(f);
    let depths = reader.lines().map(|line| line.unwrap().parse().unwrap()).collect::<Vec<i32>>();
    // count the number of times the depth increases
    let count = pt1(depths);
    println!("pt 1 {}", count);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt2_valid() {
        let test_input: Vec<i32> = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];
        assert_eq!(pt1(test_input), 7);
    }

}
