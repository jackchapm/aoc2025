pub(crate) trait Problem {
    fn solve(&self, input: &str) -> (String, String);
}
