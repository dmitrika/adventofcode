use std::fs::File;
use std::io::prelude::*;

fn read_file(path: String) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("something went wrong reading the file");

    content
}

pub fn main(path: String, with_updater: bool) -> i32 {
    let content: String = read_file(path);
    let mut instructions: Vec<_> = content.lines().map(|s: &str| s.parse::<isize>().unwrap()).collect();

    let mut steps = 0;
    let mut position: isize = 0;

    while position < instructions.len() as isize {
        let current_value = instructions[position as usize];

        if with_updater && current_value >= 3 {
            instructions[position as usize] = current_value - 1;
        } else {
            instructions[position as usize] = current_value + 1;
        }

        steps += 1;
        position += current_value;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(main(String::from("test.txt"), false), 5);
    }

    #[test]
    #[ignore]
    fn it_works_control() {
        assert_eq!(main(String::from("input.txt"), true), 0);
    }
}
