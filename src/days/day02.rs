use crate::Problem;
use fancy_regex::Regex;

pub struct Day2;
impl Problem for Day2 {
    fn solve(&self, input: &str) -> (String, String) {
        let re1 = Regex::new(r#"^(\d+)\1$"#).unwrap();
        let re2 = Regex::new(r#"^(\d+)\1+$"#).unwrap();

        let (sol1, sol2): (Vec<i64>, Vec<i64>) = input.split(",").map(|range| {
            let split: Vec<i64> = range.splitn(2, "-").map(|n| n.parse::<i64>().unwrap()).collect();
            (split[0]..=split[1]).collect::<Vec<i64>>()
        }).map(|range| {
            let ids2 = range.iter().filter(|x|re2.is_match(x.to_string().as_str()).unwrap());
            let sum2: i64 = ids2.clone().sum();
            (ids2.filter(|x|re1.is_match(x.to_string().as_str()).unwrap()).sum::<i64>(), sum2)
        }).unzip();
        (sol1.into_iter().sum::<i64>().to_string(), sol2.into_iter().sum::<i64>().to_string())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_two_example() {
        let input= "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(Day2.solve(input), ("1227775554".to_string(), "4174379265".to_string()))
    }
}