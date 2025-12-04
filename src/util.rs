pub const CARDINALS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 1),
    (1, -1), (1, 0), (1, 1),
];

pub trait AddUsize {
    fn add(&self, n: i32) -> Option<usize>;
}

impl AddUsize for usize {
    fn add(&self, n: i32) -> Option<usize>  {
        if n.is_negative() {
            self.checked_sub(n.saturating_abs() as usize)
        } else {
            self.checked_add(n as usize)
        }
    }
}