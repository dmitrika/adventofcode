// The captcha requires you to review a sequence of digits (your puzzle input)
// and find the sum of all digits that match the next digit in the list.
// The list is circular, so the digit after the last digit is the first digit in the list.

fn sum(vec: Vec<i32>) -> i32 {
    let mut matched: Vec<i32> = Vec::new();

    for i in 0..vec.len() {
        if i < vec.len() - 1 && vec[i] == vec[i + 1] {
            matched.push(vec[i]);
        }

        if i == vec.len() - 1 && vec[i] == vec[0] {
            matched.push(vec[i]);
        }
    }

    matched.iter().sum()
}

fn sum2(vec: Vec<i32>) -> i32 {
    let mut matched: Vec<i32> = Vec::new();

    let mut doubled_array = vec.clone();
    let vec2 = vec.clone();

    doubled_array.extend(vec2);

    let step = vec.len() / 2;

    for i in 0..vec.len() {
        let next_index = doubled_array[i + step];

        if vec[i] == next_index {
            matched.push(vec[i]);
        }
    }

    matched.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let vec1 = vec![1, 1, 2, 2];
        assert_eq!(sum(vec1), 3);
    }

    #[test]
    fn two() {
        let vec2 = vec![1, 1, 1, 1];
        assert_eq!(sum(vec2), 4);
    }

    #[test]
    fn three() {
        let vec3 = vec![1, 2, 3, 4];
        assert_eq!(sum(vec3), 0);
    }

    #[test]
    fn four() {
        let vec4 = vec![9, 1, 2, 1, 2, 1, 2, 9];
        assert_eq!(sum(vec4), 9);
    }

    #[test]
    fn five() {
        let vec = vec![1, 2, 1, 2];
        assert_eq!(sum2(vec), 6);
    }

    #[test]
    fn six() {
        let vec = vec![1, 2, 2, 1];
        assert_eq!(sum2(vec), 0);
    }

    #[test]
    fn seven() {
        let vec = vec![1, 2, 3, 4, 2, 5];
        assert_eq!(sum2(vec), 4);
    }

    #[test]
    fn eight() {
        let vec = vec![1, 2, 3, 1, 2, 3];
        assert_eq!(sum2(vec), 12);
    }

    #[test]
    fn nine() {
        let vec = vec![1, 2, 1, 3, 1, 4, 1, 5];
        assert_eq!(sum2(vec), 4);
    }
}

