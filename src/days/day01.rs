use crate::Problem;

pub struct Day1;

impl Day1 {
    fn parse(input: &str) -> impl Iterator<Item = i32> + '_ {
        input.lines().map(|s| {
            let (c, i) = s.split_at(1);
            let x = i.parse::<i32>().unwrap();
            if c == "L" { -x } else { x }
        })
    }
}

impl Problem for Day1 {
    fn part_one(&self, input: &str) -> String {
        let mut zeros = 0;
        let _ = Self::parse(input).fold(50, |acc, x| {
            let new = (acc + x) % 100;
            let new = if new < 0 { 100 + new } else { new };
            if new == 0 { zeros += 1; }
            new
        });

        zeros.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut zeros = 0;
        let _ = Self::parse(input).fold(50, |acc, x| {
            let new = (acc + x) % 100;
            let new = if new < 0 { 100 + new } else { new };
            zeros += (acc + x).abs() / 100;
            if acc != 0 && (acc + x <= 0) { zeros += 1}
            new
        });

        zeros.to_string()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn example_part_one() {
        let input = indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};

        assert_eq!(Day1.part_one(input), "3")
    }

    #[test]
    fn example_part_two() {
        let input = indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};

        assert_eq!(Day1.part_two(input), "6")
    }
}