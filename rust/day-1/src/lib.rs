#[allow(dead_code)]
fn day_1_part_1(input: &str) -> i32 {
    day_1_parse(input).iter().fold(0, |acc, x| acc + day_1_fuel(*x))
}

#[allow(dead_code)]
fn day_1_part_2(input: &str) -> i32 {
    day_1_parse(input).iter()
    .fold(0, |acc, x| {
        let fuel = day_1_fuel(*x);
        let fuel_for_fuel = day_1_fuel_for_fuel(fuel);
        acc + fuel + fuel_for_fuel
    })
}

fn day_1_parse(input: &str) -> Vec<i32> {
    input.lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", x))
        })
        .collect()
}

#[inline(always)]
fn day_1_fuel(mass: i32) -> i32 {
    (mass as f32 / 3.0).floor() as i32 - 2
}

fn day_1_fuel_for_fuel(mass: i32) -> i32 {
    let mut total = 0;
    let mut last_mass = mass;

    loop {
        last_mass = day_1_fuel(last_mass);
        if last_mass <= 0 { break; }
        total += last_mass;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::day_1_part_1;
    use super::day_1_part_2;

    #[test]
    fn day_1_part_1_examples() {
        assert_eq!(day_1_part_1("12"), 2);
        assert_eq!(day_1_part_1("14"), 2);
        assert_eq!(day_1_part_1("1969"), 654);
        assert_eq!(day_1_part_1("100756"), 33583);
    }

    #[test]
    fn day_1_part_1_test_input() {
        assert_eq!(day_1_part_1(include_str!("input")), 3291760);
    }

    #[test]
    fn day_1_part_2_examples() {
        assert_eq!(day_1_part_2("12"), 2);
        assert_eq!(day_1_part_2("1969"), 966);
        assert_eq!(day_1_part_2("100756"), 50346);
    }

    #[test]
    fn day_1_part_2_test_input() {
        assert_eq!(day_1_part_2(include_str!("input")), 4934767);
    }
}
