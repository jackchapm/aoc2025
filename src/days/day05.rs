use crate::Problem;

pub struct Day5;


impl Problem for Day5 {
    fn solve(&self, input: &str) -> (String, String) {
        let (fresh, checks) = input.split_once("\n\n").unwrap();
        let mut fresh: Vec<(u64, u64)> = fresh.lines().map(|line| {
            let (min,max) = line.split_once("-").unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        }).collect();

        fresh.sort_by_key(|r| r.0);

        let mut packed = Vec::new();
        let mut current = fresh[0];

        for &(start, end) in &fresh[1..] {
            if start <= current.1 {
                current.1 = current.1.max(end);
            } else {
                packed.push(current.clone());
                current = (start, end);
            }
        }

        packed.push(current);

        let part_one: Vec<_> = checks.lines().map(|x| x.parse().unwrap()).filter(|x: &u64| {
            packed.iter().any(|(min, max)| x >= min && x <= max)
        }).collect();

        let part_two: u64 = packed.iter().map(|(min, max)| max - min + 1).sum();

        (part_one.len().to_string(), part_two.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn day_five_example() {
        let input = indoc! {"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};

        assert_eq!(Day5.solve(input), ("3".to_string(), "14".to_string()));
    }
}
