use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl From<&String> for Vent {
    /// converts "n,n -> n,n"
    /// should be FromStr but i don't want to learn about error classes now
    fn from(s: &String) -> Self {
        let points: Vec<&str> = s.split(" -> ").collect();
        let coords: Vec<(i32, i32)> = points.iter().map(|e| {
            let mut split = e.split(",");
            let x = split.next().unwrap().parse::<i32>().unwrap();
            let y = split.next().unwrap().parse::<i32>().unwrap();
            (x, y)
        }).collect();
        let (x1, y1) = coords[0];
        let (x2, y2) = coords[1];
        Vent {
            x1,
            y1,
            x2,
            y2,
        }
    }
}

fn pt1(vents: &Vec<Vent>) -> usize {
    // remove diagonals. only look at vertical or horizontal vents
    let no_diagonal = vents.iter().filter(|e| {
        e.x1 == e.x2 || e.y1 == e.y2
    });
    let mut danger_map: HashMap<(i32, i32), usize> = HashMap::new();
    
    for v in no_diagonal {
        // mark the start/end before iterating so the iteration code is simpler
        let source = danger_map.entry((v.x1, v.y1)).or_insert(0);
        *source += 1;
        let target = danger_map.entry((v.x2, v.y2)).or_insert(0);
        *target += 1;
        // if these are different, the other axis is the dupe to maintain 
        // the horizontal/vertical constraint
        if v.x1 != v.x2 {
            let (start, end) = if v.x1 < v.x2 {(v.x1, v.x2)} else {(v.x2, v.x1)};
            for x in (start+1)..end {
                let here = danger_map.entry((x, v.y1)).or_insert(0);
                *here += 1;
            }
        }
        if v.y1 != v.y2 {
            let (start, end) = if v.y1 < v.y2 {(v.y1, v.y2)} else {(v.y2, v.y1)};
            for y in (start+1)..end {
                let here = danger_map.entry((v.x1, y)).or_insert(0);
                *here += 1;
            }
        }
    }

    // collect points with danger greater than 1
    danger_map.values().filter(|e| **e > 1).count()
}

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/5")?;
    let reader: BufReader<File> = BufReader::new(f);
    let input: Vec<String> = reader.lines().map(|i| i.unwrap()).collect();
    let vents: Vec<Vent> = input.iter().map(|e| e.into()).collect();
    println!("pt1: {}", pt1(&vents));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_parse() {
        let input: Vec<String> = TEST.lines().map(|i| i.to_string()).collect();
        let vents: Vec<Vent> = input.iter().map(|e| e.into()).collect();
        assert_eq!(vents[0].x1, 0);
        assert_eq!(vents[0].y1, 9);
        assert_eq!(vents[0].x2, 5);
        assert_eq!(vents[0].y2, 9);
    }

    #[test]
    fn test_example1() {
        let input: Vec<String> = TEST.lines().map(|i| i.to_string()).collect();
        let vents: Vec<Vent> = input.iter().map(|e| e.into()).collect();
        assert_eq!(pt1(&vents), 5);
    }


}
