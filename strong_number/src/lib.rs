pub fn strong(n: u64) -> String {
    let number = n
        .to_string()
        .chars()
        .filter_map(|char| char.to_digit(10))
        .map(|number| number)
        .map(|number| number.factorial())
        .map(|number| u64::from(number))
        .sum::<u64>();
    if number == n {
        "STRONG!!!!"
    } else {
        "Not Strong !!"
    }
    .to_string()
}

trait Factorial {
    fn factorial(&self) -> u32;
}

impl Factorial for u32 {
    fn factorial(&self) -> u32 {
        match self {
            0 => 1,
            number => (number - 1).factorial() * number,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(1, "STRONG!!!!")]
    #[test_case(2, "STRONG!!!!")]
    #[test_case(145, "STRONG!!!!")]
    #[test_case(7, "Not Strong !!")]
    #[test_case(93, "Not Strong !!")]
    #[test_case(185, "Not Strong !!")]
    fn basic(input: u64, expected: &str) {
        assert_eq!(strong(input), expected)
    }

    #[test_case(0, 1)]
    #[test_case(1, 1)]
    #[test_case(2, 2)]
    #[test_case(3, 6)]
    #[test_case(4, 24)]
    #[test_case(5, 120)]
    fn factorial_test(input: u32, expected: u32) {
        assert_eq!(input.factorial(), expected)
    }
}
