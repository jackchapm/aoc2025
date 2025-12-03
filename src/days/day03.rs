use crate::Problem;

pub struct Day3;

impl Problem for Day3 {
    fn solve(&self, input: &str) -> (String, String) {
        let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

        fn max_joltage<const N: usize>(lines: &Vec<&[u8]>) -> u64 {
            lines.into_iter().map(|bank| {
                let mut digits = [b'0'; N];

                for (i, &ch) in bank.iter().enumerate() {
                    for idx in 0..N {
                        if ch > digits[idx] && bank.len() - i > N - idx - 1 {
                            digits[idx] = ch;
                            for j in idx + 1..N {
                                digits[j] = b'0';
                            }
                            break;
                        }
                    }
                }

                digits.iter().fold(0u64, |acc, &d| acc * 10 + (d - b'0') as u64)
            }).sum()
        }

        (
            max_joltage::<2>(&lines).to_string(),
            max_joltage::<12>(&lines).to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn day_three_example() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};

        assert_eq!(
            Day3.solve(input),
            ("357".to_string(), "3121910778619".to_string())
        )
    }
}
