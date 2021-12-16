use std::fs::File;
use std::io::{BufRead, BufReader};

fn convert_to_bits(s: &str) -> Vec<bool> {
    let mut o: Vec<bool> = vec![];
    for c in s.chars() {
        /*
         */
        let mut a = match c {
            '0' => {
                vec![false, false, false, false]
            }
            '1' => {
                vec![false, false, false, true]
            }
            '2' => {
                vec![false, false, true, false]
            }
            '3' => {
                vec![false, false, true, true]
            }
            '4' => {
                vec![false, true, false, false]
            }
            '5' => {
                vec![false, true, false, true]
            }
            '6' => {
                vec![false, true, true, false]
            }
            '7' => {
                vec![false, true, true, true]
            }
            '8' => {
                vec![true, false, false, false]
            }
            '9' => {
                vec![true, false, false, true]
            }
            'A' => {
                vec![true, false, true, false]
            }
            'B' => {
                vec![true, false, true, true]
            }
            'C' => {
                vec![true, true, false, false]
            }
            'D' => {
                vec![true, true, false, true]
            }
            'E' => {
                vec![true, true, true, false]
            }
            'F' => {
                vec![true, true, true, true]
            }
            e => {
                panic!("not sure how to parse char {}", e);
            }
        };
        o.append(&mut a);
    }

    o
}

struct RawPacket {
    version: i32,
    packet_type: i32,
    data: Vec<bool>,
}

impl From<&str> for RawPacket {
    fn from(raw: &str) -> Self {
        RawPacket {
            version: 1,
            packet_type: 1,
            data: vec![],
        }
    }
}

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/16")?;
    let reader: BufReader<File> = BufReader::new(f);
    let raw: String = reader.lines().next().unwrap().unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bits1() {
        let raw = "0";
        assert_eq!(convert_to_bits(raw), vec![false, false, false, false]);
    }

    #[test]
    fn test_hex_to_bits_long() {
        let raw = "D2FE28";
        let expected = vec![
            true, true, false, true, false, false, true, false, true, true, true, true, true, true,
            true, false, false, false, true, false, true, false, false, false,
        ];
        assert_eq!(convert_to_bits(raw), expected);
    }

    //#[test]
    fn test_first_small_example() {
        let raw = "D2FE28";
        let raw_decoded: RawPacket = raw.into();
        assert_eq!(raw_decoded.version, 6);
        assert_eq!(raw_decoded.packet_type, 4);
        assert_eq!(
            raw_decoded.data,
            vec![false, true, true, true, true, true, true, false, false, true, false, true]
        );
    }
}
