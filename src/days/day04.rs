use crate::Problem;
use crate::util::*;

pub struct Day4;

impl Problem for Day4 {
    fn solve(&self, input: &str) -> (String, String) {
        let chars = input.as_bytes().to_vec();
        let len = input.find('\n').unwrap() + 1;

        let remove_rolls = |chars: &Vec<u8>| -> (i32, Vec<u8>) {
            let is_full = |idx: usize| -> Option<bool> {
                chars.get(idx).map(|x| *x == b'@')
            };

            let mut accessible = 0;
            let mut vec: Vec<usize> = Vec::new();
            for i in 0..input.len() {
                if chars[i] != b'@' { continue; }

                let cardinal_full = CARDINALS.iter().map(|(y, x)| {
                    let idx = i.add(x + y * (len as i32));
                    idx.map(|idx| is_full(idx)).flatten().unwrap_or(false)
                }).filter(|p| *p).count();

                if cardinal_full < 4 {
                    accessible += 1;
                    vec.push(i);
                }
            }

            let mut new: Vec<u8> = chars.iter().cloned().collect();
            for c in vec {
                new[c] = b'x';
            }
            (accessible, new)
        };

        let (part_one, mut working) = remove_rolls(&chars);
        let mut part_two = part_one;

        while let (val @ 1.., next_working) = remove_rolls(&working) {
            part_two += val;
            working = next_working;
        }
        

        (part_one.to_string(), part_two.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn day_four_example() {
        let input = indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "};

        assert_eq!(Day4.solve(input), ("13".to_string(), "43".to_string()))
    }
}
