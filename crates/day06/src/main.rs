use std::{fs::File, io::{BufRead, BufReader}};




pub fn main() -> std::io::Result<()> {
    let f = File::open("data/6")?;
    let reader: BufReader<File> = BufReader::new(f);
    let input: Vec<i32> = reader.lines().map(|i| i.unwrap().parse().unwrap()).collect();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_example1() {
        let input: Vec<i32> = vec![3,4,3,1,2];
    }

}
