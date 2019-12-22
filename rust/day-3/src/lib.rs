use std::cmp;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::i32;
use std::ops::Add;
use std::str::FromStr;

#[allow(dead_code)]
fn day_3_part_1(input: &str) -> Result<i32, String> {
    let paths: WirePaths = input.parse()?;
    let first_path = paths.first.to_visited_points();
    let second_path = paths.second.to_visited_points();

    let central_port = Point::zero();
    let crossings = first_path.intersection(&second_path);

    Ok(crossings.fold(i32::MAX, |closest_dist, point| {
        let dist = central_port.manhattan_distance_to(point);
        cmp::min(closest_dist, dist.abs())
    }))
}

#[allow(dead_code)]
fn day_3_part_2(input: &str) -> Result<i32, String> {
    let paths: WirePaths = input.parse()?;
    let first_path = paths.first.to_visited_points();
    let second_path = paths.second.to_visited_points();

    let crossings = first_path.intersection(&second_path);

    Ok(crossings.fold(i32::MAX, |least_steps, point| {
        let first = first_path.get(point).map(|p| p.steps).unwrap_or(i32::MAX);
        let second = second_path.get(point).map(|p| p.steps).unwrap_or(i32::MAX);
        cmp::min(least_steps, first + second)
    }))
}

#[derive(Debug)]
struct VisitState {
    current_pos: Point,
    visited: HashSet<Point>
}

impl VisitState {
    fn empty() -> VisitState {
        VisitState {
            current_pos: Point::zero(),
            visited: HashSet::new()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
    steps: i32
}

impl Point {
    fn zero() -> Self {
        Self { x: 0, y: 0, steps: 0 }
    }

    fn manhattan_distance_to(&self, dest: &Self) -> i32 {
        (self.x - dest.x).abs() + (self.y - dest.y).abs()
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        match dir {
            Direction::Up(amt) => Self { x: self.x, y: self.y + amt, steps: self.steps + amt },
            Direction::Down(amt) => Self { x: self.x, y: self.y - amt, steps: self.steps + amt },
            Direction::Left(amt) => Self { x: self.x - amt , y: self.y, steps: self.steps + amt },
            Direction::Right(amt) => Self { x: self.x + amt, y: self.y, steps: self.steps + amt },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

impl Direction {
    fn range_from(self, start: i32) -> Vec<Self> {
        let end = match self {
            Self::Up(amt) => amt,
            Self::Down(amt) => amt,
            Self::Left(amt) => amt,
            Self::Right(amt) => amt,
        };

        (start..end).map(|i| {
            match self {
                Self::Up(_) => Self::Up(i + 1),
                Self::Down(_) => Self::Down(i + 1),
                Self::Left(_) => Self::Left(i + 1),
                Self::Right(_) => Self::Right(i + 1),
            }
        }).collect()
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amt = s.chars().skip(1).collect::<String>().parse::<i32>()
            .map_err(|_| format!("Could not parse as Direction, chars from pos 2 onwards are not all numbers."))?;

        match s.chars().nth(0) {
            Some('U') => Ok(Direction::Up(amt)),
            Some('D') => Ok(Direction::Down(amt)),
            Some('L') => Ok(Direction::Left(amt)),
            Some('R') => Ok(Direction::Right(amt)),
            None => Err("Could not parse blank string as Direction.".to_string()), // No chars!
            Some(c) => Err(format!("Direction started with '{}' instead of U, D, L or R.", c)), // Wrong char!
        }
    }
}

struct WirePath(Vec<Direction>);

impl WirePath {
    fn to_visited_points(&self) -> HashSet<Point> {
        self.0.iter().fold(VisitState::empty(), |visit_state, dir| {
            let new_visits: Vec<Point> = dir.range_from(0)
                .iter()
                .map(|dir| visit_state.current_pos + *dir)
                .collect();

            let mut new_visited = visit_state.visited.clone();
            new_visited.extend(new_visits);

            VisitState {
                current_pos: visit_state.current_pos + *dir,
                visited: new_visited
            }
        }).visited
    }
}

struct WirePaths {
    first: WirePath,
    second: WirePath
}

impl FromStr for WirePaths {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut paths = s.lines()
            .filter(|x| !x.is_empty())
            .map(|line| {
                let directions = line.split(',')
                    .map(|x| {
                        // FIXME: It will panic here on expect.
                        x.trim().parse::<Direction>().expect(&format!("Could not parse '{}' as Direction.", x))
                    })
                    .collect();

                WirePath(directions)
            });

        let first = paths.next().ok_or("Could not parse first line of input.")?;
        let second = paths.next().ok_or("Could not parse second line of input.")?;
        Ok(WirePaths { first, second })
    }
}

#[cfg(test)]
mod tests {
    use day_3_part_1;
    use day_3_part_2;

    #[test]
    fn day_3_part_1_examples() {
        assert_eq!(day_3_part_1("R8,U5,L5,D3\nU7,R6,D4,L4"), Ok(6));
        assert_eq!(day_3_part_1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), Ok(159));
        assert_eq!(day_3_part_1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), Ok(135));
    }

    #[test]
    fn day_3_part_1_test_input() {
        assert_eq!(day_3_part_1(include_str!("input")), Ok(557));
    }

    #[test]
    fn day_3_part_2_examples() {
        assert_eq!(day_3_part_2("R8,U5,L5,D3\nU7,R6,D4,L4"), Ok(30));
        assert_eq!(day_3_part_2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), Ok(610));
        assert_eq!(day_3_part_2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), Ok(410));
    }

    #[test]
    fn day_3_part_2_test_input() {
        assert_eq!(day_3_part_2(include_str!("input")), Ok(56410));
    }
}
