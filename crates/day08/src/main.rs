use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

struct SignalEntry {
    signals: Vec<String>,
    output: Vec<String>,
}

enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

struct PotentialNumbers(Vec<i32>);

impl From<HashSet<Segment>> for PotentialNumbers {
    fn from(_: HashSet<Segment>) -> Self {
        todo!()
    }
}

impl From<&String> for SignalEntry {
    fn from(line: &String) -> Self {
        let parts: Vec<&str> = line.split(" | ").collect();
        let signals = parts[0].split(" ").map(|e| e.to_string()).collect();
        let output = parts[1].split(" ").map(|e| e.to_string()).collect();
        SignalEntry {
            signals,
            output,
        }
    }
}

fn pt1(entries: &Vec<SignalEntry>) -> i32 {
    // "In the output values, how many times do digits 1, 4, 7, or 8 appear?"
    // 1: 2 segments
    // 4: 4 segments
    // 7: 3 segments
    // 8: 7 segments
    let mut count = 0;
    for e in entries {
        let counted = e.output.iter().filter(|digit| {
            let segments = digit.len();
            segments == 2 ||
            segments == 4 ||
            segments == 3 ||
            segments == 7
        }).count();
        count += counted;
    }
    count as i32
}

/*
fn pt2_decode(entry: &SignalEntry) -> Vec<i32> {
}
*/

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/8")?;
    let reader: BufReader<File> = BufReader::new(f);
    let lines: Vec<String> = reader.lines().map(|i| i.unwrap()).collect();
    let entries: Vec<SignalEntry> = lines.iter().map(|e| e.into()).collect();
    println!("pt1: {}", pt1(&entries));
    //println!("pt2: {}", pt2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_example1() {
        let input: Vec<String> = TEST.lines().map(|i| i.to_string()).collect();
        let entries: Vec<SignalEntry> = input.iter().map(|e| e.into()).collect();
        assert_eq!(pt1(&entries), 26);
    }

    #[test]
    fn test_example2() {
        let input: Vec<String> = TEST.lines().map(|i| i.to_string()).collect();
        let entries: Vec<SignalEntry> = input.iter().map(|e| e.into()).collect();
        assert_eq!(pt2_decode(&entries[0]), vec![5,3,5,3]);
    }

    // #[test]
    // fn test_example2() {
    //     let input: Vec<i32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    //     assert_eq!(pt2(&input), 168);
    // }
}
