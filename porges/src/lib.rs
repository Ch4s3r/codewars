fn porges_cycle(mut num: i32) -> i32 {
    let infinite_cycle_numbers: Vec<i32> = vec![4, 16, 37, 58, 89, 145, 42, 20];
    for _ in 1..1000 {
        if infinite_cycle_numbers.contains(&num) {
            return num as i32;
        }
        num = cycle(num);
    }
    -1
}

fn cycle(num: i32) -> i32 {
    num.to_string()
        .chars()
        .filter_map(|char| char.to_digit(10))
        .map(|number| number.pow(2) as i32)
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(porges_cycle(1), -1);
        assert_eq!(porges_cycle(2), 4);
        assert_eq!(porges_cycle(4), 4);
        assert_eq!(porges_cycle(7), -1);
        assert_eq!(porges_cycle(9), 37);
        assert_eq!(porges_cycle(19), -1);
        assert_eq!(porges_cycle(73), 58);
        assert_eq!(porges_cycle(103), -1);
        assert_eq!(porges_cycle(452), 89);
        assert_eq!(porges_cycle(3564), -1);
    }
}
