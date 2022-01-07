use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{remove_file, File};

    #[test]
    fn json_serialize_file() {
        let test_file = "json_test.txt";

        let a = Move::Up(5);
        let file = File::create(test_file).expect("failed to create file.");
        serde_json::to_writer(file, &a).expect("failed to serialize Move to file.");

        let file = File::open(test_file).expect("failed to open file.");
        let b: Move = serde_json::from_reader(file).expect("failed to deserialze Move from file.");

        assert_eq!(a, b);
        remove_file(test_file).unwrap();
    }

    #[test]
    fn ron_serialize_buffer() {
        let a = Move::Down(1);
        let buf = ron::ser::to_string(&a).unwrap().into_bytes();
        assert_eq!(a, ron::de::from_bytes(&buf).unwrap());
    }

    #[test]
    fn bson_serialize_1000() {
        let mut moves = vec![];
        for i in 0..1000 {
            match i % 4 {
                0 => moves.push(Move::Up(i % 25)),
                1 => moves.push(Move::Down(i % 25)),
                2 => moves.push(Move::Left(i % 25)),
                3 => moves.push(Move::Right(i % 25)),
                _ => unreachable!(),
            }
        }
    }
}
