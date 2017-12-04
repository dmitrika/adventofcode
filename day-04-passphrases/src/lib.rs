use std::fs::File;
use std::io::prelude::*;

extern crate itertools;
use itertools::Itertools;

fn read_file(path: String) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("something went wrong reading the file");

    content
}

fn normalize(s: &str) -> Vec<String> {
    s.chars()
        .sorted()
        .iter()
        .map(|c| c.to_string())
        .collect()
}

fn equal_all(line: &str) -> bool {
    let mut phrases: Vec<_> = line.split_whitespace().map(|s| normalize(s)).collect();
    let initial_len = phrases.len();

    phrases.sort();
    phrases.dedup();

    initial_len == phrases.len()
}

fn main(path: String) -> i32 {
    let content: String = read_file(path);
    let lines: Vec<&str> = content.lines().collect();

    let mut sum = 0;

    for line in lines {
        if equal_all(&line) {
            sum += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(main(String::from("test.txt")), 2);
    }

    #[test]
    fn it_works2() {
        assert_eq!(main(String::from("test2.txt")), 3);
    }

    #[test]
    #[ignore]
    fn it_works_control() {
        assert_eq!(main(String::from("input.txt")), 0);
    }

    #[test]
    #[ignore]
    fn it_works_control2() {
        assert_eq!(main(String::from("input.txt")), 0);
    }
}
