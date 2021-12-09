use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse(input: Vec<String>) -> Vec<Vec<i32>> {
    let map: Vec<Vec<i32>> = input
        .iter()
        .map(|e| {
            e.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    map
}

fn pt1(map: &Vec<Vec<i32>>) -> i32 {
    let y_edge = map.len() as i32;
    let x_edge = map[0].len() as i32;
    let mut lowest_points: Vec<i32> = vec![];
    for y in 0..y_edge {
        for x in 0..x_edge {
            let cell = (x, y);
            // stuff to check
            let up = (x, y - 1);
            let down = (x, y + 1);
            let right = (x + 1, y);
            let left = (x - 1, y);

            let mut surrounding = vec![];
            if up.1 >= 0 {
                let (_x, _y) = up;
                surrounding.push(map[_y as usize][_x as usize]);
            }

            if down.1 < y_edge {
                let (_x, _y) = down;
                surrounding.push(map[_y as usize][_x as usize]);
            }

            if right.0 < x_edge {
                let (_x, _y) = right;
                surrounding.push(map[_y as usize][_x as usize]);
            }

            if left.0 >= 0 {
                let (_x, _y) = left;
                surrounding.push(map[_y as usize][_x as usize]);
            }

            let min_neighbor = surrounding.iter().min().unwrap();
            if min_neighbor > &map[y as usize][x as usize] {
                lowest_points.push(map[y as usize][x as usize]);
            }
        }
    }
    lowest_points.iter().map(|e| e + 1).sum::<i32>()
}

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/9")?;
    let reader: BufReader<File> = BufReader::new(f);
    let lines: Vec<String> = reader.lines().map(|i| i.unwrap()).collect();
    let map = parse(lines);
    println!("pt1: {}", pt1(&map));
    //println!("pt2: {}", pt2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST: &str = "2199943210
    3987894921
    9856789892
    8767896789
    9899965678";

    #[test]
    fn test_example1() {
        let input: Vec<String> = TEST.lines().map(|i| i.to_string()).collect();
        let entries: Vec<Vec<i32>> = parse(input);
        assert_eq!(pt1(&entries), 15);
    }

    // #[test]
    // fn test_example2() {
    //     let input: Vec<i32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    //     assert_eq!(pt2(&input), 168);
    // }
}
