fn get_element(y: i64, x: i64) -> i64 {
    let mut count = -1i64;
    for row in 1..=y {
        for col in 0..row {
            count += 2;
            println!("{row}, {col}: {count}");
            if row == y && col == x {
                return count;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1
    // 3   5
    // 7   9   11
    // 13  15  17  19
    // 21  23  25  27  29
    // 31  33  35  37  39 41

    #[test]
    fn returns_expected1() {
        assert_eq!(get_element(1, 0), 1);
    }

    #[test]
    fn returns_expected3() {
        assert_eq!(get_element(3, 1), 9);
    }

    #[test]
    fn returns_expected35() {
        assert_eq!(get_element(6, 2), 35);
    }
}
