use std::cmp::max;
use fancy_regex::Regex;
use crate::Problem;

pub struct Day6;

impl Problem for Day6 {
    fn solve(&self, input: &str) -> (String, String) {
        let mut raw_lines = input.lines();
        let re = Regex::new(r" (?=[*+])").unwrap();
        let ops: Vec<_> = re.split(raw_lines
            .next_back()
            .unwrap())
            .map(|x| x.unwrap())
            .collect();
        let mut spans: Vec<_> = ops.iter().map(|l| l.len()).collect();

        let mut rows = Vec::new();
        for line in raw_lines {
            let mut chunks = Vec::new();
            let mut chars = line.chars();

            for &span in &spans[..spans.len()-1] {
                let chunk: String = chars.by_ref().take(span).collect();
                let _ = chars.next();
                chunks.push(chunk);
            }
            chunks.push(chars.collect());
            let last = spans.len() - 1;
            spans[last] = max(spans[last], chunks.last().unwrap().len());
            rows.push(chunks);
        }

        let mut product1 = 0u64;
        let mut product2 = 0u64;
        for idx in 0..ops.len() {
            let init = match ops[idx].trim() {
                "+" => 0,
                "*" => 1,
                _ => panic!("Invalid operation found: {}", ops[idx]),
            };

            let fold_func = |acc, x| match init {
                0 => acc + x,
                1 => acc * x,
                _ => panic!("Invalid operation found: {}", ops[idx]),
            };

            product1 += rows
                .iter()
                .map(|x| x[idx].trim().parse::<u64>().unwrap())
                .fold(init, fold_func);

            let len = spans[idx];
            product2 += (0..len).map(|i| {
                rows.iter().map(|x| {
                    x[idx].as_bytes().get(i).copied().unwrap_or(b' ') as char
                }).collect::<String>().trim().parse::<u64>().unwrap()
            }).fold(init, fold_func);
        }

        (product1.to_string(), product2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn day_six_example() {
        let input = indoc! {"
            123 328  51 64
             45 64  387 23
              6 98  215 314
            *   +   *   +
        "};

        assert_eq!(
            Day6.solve(input),
            ("4277556".to_string(), "3263827".to_string())
        );
    }
}
