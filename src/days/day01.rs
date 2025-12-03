use crate::Problem;

pub struct Day1;

impl Problem for Day1 {
    fn solve(&self, input: &str) -> (String, String) {
        let mut z1 = 0;
        let mut z2 = 0;
        input.lines().map(|s| {
            let (c, i) = s.split_at(1);
            let x = i.parse::<i32>().unwrap();
            if c == "L" { -x } else { x }
        }).fold(50, |acc, x| {
            let new = (acc + x) % 100;
            let new = if new < 0 { 100 + new } else { new };

            if new == 0 { z1 += 1; }

            z2+= (acc + x).abs() / 100;
            if acc != 0 && (acc + x <= 0) { z2 += 1}
            new
        });

        (z1.to_string(), z2.to_string())
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

        assert_eq!(Day1.solve(input), ("3".to_string(), "6".to_string()))
    }
}