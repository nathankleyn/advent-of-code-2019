#[allow(dead_code)]
fn day_4_part_1(start: i32, end: i32) -> usize {
    check_passwords_between(start, end, true)
}

#[allow(dead_code)]
fn day_4_part_2(start: i32, end: i32) -> usize {
    check_passwords_between(start, end, false)
}

fn check_passwords_between(start: i32, end: i32, allow_larger_groups_of_adjacent_digits: bool) -> usize {
    (start..=end).filter(|i| {
        i.to_string().chars()
            .fold(PasswordMatchStatus::new(allow_larger_groups_of_adjacent_digits), |status, c| status.next(c))
            .is_valid()
    }).count()
}

#[derive(Debug)]
struct PasswordMatchStatus {
    two_adjacent_digits: bool,
    never_decreased: bool,
    previous_digit: Option<char>,
    previous_adjacent_digits: i32,
    allow_larger_groups_of_adjacent_digits: bool
}

impl PasswordMatchStatus {
    fn new(allow_larger_groups_of_adjacent_digits: bool) -> Self {
        PasswordMatchStatus {
            two_adjacent_digits: false,
            never_decreased: true,
            previous_digit: None,
            previous_adjacent_digits: 0,
            allow_larger_groups_of_adjacent_digits
        }
    }

    fn next(self, digit: char) -> Self {
        let (never_decreased, two_adjacent_digits, previous_adjacent_digits) = match self.previous_digit {
            Some(prev_digit) => {
                let never_decreased = prev_digit <= digit;

                let two_adjacent_digits = prev_digit != digit && if self.allow_larger_groups_of_adjacent_digits {
                     self.previous_adjacent_digits >= 2
                } else {
                    self.previous_adjacent_digits == 2
                };

                let previous_adjacent_digits = if prev_digit == digit {
                    self.previous_adjacent_digits + 1
                } else {
                    1
                };

                (never_decreased, two_adjacent_digits, previous_adjacent_digits)
            },
            None => (true, false, 1)
        };

        PasswordMatchStatus {
            two_adjacent_digits: self.two_adjacent_digits || two_adjacent_digits,
            never_decreased: self.never_decreased && never_decreased,
            previous_digit: Some(digit),
            previous_adjacent_digits: previous_adjacent_digits,
            allow_larger_groups_of_adjacent_digits: self.allow_larger_groups_of_adjacent_digits
        }
    }

    fn is_valid(&self) -> bool {
        let adjacent_digits = if self.allow_larger_groups_of_adjacent_digits {
            self.two_adjacent_digits || self.previous_adjacent_digits >= 2
        } else {
            self.two_adjacent_digits || self.previous_adjacent_digits == 2
        };

        self.never_decreased && adjacent_digits
    }
}

#[cfg(test)]
mod tests {
    use super::day_4_part_1;
    use super::day_4_part_2;

    #[test]
    fn day_4_part_1_examples() {
        assert_eq!(day_4_part_1(111111, 111111), 1);
        assert_eq!(day_4_part_1(223450, 223450), 0);
        assert_eq!(day_4_part_1(123789, 123789), 0);
    }

    #[test]
    fn day_4_part_1_test_input() {
        assert_eq!(day_4_part_1(172851, 675869), 1660);
    }

    #[test]
    fn day_4_part_2_examples() {
        assert_eq!(day_4_part_2(112233, 112233), 1);
        assert_eq!(day_4_part_2(123444, 123444), 0);
        assert_eq!(day_4_part_2(111122, 111122), 1);
        assert_eq!(day_4_part_2(444444, 444444), 0);
        assert_eq!(day_4_part_2(444445, 444445), 0);
    }

    #[test]
    fn day_4_part_2_test_input() {
        assert_eq!(day_4_part_2(172851, 675869), 1135);
    }
}
