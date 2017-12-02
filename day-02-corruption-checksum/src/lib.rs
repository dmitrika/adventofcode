//  For each row, determine the difference between the largest value and the smallest value;
// the checksum is the sum of all of these differences.

fn main(vec: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for i in vec {
        let max = i.iter().max().unwrap();
        let min = i.iter().min().unwrap();

        sum += max - min;
    }

    sum
}

// It sounds like the goal is to find the only two numbers in each row where one evenly divides the other
// - that is, where the result of the division operation is a whole number.
// They would like you to find those numbers on each line, divide them, and add up each line's result.
fn main2(vec: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for inner_vec in vec {
        for (index, item) in inner_vec.iter().enumerate() {
            let mut vec_without_item = inner_vec.clone();

            vec_without_item.remove(index);

            for item_inner in vec_without_item {
                if item.checked_rem(item_inner).unwrap() == 0 {
                    sum += item / item_inner
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![
            vec![5, 1, 9, 5],
            vec![7, 5, 3],
            vec![2, 4, 6, 8]
        ];

        assert_eq!(main(vec), 18);
    }

    #[test]
    fn second_works() {
        let vec = vec![
            vec![5, 9, 2, 8],
            vec![9, 4, 7, 3],
            vec![3, 8, 6, 5],
        ];

        assert_eq!(main2(vec), 9);
    }
}
