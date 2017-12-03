fn main(n: f64) -> f64 {
    // https://www.reddit.com/r/adventofcode/comments/7h7ufl/2017_day_3_solutions/dqoxrb7/?st=jaqv6l1q&sh=dd7b1978
    let root = n.sqrt().ceil();

    let side_length;

    if root % 2.0 != 0.0 {
        side_length = root;
    } else {
        side_length = root + 1.0;
    }

    let steps_to_center = (side_length - 1.0) / 2.0;

    let next_square = n - ((side_length - 2.0).powi(2));

    let inner_offset = next_square % (side_length - 1.0);

    steps_to_center + (inner_offset - steps_to_center).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_12() {
        assert_eq!(main(12.0), 3.0);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(main(2.0), 1.0);
    }

    #[test]
    fn it_works_15() {
        assert_eq!(main(15.0), 2.0);
    }

    #[test]
    #[ignore]
    fn it_works_368078() {
        assert_eq!(main(368078.0), 0.0);
    }
}